use once_cell::sync::Lazy;
use std::{fs, path::{Path, PathBuf}};


pub static PARENT_DIR: Lazy<PathBuf> = Lazy::new(
    || {
        Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
    }
);

pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PARENT_DIR.join("data").to_path_buf()
}); 

pub static IMAGES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PARENT_DIR.join("images").to_path_buf()
}); 

pub static OCR_OUTPUTS: Lazy<PathBuf> = Lazy::new(|| {
    PARENT_DIR.join("OCR").to_path_buf()
}); 

pub static OCR_IMAGES: Lazy<PathBuf> = Lazy::new(|| {
    OCR_OUTPUTS.join("images").to_path_buf()
}); 

pub static PDFS_AFTER_OCR: Lazy<PathBuf> = Lazy::new(|| {
    OCR_OUTPUTS.join("pdf").to_path_buf()
}); 

pub static TXT_AFTER_OCR: Lazy<PathBuf> = Lazy::new(|| {
    OCR_OUTPUTS.join("txt").to_path_buf()
}); 

pub static CHROMA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    PARENT_DIR.join("/.chroma").to_path_buf()
}); 

// pub static ARCHIVE_DIR: Lazy<PathBuf> = Lazy::new(|| {
//     DATA_DIR.join("archive.json").to_path_buf()
// }); 

pub static IMAGES_IN_DOWNLOADS: Lazy<PathBuf> = Lazy::new(|| {
    IMAGES_IN_DOWNLOADS.join("images_in_downloads").to_path_buf()
}); 


pub fn make_fundamental_paths() {
    let paths_to_create: Vec<&Path> = vec![
        IMAGES_DIR.as_path(),
        DATA_DIR.as_path(), 
        CHROMA_DIR.as_path(),
        OCR_OUTPUTS.as_path(), 
        OCR_IMAGES.as_path(), 
        PDFS_AFTER_OCR.as_path(), 
        TXT_AFTER_OCR.as_path(),
        IMAGES_IN_DOWNLOADS.as_path(), 
    ];

    for path in paths_to_create {
        if !path.exists() {
            let _ = fs::create_dir(path);
        }
    }
}
