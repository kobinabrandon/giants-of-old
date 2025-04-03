use std::fs;
use std::path::PathBuf;

use crate::setup::paths::Directories;
use crate::data_preparation::sourcing::{ViaScraper, ViaHTTP};


pub struct Author {
    pub name: String, 
    books_via_http: Option<Vec<ViaHTTP>>, 
    books_via_scraper: Option<Vec<ViaScraper>>,
    biographers_and_compilers: Option<Vec<String>>,
}


impl Author {

    fn set_path_to_raw_data(&self, directories: &Directories) -> PathBuf {
        directories.data.join("raw")
    } 

    pub fn set_author_root(&self, directories: &Directories) -> PathBuf {
        directories.data.join(&self.name)
    } 

    fn get_file_paths(self) -> Vec<PathBuf> {

        let directories = Directories::setup();
        let path_to_raw_data = self.set_path_to_raw_data(&directories); 

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
                            None
                        }
                    }
                }
            )
            .collect(); 

        files
    }

    
}


pub fn prepare_sources() -> Vec<Author> {

    let authors = vec![
        Author{
            name: String::from("Kwame Nkrumah"),
            biographers_and_compilers: None,
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

