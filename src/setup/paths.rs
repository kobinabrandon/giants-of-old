use log;
use struct_iterable::Iterable; 
use std::{fs, io::ErrorKind, path::PathBuf};


#[derive(Iterable)]
pub struct Directories {
    pub data: PathBuf,
    pub images: PathBuf,
    pub models: PathBuf,
    pub ocr_outputs: PathBuf,
    pub pdfs_after_ocr: PathBuf,
    pub txt_after_ocr: PathBuf,
    pub chroma: PathBuf,
    pub images_in_downloads: PathBuf
}


impl Directories {

    pub fn get() -> Directories {
        let directories = Directories::setup();
        directories
    }

    pub fn setup() -> Self {
        let parent = std::env::current_dir().expect("Failed to provide current directory");
        let data: PathBuf = parent.join("data");
        let images: PathBuf = parent.join("images");
        let models: PathBuf = parent.join("models");
        let chroma: PathBuf = parent.join("chroma").to_path_buf();
        let ocr_outputs: PathBuf = parent.join("OCR").to_path_buf();
        let pdfs_after_ocr: PathBuf = ocr_outputs.join("pdf").to_path_buf();
        let txt_after_ocr: PathBuf = ocr_outputs.join("pdf").to_path_buf();
        let images_in_downloads: PathBuf = images.join("images_in_downloads").to_path_buf();
       
        Self {
            models, 
            data, 
            images, 
            chroma,
            ocr_outputs,
            images_in_downloads,
            txt_after_ocr,
            pdfs_after_ocr
        }
    }
}



pub fn make_fundamental_directories() {
   
    let mut directories_to_make: Vec<PathBuf> = Vec::new();

    // Make sure we downcast the second 
    for field_tuple in Directories::get().iter() {
        if let Some(dir) = field_tuple.1.downcast_ref::<PathBuf>() { 
            directories_to_make.push(dir.to_path_buf());
        }

    } 

    for dir in directories_to_make {
        match fs::create_dir(&dir) {
            Ok(_) => log::info!("Created {} directory: ", dir.to_str().unwrap()),
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    continue
                } else {
                    log::error!("Could not create directory {}", dir.display());
                }
            }
        }
    }
}

