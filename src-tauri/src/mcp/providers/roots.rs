use std::path::{Path, PathBuf};
use std::env;
use log::info;

pub fn get_root_paths() -> Vec<PathBuf> {
    let home = env::var("HOME").unwrap_or_else(|_| "/home/atlantispleb".to_string());
    let paths = vec![
        format!("{}/code/pylon", home),
        format!("{}/code/onyx", home),
    ];
    
    let mut valid_paths = Vec::new();
    for path in paths {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() && path_buf.is_dir() {
            info!("Found valid root path: {:?}", path_buf);
            valid_paths.push(path_buf);
        } else {
            info!("Path does not exist or is not a directory: {:?}", path_buf);
        }
    }
    
    valid_paths
}