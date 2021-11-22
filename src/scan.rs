use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use audiotags::*;
use crate::musicfile::MusicFile;


const SUPPORTED_EXTENSIONS: [&str; 8] = ["mp3", "mp4", "Flac", "m4a", "m4p", "m4b", "m4r", "m4v"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

/// supported extension : mp3, mp4, Flac, m4a, m4p, m4b, m4r, m4v
/// 
/// Scan effectue une recherche a partir d'un path qui peut etre un dossier ou un ficher.
/// La fonction renvoie un vecteur de MusicFile.
/// # Examples:
/// ```
/// let scaned_files: Vec<MusicFile> = scan(std::path::Path::new("location"));
/// 
/// ```

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    
    for entry in walker {
        match entry {
            Ok(values) => if is_supported(&values) {
                let value = Tag::default().read_from_path(values.path()).unwrap();
                music_files.push(MusicFile::new(
                    values.path(),
                match value.title() {Some(e) => e.to_string(), None => "None".to_string()},
                    match value.year() { Some(e) => e, None => 0,},
                    match value.artist() { Some(e) => e.to_string(), None => "None".to_string()},
                    match value.album_title() { Some(e) => e.to_string(), None => "None".to_string()},
                ));
            },
            Err(err) => panic!("{:?}", err),
        };
        
    };
    music_files
}
/// supported extension : mp3, mp4, Flac, m4a, m4p, m4b, m4r, m4v 
/// 
/// scan_add_tag effectue une recherche a partir d'un path qui peut etre un dossier ou un ficher.
/// La fonction prend en entrée le tag a modifier ainsi que l'argument a mettre puis modifie les metadatas du/des fichier/s.
/// # Examples:
/// ```
/// let tag =  "year";
/// let argument = "2000";
/// scan_add_tag(std::path::Path::new("location"), &tag, &argument);
/// 
/// ```
pub fn scan_add_tag(path: &Path, cat: &str, tag: &str) {
    let walker = WalkDir::new(path).into_iter();
    
    for entry in walker {
        match entry {
            Ok(values) => if is_supported(&values) {
                let mut value = Tag::default().read_from_path(&values.path()).unwrap();
                match cat {
                    "title" => {
                        value.set_title(tag);
                    },
                    "year" => {
                        println!("OUI");
                        value.set_year(std::str::FromStr::from_str(tag).unwrap());
                    },
                    "artist" => {
                        value.set_artist(tag);
                    },
                    "album" => {
                        value.set_album_title(tag);
                    },
                    _ => {println!("Bad category requested. Only Title and Year can be changed.")},
                }
                value.write_to_path(&values.path().to_str().unwrap()).unwrap();
            },
            Err(err) => panic!("{:?}", err),
        };
        
    };
}