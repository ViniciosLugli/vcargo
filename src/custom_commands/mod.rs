use ansi_term::Colour;
use fs_extra::dir::{copy, CopyOptions};

pub fn copy_folder(_custom_file: &str, to: &str) -> bool {
    let options = CopyOptions {
        overwrite: true,
        copy_inside: true,
        content_only: true,
        ..Default::default()
    };

    match copy(&_custom_file, to, &options) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
