use std::{fs, path::PathBuf};


pub fn get_file_paths(path: &PathBuf) -> Vec<PathBuf> {

    let files: Vec<PathBuf> = fs::read_dir(path)
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

