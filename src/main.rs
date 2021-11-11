use medman::cli::CliArguments;
use medman::scan::scan;
use medman::write2md::{write2md};
use std::fs::File;
use std::io::Write;

fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    
    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            let serialized = serde_json::to_string_pretty(&music_files).unwrap();
            let mut file = File::create("bar.json").unwrap();
            file.write_all(serialized.as_bytes()).expect("Err")
        },
        "write2md" => { // add possibility to give text
            match args.path().to_str() {
                None => println!("Veulliez indiquer le texte à écrire dans le markdown"),
                Some(_) => {
                    let music_files = scan(args.path());
                    write2md(music_files, args.path_str());
                },
            }
        },
        "search" => {
            let music_files = scan(args.path());
            
        },
        _ => panic!("Bad command"),
    }
}
