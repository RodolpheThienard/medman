use markdown_gen::markdown::*;
use std::{fs::File, ops::Index};

use crate::musicfile::{MusicFile};


pub fn write2md(musicfiles : Vec<MusicFile>, path: &str) {

    let title_path = format!("{}{}", "All music in ", path);
    let mut i = 0;
    let file = File::create("bar.md").unwrap();
    let mut md = Markdown::new(file);
    md.write(title_path.heading(1)).unwrap();
    for music in musicfiles {
        
        md.write(format!("N° : {}, path :  {}", i.to_string(), music.path()).heading(2)).unwrap();
        
        md.write(format!("{}{}", "Artist : ", music.artist()).paragraph()).unwrap();

        md.write(format!("{}{}", "Album : ", music.album()).paragraph()).unwrap();

        md.write(format!("{}{}", "Title : ", music.title()).paragraph()).unwrap();

        md.write(format!("{}{}", "Année : ", music.year()).paragraph()).unwrap();

        i += 1;
    }
}