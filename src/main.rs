use medman::cli::CliArguments;
use medman::musicfile::MusicFile;
use medman::scan::{scan, scan_add_tag};
use medman::write2md::{write2md};
use medman::search::{search};
use medman::interact::user_helper;
use medman::write2playlist::write2pls;
use medman::scrap::{scrap};
use std::fs::File;
use std::io::{Write};

fn main() {
    let args = CliArguments::new();
    
    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            match args.seria() {
                false => {},
                true => {
                    let serialized = serde_json::to_string_pretty(&music_files).unwrap();
                    let mut file = File::create("seriafile.json").unwrap();
                    file.write_all(serialized.as_bytes()).expect("Err");
                }
            }
            
        },
        "write2md" => {
            match args.deseria() {
                false => {
                    match args.path().to_str() {
                        None => println!("Veulliez indiquer le path ou un fichier sérialié des musiques à écrire dans le markdown"),
                        Some(_) => {
                            let music_files = scan(args.path());
                            write2md(music_files);
                        },
                    }
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    write2md(deserialize);
                }
            }
           
        },
        "search" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    search(music_files, &args.search());
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    search(deserialize, &args.search());
                },
            }
            
        },
        "tag" => {
            scan_add_tag(args.path(), &args.category(), &args.tag());
        },
        "write2playlist" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    write2pls(music_files);
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    write2pls(deserialize);
                },
            }           
        },
        "scrap" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    let result = scrap(music_files);
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    let result = scrap(deserialize);
                },
            }
        },
        _ => user_helper(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn scan_file() {
        //MusicFile list of music in folder.
        let mut music_files: Vec<MusicFile> = Vec::new();
        music_files.push(MusicFile::new(
            std::path::Path::new("folder/lomepal - trop beau.mp3"),
            " trop beau".to_string(),
            2019,
            "lomepal ".to_string(),
            "3 jours à Motorbass".to_string()
        ));
        music_files.push(MusicFile::new(
            std::path::Path::new("folder/Therapie TAXI - Ete 90.mp3"),
            " Ete 90".to_string(),
            2021,
            "Therapie TAXI ".to_string(),
            "Rupture 2 merde".to_string()
        ));

        //scan function.
        let music = scan(std::path::Path::new("folder"));
        
        // convert into string to compare it
        assert_eq!(serde_json::to_string_pretty(&music_files).unwrap(), serde_json::to_string_pretty(&music).unwrap());
    }
}

