use log;
use std::{fs::File, io::Write, path::PathBuf};


#[allow(dead_code)]
#[derive(Clone, Default)] 
pub struct ViaHTTP {
    pub title: String,
    pub url: String, 
    pub format: String, 
    pub needs_ocr: bool, 
    pub start_page: Option<i64>,
    pub end_page: Option<i64>
}


impl ViaHTTP {

    pub fn get_file_name(&self) -> String {
        self.title.replace(" ", "_").to_string() + &self.format
    }

    pub async fn download(self, download_path: &PathBuf) {
        if !download_path.exists() {
            log::info!("Downloading {}", self.title);
            let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(self.url).await;

            match response.as_ref().unwrap().status() {
                reqwest::StatusCode::OK => {
                    let bytes = response.unwrap().bytes();
                    let file: Result<File, std::io::Error> = File::create(download_path); 
                    _ = file.unwrap().write_all(&bytes.await.unwrap());
                } 
                _ => log::error!("Unable to download {}", self.title)
            }; 
        } 
    }
}

