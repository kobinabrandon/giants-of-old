use crate::setup::paths::{make_fundamental_directories, Directories};
use std::{fs::{self, File}, io::{Bytes, ErrorKind, Write}, path::PathBuf};

use log;
use reqwest;


pub struct ViaHTTP {
    title: String,
    url: String, 
    format: String, 
    needs_ocr: bool, 
    start_page: Option<i64>,
    end_page: Option<i64>
}


impl ViaHTTP {
    
    fn make_file_name(self) -> String {
        self.title.replace(" ", "_")
    }

    async fn download(self, file_path: &PathBuf) {
        if !file_path.exists() {
            log::info!("Downloading {}", self.title);

            let response = reqwest::get(self.url).await;

            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        let bytes = response.bytes();
                        let mut file = File::create(bytes); 
                        file.write_all(&bytes);
                    } 
                } 
                Err(e) => log::error!("Unable to download {}. Error: {}", self.title, e)
            }; 
        }


    }
    
}


pub struct ViaScraper {
    title: String, 
    url: String,
    is_interview: bool,
    initial_marker: String, 
    terminal_marker: String 
}

impl ViaScraper {

    fn make_file_name(self) -> String {
        self.title.to_string() + "txt"
    }

    fn download(self, author_name: String) {
        

    }
}


struct Author {
    name: String, 
    books_via_http: Option<Vec<ViaHTTP>>, 
    books_via_scraper: Option<Vec<ViaScraper>>,
    biographers_and_compilers: Option<Vec<String>>,
    path_to_author_data: Option<PathBuf>
}


impl Author {

    fn set_path_to_raw_data(self) -> PathBuf {
        let directories = Directories::setup();
        directories.data.join("raw")
    } 

    fn set_author_root(self) -> PathBuf {
        let directories = Directories::setup();
        directories.data.join(self.name)
    } 


    fn get_file_paths(self) -> Vec<PathBuf> {

        let path_to_raw_data = self.set_path_to_raw_data(); 

        let files: Vec<PathBuf> = fs::read_dir(&path_to_raw_data)
            .expect("Failed to read directory")
            .filter_map(
                |dir| {
                    match dir {
                        Ok(dir) => {
                            let path = dir.path();
                            if path.is_file() {
                                Some(path) // Retail if this is a file 
                            } else {
                                None 
                            }
                        }

                        Err(e) => {
                            log::error!("Warning: Could not read file dir: {}", e);
                            None // Ensures that 
                        }
                    }
                }
            ).collect(); 

        files
    }

    fn new(self) {
       
        

    }
}











// impl Default for ViaHTTP {
//     fn default() -> Self {
//         Self {
//             title: None, 
//             url: None, 
//             format: String::from("pdf"),
//             needs_ocr: false,
//             start_page: None,
//             end_page: None,
//         }
//     }
// }
//


