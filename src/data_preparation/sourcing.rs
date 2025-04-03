use log;
use anyhow;
use reqwest;
use scraper::{self, Html, Selector};
use std::{fs::File, io::Write, path::PathBuf};

use crate::setup::paths::Directories;
use crate::data_preparation::authors::prepare_sources;


pub struct ViaHTTP {
    pub title: String,
    pub url: String, 
    pub format: String, 
    pub needs_ocr: bool, 
    pub start_page: Option<i64>,
    pub end_page: Option<i64>
}


impl ViaHTTP {

    fn set_file_name(self) -> String {
        self.title.replace(" ", "_")
    }

    async fn download(self, file_path: &PathBuf) {
        if !file_path.exists() {
            log::info!("Downloading {}", self.title);
            let response = reqwest::get(self.url).await;

            match response {
                Ok(response) => {
                    // Runs if there is a response
                    if response.status().is_success() {
                        let bytes = response.bytes();
                        let file: Result<File, std::io::Error> = File::create(file_path); 
                        _ = file.unwrap().write_all(&bytes.await.unwrap());
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

    fn make_file_name(&self) -> String {
        self.title.to_string() + ".txt"
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

        let directories = Directories::setup();
        let path = prepare_sources()
            .iter()
            .find(|author| author.name == author_name)
            .map(|author| author.set_author_root(&directories))
            .unwrap()
            .to_path_buf();

        path
    }

    async fn download(&self, author_name: String) {
        let file_name = self.make_file_name().to_string();
        let destination_path = self.find_raw_data_for_author(author_name); 
        let file_path: PathBuf = destination_path.join(&file_name);

        if !file_path.exists() {
            log::warn!("Attempting to scrape {}", self.title);
            let scraped_text = self.scrape().await;
            let file: Result<File, std::io::Error> = File::create(file_path);
            _ = file.unwrap().write_all(scraped_text.as_bytes());
        } else {
            log::warn!("{} already exists at {}", file_name, file_path.display());
        }
    
        
    }
}





