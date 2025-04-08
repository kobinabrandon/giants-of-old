use std::fs;
use std::path::PathBuf;

use crate::sources::http::ViaHTTP;
use crate::setup::paths::Directories;
use crate::sources::scraping::ViaScraper;
use crate::sources::torrents::ViaTorrent;


#[derive(Default)]
#[allow(dead_code)]
pub struct Author {
    pub name: String, 
    pub books_via_http: Option<Vec<ViaHTTP>>, 
    pub books_via_scraper: Option<Vec<ViaScraper>>,
    pub books_via_torrent: Option<Vec<ViaTorrent>>,
    pub biographers_and_compilers: Option<Vec<String>>,
}


impl Author {

    pub fn set_path_to_raw_data(&self) -> PathBuf {
        let author_data_root = Directories::get().data.join(&self.name);
        _ = fs::create_dir(&author_data_root);
        author_data_root.join("raw")
    } 

    pub fn get_file_paths(self) -> Vec<PathBuf> {

        let path_to_raw_data = self.set_path_to_raw_data(); 

        let files: Vec<PathBuf> = fs::read_dir(&path_to_raw_data)
            .expect("Failed to read directory")
            .filter_map(
                |dir| {
                    match dir {
                        Ok(dir) => {
                            let path = dir.path();
                            if path.is_file() {
                                Some(path) // Return if this is a file 
                            } else {
                                None // I'm not willing to assume that the directory will only ever contain files
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

    async fn download_via_torrent(&self) {
        let books_to_torrent: &Vec<ViaTorrent> = &self.books_via_torrent.clone().unwrap();
        for book in books_to_torrent {
            let download_path = self.set_path_to_raw_data();
            book.download(download_path.clone()).await;
            book.extract_files(download_path, &self.name);

        }    
    }

    pub async fn download_books(&self) {
        let http_books: Option<Vec<ViaHTTP>> = self.books_via_http.clone();
        let books_to_scrape: Option<Vec<ViaScraper>> = self.books_via_scraper.clone();
        let books_to_torrent: Option<Vec<ViaTorrent>> = self.books_via_torrent.clone();

        log::info!("Downloading {}'s texts", &self.name);
        match (http_books, books_to_scrape, books_to_torrent) {

            (None, None, None) => {
                log::error!("{} has no books that can be acquired by HTTP, scraping, or torrenting", &self.name)
            },

            (Some(_http_books), None, None) => {
                self.download_via_http().await;
            },

            (None, Some(_books_to_scrape), None) => {
                self.download_via_scraper().await;
            },

            (None, None, Some(_books_to_torrent)) => {
                self.download_via_torrent().await;
            },

            (Some(_http_books), Some(_books_to_scrape), None) => {
                self.download_via_http().await;
                self.download_via_scraper().await;
            },

            (None, Some(_books_to_scrape), Some(_books_to_torrent)) => {
                self.download_via_scraper().await;
                self.download_via_torrent().await;
            },

            (Some(_http_books), None, Some(_books_to_torrent)) => {
                self.download_via_http().await;
                self.download_via_scraper().await;
            },

            (Some(_http_books), Some(_books_to_scrape), Some(_books_to_torrent)) => {
                self.download_via_http().await;
                self.download_via_scraper().await;
                self.download_via_torrent().await;
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
        },

        Author{
            name: String::from("Swami Vivekananda"),
            biographers_and_compilers: None, 
            books_via_http: Some(
                vec![
                    ViaHTTP{
                        title: String::from("The Complete Works of Swami Vivekananda"),
                        url: String::from("https://ia801608.us.archive.org/9/items/complete-works-of-swami-vivekananda-all-volumes-swami-vivekananda/Complete%20Works%20of%20Swami%20Vivekananda%20-%20%20All%20Volumes%20-%20Swami%20Vivekananda.pdf"),
                        start_page: Some(81),
                        end_page: Some(5162),
                        format: String::from("pdf"),
                        ..ViaHTTP::default()
                    }
                ]
            ),
            ..Author::default()
        },

        Author{
            name: String::from("Helena Pretrovna Blavatsky"),
            biographers_and_compilers: Some(
                vec!["Marion Meade".to_string(), "Gary Lachman".to_string()]
            ),
            books_via_torrent: Some(
                vec![
                    ViaTorrent{
                        magnet: String::from("magnet:?xt=urn:btih:7933F8B90EAC4CBCCEED1667B5E5FF0C7E5F9B29&dn=H.%20P.%20Blavatsky%20-%20Collected%20Writings%20and%20More%20%5Bepub%20mobi%20pdf%5D&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Ftracker.bittor.pw%3A1337%2Fannounce&tr=udp%3A%2F%2Fpublic.popcorn-tracker.org%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.dler.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969&tr=udp%3A%2F%2Fopen.demonii.com%3A1337%2Fannounce")
                    }
                ]
            ), 
            ..Author::default()
        },


    ]; 

    authors
}

