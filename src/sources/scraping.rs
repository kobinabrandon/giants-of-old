use log;
use scraper::{self, Html, Selector};

use std::{fs::File, io::Write, path::PathBuf};

use crate::sources::authors::prepare_sources;


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

