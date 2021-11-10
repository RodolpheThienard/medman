use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicFile {
    path: PathBuf,
    title: String,
    year: i32,
    artist: String,
    album: String,
}

impl MusicFile {
    pub fn new(path: &Path, title: String, year: i32, artist: String, album: String) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            title,
            year,
            artist,
            album,
        }
    }

    pub fn album(&self) -> String {
        self.album.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}