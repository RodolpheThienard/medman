use medman::cli::CliArguments;
use medman::musicfile::MusicFile;
use medman::scan::scan;
use medman::write2md::{write2md};
use medman::search::{search};
use medman::tag::{set_tag};
use medman::interact::user_helper;
use std::fs::File;
use std::io::{Write};

fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    
    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            match args.save() {
                None => {},
                Some (e) => {
                    let serialized = serde_json::to_string_pretty(&music_files).unwrap();
                    let mut file = File::create(format!("{}.json", e)).unwrap();
                    file.write_all(serialized.as_bytes()).expect("Err");
                }
            }
            
        },
        "write2md" => { // add possibility to give text
            match args.file() {
                None => {
                    match args.path().to_str() {
                        None => println!("Veulliez indiquer le path ou un fichier sérialié des musiques à écrire dans le markdown"),
                        Some(_) => {
                            let music_files = scan(args.path());
                            write2md(music_files, "sériafile".to_string());
                        },
                    }
                },
                Some(e) => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string(format!("{}.json",&e)).expect("msg")).expect("msg");
                    write2md(deserialize, e);
                }
            }
           
        },
        "search" => {
            match args.file() {
                None => {
                    let music_files = scan(args.path());
                    let result = search(music_files, args.search());
                    println!("{:#?}", result);
                },
                Some(file) => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string(format!("{}.json",&file)).expect("msg")).expect("msg");
                    let result = search(deserialize, args.search());
                    println!("{:#?}", result);
                },
            }
            
        },
        "tag" => {
            let music_files = scan(args.path());
            set_tag(music_files);
        },
        _ => user_helper(),
    }
}
