use tokio;

use setup::logging::setup_logging;
use data_preparation::sourcing::download_all_texts;

mod setup {
    pub mod paths; 
    pub mod logging; 
}

mod data_preparation {
    pub mod sourcing;
    pub mod authors;
}

#[tokio::main]
async fn main() {
    setup_logging();
    download_all_texts().await
}
