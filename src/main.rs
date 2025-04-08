use tokio;

use setup::logging::setup_logging;
use data_preparation::downloads::download_all_texts;

mod setup {
    pub mod paths; 
    pub mod logging; 
}

mod sources {
    pub mod http;
    pub mod scraping;
    pub mod torrents;
    pub mod authors;
    pub mod utils;
}

mod data_preparation{
    pub mod downloads;
}


#[tokio::main]
async fn main() {
    setup_logging();
    download_all_texts().await
}
