use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub fn find_executable(executable: &str) -> Option<PathBuf> {
    let Ok(paths) = env::var("PATH") else {
        return None;
    };

    for dir in env::split_paths(&paths) {
        let full_path = Path::new(&dir).join(executable);

        if full_path.exists() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 != 0 {
                    return Some(full_path);
                }
            }
        }
    }

    None
}
