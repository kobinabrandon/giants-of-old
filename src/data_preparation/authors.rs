use std::path::PathBuf;

use crate::setup::paths::Directories;
use crate::data_preparation::sourcing::{ViaScraper, ViaHTTP};


pub struct Author {
    pub name: String, 
    books_via_http: Option<Vec<ViaHTTP>>, 
    books_via_scraper: Option<Vec<ViaScraper>>,
    // biographers_and_compilers: Option<Vec<String>>,
}


impl Author {

    pub fn set_path_to_raw_data(&self) -> PathBuf {
        Directories::get().data.join("raw")
    } 

    pub fn set_author_root(&self) -> PathBuf {
        Directories::get().data.join(&self.name)
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
            name: String::from("Kwame Nkrumah"),
            // biographers_and_compilers: None,
            books_via_scraper: None,
            books_via_http: Some(
                vec![
                    ViaHTTP{
                        title: String::from("Neo-Colonialism, the Last Stage of imperialism"), 
                        url: String::from("https://www.marxists.org/ebooks/nkrumah/nkrumah-neocolonialism.pdf"), 
                        start_page: Some(4), 
                        end_page: Some(202),
                        needs_ocr: false,
                        format: String::from("pdf")
                    },

                    ViaHTTP{
                        title: String::from("Dark Days in Ghana"),
                        url: String::from("https://www.marxists.org/subject/africa/nkrumah/1968/dark-days.pdf"),
                        start_page: Some(7), 
                        end_page: Some(163),
                        needs_ocr: false,
                        format: String::from("pdf")
                    }, 

                    ViaHTTP{
                        title: String::from("Africa Must Unite"),
                        url: String::from("https://www.marxists.org/subject/africa/nkrumah/1963/africa-must-unite.pdf"),
                        start_page: Some(5), 
                        end_page: Some(237),
                        needs_ocr: false,
                        format: String::from("pdf")
                    },

                    ViaHTTP{
                        title: String::from("Class Struggle In Africa"),
                        url: String::from("https://ia601208.us.archive.org/22/items/class-struggle-in-africa/Class%20Struggle%20in%20Africa_text.pdf"),
                        start_page: Some(3), 
                        end_page: Some(69),
                        needs_ocr: false,
                        format: String::from("pdf")
                    },

                    ViaHTTP{
                        title: String::from("Handbook of Revolutionary Warefare: A Guide to the Armed Phase of the African Revolution"),
                        url: String::from("http://www.itsuandi.org/itsui/downloads/Itsui_Materials/handbook-of-revolutionary-warfare-a-guide-to-the-armed-phase-of-the-african-revolution.pdf"),
                        start_page: Some(8), 
                        end_page: Some(71),
                        needs_ocr: false,
                        format: String::from("pdf")
                    },

                    ViaHTTP{
                        title: String::from("Revolutionary Path"), 
                        url: String::from("https://www.sahistory.org.za/file/426894/download?token=t2k1HcFY"),
                        start_page: Some(7),
                        end_page: Some(26),
                        needs_ocr: false,
                        format: String::from("pdf")
                    },

                    ViaHTTP{
                        title: String::from("Ghana's Policy at Home and Abroad"), 
                        url: String::from("https://www.marxists.org/subject/africa/nkrumah/1957/ghanas-policy.pdf"),
                        start_page: Some(2), 
                        end_page:Some(18),
                        needs_ocr: true,
                        format: String::from("pdf")
                    },
                ]
            ),
        }
    ];
    
    authors

}

