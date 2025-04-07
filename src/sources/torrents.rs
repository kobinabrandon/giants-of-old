use std::path::PathBuf;

use librqbit::Session;
use librqbit::AddTorrent;


#[derive(Clone)]
#[allow(dead_code)]
pub struct ViaTorrent {
    pub magnet: String 
}

impl ViaTorrent {
    
    pub async fn download(&self, file_path: PathBuf) { 

        let session = Session::new(file_path.into()).await.unwrap();

        let torrent_handle = session.add_torrent(
            AddTorrent::from_url(&self.magnet),
            None
        ).await.unwrap().into_handle().unwrap();

        torrent_handle.wait_until_completed().await.unwrap()
    }

}
