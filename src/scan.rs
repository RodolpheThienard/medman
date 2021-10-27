use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use mp3_metadata::*;


use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let entry = entry.unwrap(); //TODO gérer l'erreur
        if is_supported(&entry) {
            match read_from_file(&entry.path()) {
                Ok(tags) => 
                    match tags.tag { 
                        Some(values) => music_files.push(MusicFile::new(entry.path(), values)),
                        None => println!("None"),
                    }
                Err(_) => println!("Err"),
            }
        }
    };
    music_files
}
