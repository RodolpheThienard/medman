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
        match entry {
            Ok(values) => if is_supported(&values) {
                match read_from_file(&values.path()) {
                    Ok(tags) => music_files.push(MusicFile::new(values.path(), tags.optional_info)),
                    Err(_) => println!("Err"),
                }
            },
            Err(err) => panic!("{:?}", err),
        };
        
    };
    music_files
}
