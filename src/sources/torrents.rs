use glob;
use glob::GlobError;
use core::panic;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use librqbit::Session;
use librqbit::AddTorrent;

use crate::setup::paths::Directories;


fn has_extension(target: &str, extensions: &[&str]) -> bool {
    extensions.iter()
        .any(
            |&value| target.ends_with(value)
        )
}


fn get_base_name(target: &str) -> &str {
    let path = Path::new(target);
    path.file_name().unwrap().to_str().unwrap()
}

fn list_path_contents(path: &PathBuf) -> Result<Vec<Result<PathBuf, GlobError>>, anyhow::Error> {
    let pattern: String = format!("{}/**/*", path.to_str().unwrap());
    
    // The glob returns a iterator of Path objects, and we map each path to an owned string, or fail
    // to do so, and have an error.
    let contents: Vec<Result<PathBuf, glob::GlobError>> = glob::glob(&pattern)?
        .map(
            |entry| match entry {
                Ok(path) => Ok(path),
                Err(err) => Err(err) 
            }
        ).collect();

    Ok(contents)
}




#[derive(Clone)]
#[allow(dead_code)]
pub struct ViaTorrent {
    pub magnet: String 
}

impl ViaTorrent {
    
    pub async fn download(&self, download_path: PathBuf) { 

        let session = Session::new(download_path.into()).await.unwrap();

        let torrent_handle = session.add_torrent(
            AddTorrent::from_url(&self.magnet),
            None
        ).await.unwrap().into_handle().unwrap();

        torrent_handle.wait_until_completed().await.unwrap()
    }

    fn extract_files(&self, download_path: PathBuf, author_name: String) {

        let path_contents: Vec<Result<PathBuf, GlobError>> = list_path_contents(&download_path).expect("Could not get contents of download path"); 
        let mut file_paths: Vec<PathBuf> = Vec::new();
        let mut directories: Vec<PathBuf> = Vec::new();

        let text_extensions: [&str; 5] = ["txt", "pdf", "epub", "mobi", "azw3"];
        let image_extensions: [&str; 2] = ["jpg", "png"];


        let author_image_dir = Directories::setup().images_in_downloads.join(author_name);

        for file in kdam::tqdm!(
            path_contents.iter(),
        ) {

            match file {
                Ok(real_path) => {
                    let path: &Path = real_path.as_path();

                    // Changing the &Path to a PathBuf avoid having dangling references in the vector
                    if path.is_file() {
                        file_paths.push(path.to_path_buf()); 
                    } else {
                        directories.push(path.to_path_buf()); 
                    }                 
                }
                
                Err(e) => {
                    log::error!("Warning: Could not read directory: {}", e);
                }
            }
        }


        let mut paths_of_downloaded_files: Vec<String> = Vec::new(); 
        let mut paths_of_downloaded_images: Vec<String> = Vec::new(); 

        for file in kdam::tqdm!(file_paths.iter()) {
            let file_name_as_string: &str = file.to_str().expect("File path could not be rendered as a string slice");
            let file_is_text: bool = has_extension(&file_name_as_string.to_lowercase(), &text_extensions); 
            let file_is_image: bool = has_extension(&file_name_as_string.to_lowercase(), &image_extensions); 
            let file_base_name: &str = get_base_name(file_name_as_string);

            if file_is_text || file_is_image {

                if !download_path.join(file_base_name).exists() {
                    fs::rename(file_name_as_string, &download_path).unwrap();

                    paths_of_downloaded_files.push(
                        download_path.join(file_base_name).to_str().unwrap().to_string()
                    );

                } else if !author_image_dir.join(file_base_name).exists() {
                    fs::rename(file_name_as_string, &download_path).unwrap();

                    paths_of_downloaded_images.push(
                        author_image_dir.join(file_base_name).to_str().unwrap().to_string()
                    );

                } else {
                    panic!(
                        "The file at {} is neither has none of the expected extensions", file.display()
                    )
                }
            }
        }
    }
}

