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

fn check_md_pls(args: CliArguments, music_files: &[MusicFile]) {
    match (args.markdown(), args.playlist()) {
        (false, false) => {},
        (false, true) => {
            write2pls(music_files);
        },
        (true, false) => {
            write2md(music_files);
        },
        (true, true) => {
            write2pls(music_files);
            write2md(music_files);
        },
    }
}

fn main() {
    let args = CliArguments::new();
    
    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            match args.seria() {
                false => {
                    check_md_pls(args, &music_files);
                },
                true => {
                    let serialized = serde_json::to_string_pretty(&music_files).unwrap();
                    let mut file = File::create("seriafile.json").unwrap();
                    file.write_all(serialized.as_bytes()).expect("Err");
                    check_md_pls(args, &music_files);
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
                            write2md(&music_files);
                        },
                    }
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    write2md(&deserialize);
                }
            }
           
        },
        "search" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    let result = search(&music_files, &args.search());
                    check_md_pls(args, &result);
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    search(&deserialize, &args.search());
                    check_md_pls(args, &deserialize);
                },
            }
            
        },
        "tag" => {
            scan_add_tag(args.path(), &args.category(), &args.tag());
            let music_files = scan(args.path());
            check_md_pls(args, &music_files);
        },
        "write2playlist" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    write2pls(&music_files);
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    write2pls(&deserialize);
                },
            }           
        },
        "scrap" => {
            match args.deseria() {
                false => {
                    let music_files = scan(args.path());
                    let _ = scrap(&music_files);
                    check_md_pls(args, &music_files);
                },
                true => {
                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                    let _ = scrap(&deserialize);
                    check_md_pls(args, &deserialize);
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
        //MusicFile list of music in folder.

        //scan function.
        let music = scan(std::path::Path::new("folder"));
        
        // convert into string to compare it
        assert_eq!(serde_json::to_string_pretty(&music_files).unwrap(), serde_json::to_string_pretty(&music).unwrap());
    }

    #[test]
    fn check_md_file() {
        if std::fs::read("seriafile.md").is_ok() {
            let _ = std::fs::remove_file("seriafile.md");
        }
        let music = scan(std::path::Path::new("folder"));
        write2md(&music);

        assert!(std::fs::read("seriafile.md").is_ok());
    }

    #[test]
    fn check_tag() {
        scan_add_tag(std::path::Path::new("folder/lomepal - trop beau.mp3"), "year", "2000");
        let scan_init = scan(std::path::Path::new("folder/lomepal - trop beau.mp3"));

        scan_add_tag(std::path::Path::new("folder/lomepal - trop beau.mp3"), "year", "2019");
        let scan_compare = scan(std::path::Path::new("folder/lomepal - trop beau.mp3"));

        // check if scan_init is different of scan_compare
        assert_ne!(serde_json::to_string_pretty(&scan_init).unwrap(), serde_json::to_string_pretty(&scan_compare).unwrap());
    }

    #[test]
    fn check_search() {
        let mut music_files: Vec<MusicFile> = Vec::new();
        music_files.push(MusicFile::new(
            std::path::Path::new("folder/Therapie TAXI - Ete 90.mp3"),
            " Ete 90".to_string(),
            2021,
            "Therapie TAXI ".to_string(),
            "Rupture 2 merde".to_string()
        ));
        //MusicFile list of music in folder.

        //scan function & take 2 musics
        let music = scan(std::path::Path::new("folder"));
        let mut request = Vec::new();
        request.push("year=2021".to_string());
        let result = search(&music, &request);
        
        // convert into string to compare it
        assert_eq!(serde_json::to_string_pretty(&music_files).unwrap(), serde_json::to_string_pretty(&result).unwrap());
    }

    #[test]
    fn check_playlist() {
        if std::fs::read("playlist.pls").is_ok() {
            let _ = std::fs::remove_file("playlist.pls");
        }
        let music = scan(std::path::Path::new("folder"));
        write2pls(&music);

        assert!(std::fs::read("playlist.pls").is_ok());
    }
}

