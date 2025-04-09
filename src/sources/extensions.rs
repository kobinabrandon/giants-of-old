use std::ffi::OsStr;
use std::path::Path;

pub const IMAGE_EXTENSIONS: [&str; 2] = [".jpg", ".png"];
pub const FILE_EXTENSIONS: [&str; 6] = [".txt", ".pdf", ".epub", ".mobi", ".azw3", ".opf"];


pub fn has_extension(target: &String, extensions: &[&str]) -> bool {
    extensions.iter()
        .any(
            |&value| target.ends_with(value)
        )
}


pub fn get_base_name(file_name_or_path: &str) -> Option<&OsStr> {
    let name = Path::new(file_name_or_path).file_name();
    name
}

