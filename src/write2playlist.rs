use pls::{PlaylistElement, ElementLength};
use crate::musicfile::MusicFile;
use std::fs::File;
use std::io::Write;

/// Write2pls est une fonction qui permet de transformer un Vec<MusicFile> en un fichier pls ( une playlist)
/// 
/// La fonction est très utile pour un utilisateur qui aurait besoin d'une playlist de ses musiques.
/// 
/// Le fichier de sorti portera le nom : playlist.pls
/// # Examples : 
/// ```ignore
/// let music_files: VecMusicFile> = scan(std::path::Path::new("location"));
/// 
/// write2pls(&music_files);
/// ```

pub fn write2pls(music_files: &[MusicFile]) {

    let mut buf = Vec::new();
    let mut playlist: Vec<PlaylistElement> = Vec::new();
    
    for element in music_files {
        playlist.push(PlaylistElement {
            path: element.path().to_string(),
            title: Some(element.title()),
            len: ElementLength::Unknown,
        });
    }

    pls::write(&playlist, &mut buf).unwrap();
    let mut file = File::create("playlist.pls").unwrap();
    let _ = file.write(&buf);
}