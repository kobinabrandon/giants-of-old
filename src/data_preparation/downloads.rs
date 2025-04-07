// Learning Note: The structs that I have created for each source needs to implement
// the Clone trait because I will be dealing with a "level 2" aggregate type that will need 
// to implement the trait. 

use crate::setup::paths::make_fundamental_directories;
use crate::sources::authors::prepare_sources;


pub async fn download_all_texts() {
    
    make_fundamental_directories();
    for author in prepare_sources() {
        author.download_books().await;
    }

}

