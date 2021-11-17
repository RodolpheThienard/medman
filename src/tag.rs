use crate::musicfile::MusicFile;
use std::fs::{self};

pub fn set_tag(music_files: Vec<MusicFile>) {
    for music in &music_files {
        let result = fs::read_to_string(music.path());
        match result {
            Ok(e) => println!("{:#?}", e),
            Err(err) => println!("{}", err),
        }
    }
}