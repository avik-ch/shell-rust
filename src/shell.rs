use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    process::{Child, Command as ProcessCommand, Stdio},
    thread::{self, JoinHandle},
};

use os_pipe::{PipeReader, PipeWriter};

use crate::builtins::Builtin;
use crate::command::{Command, Redirect};
use crate::helpers::find_executable;
use crate::parser::tokenise;

pub struct Shell;

enum OutputDestination {
    Inherit,
    Pipe(PipeWriter),
    File(File),
}

#[derive(Clone, Copy)]
enum OutputStream {
    Stdout,
    Stderr,
}

impl Shell {
    pub fn run() {
        let mut input = String::new();
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            input.clear();
            let bytes_read = io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if bytes_read == 0 {
                break;
            }

            if input.trim().is_empty() {
                continue;
            }

            let command = match tokenise(&input) {
                Ok(command) => command,
                Err(error) => {
                    eprintln!("{error}");
                    continue;
                }
            };

            if let Err(error) = Self::execute(command) {
                eprintln!("{error}");
            }
        }
    }

    fn execute(command: Command) -> io::Result<()> {
        let mut commands = command.into_simple_commands();
        let command_count = commands.len();
        let mut inputs: Vec<Option<PipeReader>> = (0..command_count).map(|_| None).collect();
        let mut outputs: Vec<Option<PipeWriter>> = (0..command_count).map(|_| None).collect();

        for index in 0..command_count.saturating_sub(1) {
            let (reader, writer) = os_pipe::pipe()?;
            outputs[index] = Some(writer);
            inputs[index + 1] = Some(reader);
        }

        let mut children: Vec<Child> = Vec::new();
        let mut writers: Vec<JoinHandle<()>> = Vec::new();

        for (index, mut command) in commands.drain(..).enumerate() {
            let executable = command.args.remove(0);
            let args = command.args;
            let stdout = match Self::destination(command.std_out, outputs[index].take()) {
                Ok(stdout) => stdout,
                Err(error) => {
                    drop(inputs[index].take());
                    eprintln!("{executable}: {error}");
                    continue;
                }
            };
            let stderr = match Self::destination(command.std_err, None) {
                Ok(stderr) => stderr,
                Err(error) => {
                    drop(inputs[index].take());
                    drop(stdout);
                    eprintln!("{executable}: {error}");
                    continue;
                }
            };

            if let Some(builtin) = Builtin::lookup(&executable) {
                // Current built-ins do not consume stdin, so closing it is the correct behavior.
                drop(inputs[index].take());

                let mut stdout_bytes = Vec::new();
                let mut stderr_bytes = Vec::new();
                builtin.execute(&args, &mut stdout_bytes, &mut stderr_bytes);
                Self::write_in_background(stdout, OutputStream::Stdout, stdout_bytes, &mut writers);
                Self::write_in_background(stderr, OutputStream::Stderr, stderr_bytes, &mut writers);
                continue;
            }

            let Some(path) = find_executable(&executable) else {
                drop(inputs[index].take());
                drop(stdout);
                Self::write_in_background(
                    stderr,
                    OutputStream::Stderr,
                    format!("{executable}: command not found\n").into_bytes(),
                    &mut writers,
                );
                continue;
            };

            let stdin = inputs[index]
                .take()
                .map(Stdio::from)
                .unwrap_or_else(Stdio::inherit);
            let mut process = ProcessCommand::new(path);
            process
                .args(args)
                .stdin(stdin)
                .stdout(Self::stdio(stdout))
                .stderr(Self::stdio(stderr));

            match process.spawn() {
                Ok(child) => children.push(child),
                Err(error) => eprintln!("{executable}: {error}"),
            }
        }

        // All parent pipe ends have been moved or dropped, so readers can observe EOF.
        drop(inputs);
        drop(outputs);

        for mut child in children {
            let _ = child.wait();
        }
        for writer in writers {
            let _ = writer.join();
        }

        Ok(())
    }

    fn destination(
        redirect: Option<Redirect>,
        pipe: Option<PipeWriter>,
    ) -> io::Result<OutputDestination> {
        if let Some(redirect) = redirect {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(!redirect.append)
                .append(redirect.append)
                .open(redirect.path)?;
            return Ok(OutputDestination::File(file));
        }

        Ok(match pipe {
            Some(pipe) => OutputDestination::Pipe(pipe),
            None => OutputDestination::Inherit,
        })
    }

    fn stdio(destination: OutputDestination) -> Stdio {
        match destination {
            OutputDestination::Inherit => Stdio::inherit(),
            OutputDestination::Pipe(pipe) => Stdio::from(pipe),
            OutputDestination::File(file) => Stdio::from(file),
        }
    }

    fn write_in_background(
        destination: OutputDestination,
        stream: OutputStream,
        bytes: Vec<u8>,
        writers: &mut Vec<JoinHandle<()>>,
    ) {
        if bytes.is_empty() {
            return;
        }

        writers.push(thread::spawn(move || match destination {
            OutputDestination::Inherit => match stream {
                OutputStream::Stdout => {
                    let _ = io::stdout().write_all(&bytes);
                }
                OutputStream::Stderr => {
                    let _ = io::stderr().write_all(&bytes);
                }
            },
            OutputDestination::Pipe(mut pipe) => {
                let _ = pipe.write_all(&bytes);
            }
            OutputDestination::File(mut file) => {
                let _ = file.write_all(&bytes);
            }
        }));
    }
}
