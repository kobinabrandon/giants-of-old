use log;
use scraper::{self, Html, Selector};
use std::{fs::File, io::Write, path::PathBuf, usize};

use crate::sources::authors;


#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct ViaScraper <'a> {
    pub title: String, 
    pub url: String,
    pub format: String,
    pub is_interview: bool,
    pub initial_marker: Option<&'a str>, 
    pub terminal_marker: Option<&'a str> 
}


impl ViaScraper <'_> {

    pub async fn download(&self, author_name: &String) {
        log::info!("Downloading {}", self.title);
        let file_name = self.get_file_name().to_string();
        let author_root = authors::get_author_root(&author_name);
        let file_path: PathBuf = author_root.join(&file_name);

        if !file_path.exists() {
            log::warn!("Attempting to scrape {}", self.title);
            let scraped_text: String = self.scrape().await;

            if self.needs_truncation() {
                let truncated_text: &str = self.truncate(&scraped_text).unwrap(); 
                self.save_file(truncated_text, &file_path)
            } else {
                self.save_file(&scraped_text, &file_path)
            }

        } 
    }

    fn save_file(&self, text: &str, file_path: &PathBuf) {
        let file: Result<File, std::io::Error> = File::create(file_path);
        _ = file.unwrap().write_all(text.as_bytes());
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

    fn truncate<'a, 'b, 'out>(&'a self, full_text: &'b String) -> Option<&'out str> 
        where 'a: 'out, 'b: 'out 
    {
        let start_index: usize = full_text.rfind(self.initial_marker.unwrap()).unwrap();
        let terminal_index: usize = full_text.rfind(self.terminal_marker.unwrap()).unwrap();

        Some(&full_text[start_index..terminal_index])
    }

    fn needs_truncation(&self) -> bool {

        match (&self.initial_marker, &self.terminal_marker) {

            (Some(_), None) | (None, Some(_)) => {
                log::error!("Partial scraping of {} has been requested without one of the markers", self.title);
                false
            }

            (Some(_), Some(_)) => {true},
            (None, None) => {false},
        }
    }

    pub fn get_file_name(&self) -> String {
        self.title.replace(" ", "_").to_string() + &self.format
    }

    async fn make_request(&self) -> Result<String, anyhow:: Error> { 
        let html = reqwest::get(&self.url).await?.text().await?; 
        Ok(html)
    }
}

