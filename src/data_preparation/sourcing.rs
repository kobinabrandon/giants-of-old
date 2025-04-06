// Learning Note: The structs that I have created for each source needs to implement
// the Clone trait because I will be dealing with a "level 2" aggregate type that will need 
// to implement the trait. 

use log;
use scraper::{self, Html, Selector};
use std::{fs::File, io::Write, path::PathBuf};

use crate::setup::paths::make_fundamental_directories;
use crate::data_preparation::authors::prepare_sources;



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

    pub async fn download(self, file_path: &PathBuf) {
        if !file_path.exists() {
            log::info!("Downloading {}", self.title);
            let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(self.url).await;

            match response.as_ref().unwrap().status() {
                reqwest::StatusCode::OK => {
                    let bytes = response.unwrap().bytes();
                    let file: Result<File, std::io::Error> = File::create(file_path); 
                    _ = file.unwrap().write_all(&bytes.await.unwrap());
                    log::info!("Downloaded")
                } 
                _ => log::error!("Unable to download {}", self.title)
            }; 
        } 
    }
}



#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct ViaScraper {
    pub title: String, 
    pub url: String,
    pub format: String,
    pub is_interview: bool,
    pub initial_marker: Option<String>, 
    pub terminal_marker: Option<String> 
}


impl ViaScraper {

    pub fn get_file_name(&self) -> String {
        self.title.replace(" ", "_").to_string() + &self.format
    }

    async fn make_request(&self) -> Result<String, anyhow:: Error> { 
        let html = reqwest::get(&self.url).await?.text().await?; 
        Ok(html)
    }

    async fn scrape(&self) -> String { 
        let html = self.make_request();
        let mut scraped_text: String = String::new();
        let document = Html::parse_document(&html.await.unwrap());
        let paragraph_selector: &Selector = &scraper::Selector::parse("p").unwrap();

        for element in document.select(paragraph_selector) {
            let paragraph_text: String = element.text().collect();
            scraped_text.push_str(&paragraph_text);
            scraped_text.push_str("\n");
        }
        scraped_text
    }

    fn find_raw_data_for_author(&self, author_name: String) -> PathBuf {

        let path = prepare_sources()
            .iter()
            .find(|author| author.name == author_name)
            .map(|author| author.set_path_to_raw_data())
            .unwrap()
            .to_path_buf();

        path
    }

    pub async fn download(&self, author_name: &String) {
        let file_name = self.get_file_name().to_string();
        let destination_path = self.find_raw_data_for_author(author_name.to_string()); 
        let file_path: PathBuf = destination_path.join(&file_name);

        if !file_path.exists() {
            log::warn!("Attempting to scrape {}", self.title);
            let scraped_text = self.scrape().await;
            let file: Result<File, std::io::Error> = File::create(file_path);
            _ = file.unwrap().write_all(scraped_text.as_bytes());
        } 
    }
}


pub async fn download_all_texts() {
    
    make_fundamental_directories();
    for author in prepare_sources() {
        author.download_books().await;
    }

}

