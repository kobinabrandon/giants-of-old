use std::path::PathBuf;
use crate::sources::authors::prepare_sources;


pub fn find_raw_data_for_author(author_name: String) -> PathBuf {

    let path = prepare_sources()
        .iter()
        .find(|author| author.name == author_name)
        .map(|author| author.set_path_to_raw_data())
        .unwrap()
        .to_path_buf();

    path
}


