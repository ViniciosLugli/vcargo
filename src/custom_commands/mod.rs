use fs_extra::dir::{copy, CopyOptions};
use std::env;

pub fn copy_folder(to: &str) -> bool {
    let _default_path: String = env::var("_DEFAULT_PATH_")
        .unwrap_or("C:/Users/vinic/Documents/codigos/rust/_DEFAULT_".to_string());

    let options = CopyOptions::new();

    match copy(_default_path, to, &options) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
