use data_preparation::authors::{self, prepare_sources};
use setup::paths::make_fundamental_directories;
use tokio;

mod setup {
    pub mod paths; 
}

mod data_preparation {
    pub mod sourcing;
    pub mod authors;
}

#[tokio::main]
async fn main() {
    env_logger::init(); 
    make_fundamental_directories();
    let all_authors = prepare_sources();
    authors::Author::download_books(&all_authors[0]).await;
}
