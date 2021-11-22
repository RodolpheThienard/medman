use markdown_gen::markdown::*;
use std::{fs::File};

use crate::musicfile::{MusicFile};

/// Write2md est une fonction qui permet de transformer un Vec<MusicFile> en un fichier markdown
/// 
/// La fonction est très utile pour un utilisateur qui aurait besoin d'un rendu visuel des musiques.
/// 
/// Le fichier de sorti portera le nom : seriafile.md
/// # Examples : 
/// ```
/// let music_files: Vec<MusicFile> = scan(std::path::Path::new("location"));
/// 
/// write2md(&music_files);
/// ```
/// 
pub fn write2md(musicfiles : &Vec<MusicFile>) {

    let mut i = 0;
    let file = File::create("seriafile.md").unwrap();
    let mut md = Markdown::new(file);
    for music in musicfiles {
        
        md.write(format!("N° : {}, path :  {}", i.to_string(), music.path()).heading(2)).unwrap();
        
        md.write(format!("{}{}", "Artist : ", music.artist()).paragraph()).unwrap();

        md.write(format!("{}{}", "Album : ", music.album()).paragraph()).unwrap();

        md.write(format!("{}{}", "Title : ", music.title()).paragraph()).unwrap();

        md.write(format!("{}{}", "Année : ", music.year()).paragraph()).unwrap();

        i += 1;
    }
}