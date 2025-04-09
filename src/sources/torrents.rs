use std::ffi::OsStr;
use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;

use serde_json;
use glob::GlobError;

use librqbit::Session;
use librqbit::AddTorrent;
use librqbit::AddTorrentOptions;

use crate::sources::utils;
use crate::sources::extensions;


#[derive(Clone)]
#[allow(dead_code)]
pub struct ViaTorrent {
    pub magnet: String 
}

impl ViaTorrent {
    
    pub async fn download(&self, download_path: PathBuf) { 

        let session = Session::new(download_path.into()).await.unwrap();

        let torrent_config = AddTorrentOptions{
            overwrite: true, // Because I would like overwrites to be possible
            ..AddTorrentOptions::default()
        };

        let torrent_handle = session.add_torrent(
            AddTorrent::from_url(&self.magnet),
            Some(torrent_config)
        ).await.unwrap().into_handle().unwrap();

        torrent_handle.wait_until_completed().await.unwrap()
    }

    pub fn extract_files(&self, download_path: PathBuf, author_name: &String) {

        let mut file_paths: Vec<PathBuf> = Vec::new();
        let mut directories: Vec<PathBuf> = Vec::new();
        let author_root = utils::get_author_root(&author_name);

        let path_contents: Vec<Result<PathBuf, GlobError>> = list_path_contents(&download_path).expect("Could not get contents of download path"); 

        for file in kdam::tqdm!(
            path_contents.iter(),
            desc="Identifying files and directories..."
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

        move_files_to_destinations(
            &author_root,
            &file_paths, 
            &mut paths_of_downloaded_files,
        );

        log_downloaded_files(author_name, &paths_of_downloaded_files);
        remove_book_directories(directories);
    }
}


fn move_files_to_destinations(
    author_root: &PathBuf,
    file_paths: &Vec<PathBuf>, 
    paths_of_downloaded_files: &mut Vec<String>,
) {
    
    for file in kdam::tqdm!(file_paths.iter(), desc="Extracting text and images...") {

        let file_path_as_string: &str = file.to_str().expect("File path could not be rendered as a string slice");
        let file_is_text: bool = extensions::has_extension(&file_path_as_string.to_lowercase(), &extensions::FILE_EXTENSIONS); 
        let file_is_image: bool = extensions::has_extension(&file_path_as_string.to_lowercase(), &extensions::IMAGE_EXTENSIONS); 

        if file_is_text {
            let file_base_name_without_extension: &OsStr = &extensions::get_base_name(file_path_as_string).unwrap();
            let destination_directory: &PathBuf = &author_root.join(file_base_name_without_extension);
            
            if destination_directory.exists(){
                let _ = fs::remove_dir(destination_directory);
                let _ = fs::create_dir(destination_directory);
            } 

            fs::rename(file_path_as_string, destination_directory).unwrap();  // Move the file
                
            paths_of_downloaded_files.push(
                destination_directory.to_str().unwrap().to_string()
            );
           
        } else if file_is_image {
            continue
        } else {
            log::error!(
                "The file at {} is neither has none of the expected extensions", file.display()
            )
        }
    }
}


fn log_downloaded_files(
    author_name: &String, 
    paths_of_downloaded_files: &Vec<String>, 
) {

    log::info!("Logging Downloaded Files");
    let mut object_types_and_paths = HashMap::new();
    let author_root = utils::get_author_root(author_name); 

    object_types_and_paths.insert(
       author_root.join("downloaded_files.json"), paths_of_downloaded_files
    );

    for (log_path, log_content) in &object_types_and_paths {

        if log_path.exists() {
            fs::remove_file(log_path).expect("A file could not be removed")
        }

        let new_log_file = fs::File::create(log_path).unwrap();
        serde_json::to_writer(new_log_file, log_content).expect("Problem writing json for downloads")
    } 
}


fn remove_book_directories(directories: Vec<PathBuf>) {

    for directory in kdam::tqdm!(
        directories.iter(),
        desc="Removing original directories"

    ) {
        if directory.exists() {
            fs::remove_dir_all(directory).expect("Cannot remove directory and its contents");
        }
    }
}


fn list_path_contents(path: &PathBuf) -> Result<Vec<Result<PathBuf, GlobError>>, anyhow::Error> {
    let pattern: String = format!("{}/**/*", path.to_str().unwrap());
    
    let contents: Vec<Result<PathBuf, glob::GlobError>> = glob::glob(&pattern)?
        .map(
            |entry| match entry {
                Ok(path) => Ok(path),
                Err(err) => Err(err) 
            }
        ).collect();

    Ok(contents)
}

