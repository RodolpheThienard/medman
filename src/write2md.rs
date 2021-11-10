use markdown_gen::markdown::*;
use std::fs::File;

use crate::musicfile::{MusicFile};


pub fn write2md(musicfiles : Vec<MusicFile>) {
    let file = File::create("bar.md").unwrap();
    let mut md = Markdown::new(file);
    for music in musicfiles {
        md.write("Album".heading(1)).unwrap();
        md.write(music.album().paragraph()).unwrap();

        md.write("Title".heading(1)).unwrap();
        md.write(music.title().paragraph()).unwrap();
    }
}