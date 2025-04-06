use std::fs;
use std::path::PathBuf;

use crate::setup::paths::Directories;
use crate::data_preparation::sourcing::{ViaScraper, ViaHTTP};


#[derive(Default)]
#[allow(dead_code)]
pub struct Author {
    pub name: String, 
    pub books_via_http: Option<Vec<ViaHTTP>>, 
    pub books_via_scraper: Option<Vec<ViaScraper>>,
    pub biographers_and_compilers: Option<Vec<String>>,
}


impl Author {

    pub fn set_path_to_raw_data(&self) -> PathBuf {
        let author_root = Directories::get().data.join(&self.name);
        _ = fs::create_dir(&author_root);
        author_root.join("raw")
    } 

    // fn get_file_paths(self) -> Vec<PathBuf> {
    //
    //     let path_to_raw_data = self.set_path_to_raw_data(); 
    //
    //     let files: Vec<PathBuf> = fs::read_dir(&path_to_raw_data)
    //         .expect("Failed to read directory")
    //         .filter_map(
    //             |dir| {
    //                 match dir {
    //                     Ok(dir) => {
    //                         let path = dir.path();
    //                         if path.is_file() {
    //                             Some(path) // Retail if this is a file 
    //                         } else {
    //                             None 
    //                         }
    //                     }
    //
    //                     Err(e) => {
    //                         log::error!("Warning: Could not read file dir: {}", e);
    //                         None
    //                     }
    //                 }
    //             }
    //         )
    //         .collect(); 
    //
    //     files
    // }

    async fn download_via_http(&self) {
        let http_books: Vec<ViaHTTP> = self.books_via_http.clone().unwrap();

        for book in http_books {
            let file_name: String = book.get_file_name();
            let path_to_raw_data: &PathBuf = &self.set_path_to_raw_data();
            let file_path = path_to_raw_data.join(file_name);
            _ = fs::create_dir(&path_to_raw_data);
            book.clone().download(&file_path).await;
        }    
    }

    async fn download_via_scraper(&self) {
        let books_to_scrape: &Vec<ViaScraper> = &self.books_via_scraper.clone().unwrap();

        for book in books_to_scrape {
            book.clone().download(&self.name).await;
        }    
    }


    pub async fn download_books(&self) {
        let http_books: Option<Vec<ViaHTTP>> = self.books_via_http.clone();
        let books_to_scrape: Option<Vec<ViaScraper>> = self.books_via_scraper.clone();

        match (http_books, books_to_scrape) {
            (Some(_http_books), Some(_books_to_scrape)) => {
                self.download_via_http().await;
                self.download_via_scraper().await;
            }

            (Some(_http_books), None) => {
                self.download_via_http().await;
            }

            (None, Some(_books_to_scrape)) => {
                self.download_via_scraper().await;
            }
            (None, None) => {
                log::error!("{} has no books that can be acquired by HTTP request or scraping", self.name)
            }

        } 
    }
}


pub fn prepare_sources() -> Vec<Author> {

    let authors = vec![

        Author{
            name: String::from("Karl Marx"),
            books_via_http: Some(
                vec![
                    ViaHTTP{
                        title: String::from("Capital Vol I"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/Capital-Volume-I.pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Capital Vol II"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/Capital-Volume-II.pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Capital Vol III"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/Capital-Volume-III.pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Value, Price & Profit"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/value-price-profit.pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Wage, Labour & Capital"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/wage-labour-capital.pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("The Communist Manifesto"),
                        url: String::from("https://www.marxists.org/archive/marx/works/download/pdf/Manifesto.pdf"),
                        start_page: Some(13),
                        end_page: Some(66),
                        ..ViaHTTP::default()
                    },
                ]
            ),
            ..Author::default()
        },

        Author{
            name: String::from("Mao Zedong"),
            books_via_scraper: Some(
                vec![
                    ViaScraper{
                        title: String::from("Combat Liberalism"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/volume-2/mswv2_03.htm"),
                        initial_marker: Some(String::from("We stand for")),
                        terminal_marker: Some(String::from("Transcription")),
                        ..ViaScraper::default()
                    }
                ],
            ),
            books_via_http: Some(
                vec![
                    ViaHTTP{
                        title: String::from("Oppose Book Worship"),
                        url: String::from("https://www.marxists.org/ebooks/mao/Oppose_Book_Worship_-_Mao_Zedong.pdf"),
                        start_page: Some(2),
                        end_page: Some(12),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Selected Works of Mao Tse-Tung Volume I"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/sw-in-pdf/sw-flp-1965-v1.pdf"),
                        format: String::from(".pdf"),
                        start_page: Some(20),
                        end_page: Some(353),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Selected Works of Mao Tse-Tung Volume II"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/sw-in-pdf/sw-flp-1965-v2.pdf"),
                        start_page: Some(18),
                        end_page: Some(473),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Selected Works of Mao Tse-Tung Volume III"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/sw-in-pdf/sw-flp-1965-v3.pdf"),
                        start_page: Some(16),
                        end_page: Some(345),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Selected Works of Mao Tse-Tung Volume IV"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/sw-in-pdf/sw-flp-1965-v4.pdf"),
                        start_page: Some(17),
                        end_page: Some(463),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()
                    },

                    ViaHTTP{
                        title: String::from("Selected Works of Mao Tse-Tung Volume V"),
                        url: String::from("https://www.marxists.org/reference/archive/mao/selected-works/sw-in-pdf/sw-flp-1971-v5.pdf"),
                        start_page: Some(22),
                        end_page: Some(524),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()
                    }
                ]
            ),
            ..Author::default()
        },

        Author{
            name: String::from("Marcus Garvey"),
            books_via_http: Some(
                vec![
                    ViaHTTP{
                        title: String::from("The Philosophy & Opinions of Marcus Garvey"),
                        url: String::from("https://www.jpanafrican.org/ebooks/eBook%20Phil%20and%20Opinions.pdf"),
                        start_page: Some(3),
                        end_page: Some(62),
                        format: String::from(".pdf"),
                        ..ViaHTTP::default()

                    },
                ]
            ),
            ..Author::default()
        } 
    ]; 
    authors
}

