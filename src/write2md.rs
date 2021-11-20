use markdown_gen::markdown::*;
use std::{fs::File};

use crate::musicfile::{MusicFile};


pub fn write2md(musicfiles : Vec<MusicFile>) {

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