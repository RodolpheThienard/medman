use std::io::*;
use crate::scan::{scan, scan_add_tag};
use crate::scrap::scrap;
use crate::write2md::write2md;
use crate::musicfile::MusicFile;
use crate::search::search;
use crate::write2playlist::write2pls;

fn output(music_files: &Vec<MusicFile>) {
    let mut toogler = String::new();
    
    println!("Souhaitez vous l'enregistrer en json ? (y / n)");
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        let serialized = serde_json::to_string_pretty(&music_files).unwrap();
                        let mut file = std::fs::File::create("interaction.json").unwrap();
                        file.write_all(serialized.as_bytes()).expect("Err");
                    },
                    _ => {},
                }

                println!("Souhaitez vous l'enregistrer en markdown ? (y / n)");
                toogler.clear();
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        write2md(&music_files);
                    },
                    _ => {},
                }

                println!("Souahitez vous créer une playlist avec les musiques ? (y / n)");
                toogler.clear();
                let _ = stdin().read_line(&mut toogler);
                match toogler.as_str() {
                    "n\n" => {},
                    "y\n" => {
                        write2pls(&music_files);
                    },
                    _ => {},
                }
}

/// Fonction pour aiguiller l'utilisateur
pub fn user_helper() {
    let mut buf = String::new();
    let mut toogler = String::new();
    let mut toogler2 = String::new();
    let mut path = String::new();
    let mut category = String::new();
    let mut arguments = String::new();


    println!("Print all commands avaible");

    println!("Scan : permet de scan un dossier pour y enregistrer 
    toutes les musiques et par la suite, modifier les informations");

    println!("search : permet de trier un scan ou un fichier sérialisé
    avec différents argument ( year / title / artist / album ) et differents
    modificateur ( not / and / or)");

    println!("tag : permet d'ajouter / modifier une metadata ( tags ) qui sont
    album / year / title / artist");
   
    'interact: loop {
        let _ = stdin().read_line(&mut buf);
        match buf.as_str() {
            "scan\n" => {
                println!("où souhaitez vous scanner les musiques ?");
                stdin().read_line(&mut path).expect("Path non reconnu");
                let path = std::path::Path::new(&path[0..path.len()-1]);
                let music_files = scan(&path);

                output(&music_files);

                break 'interact;},

            "search\n" => {
                
                println!("souhaitez vous utiliser un fichier serialise ? y/n");
                let _ = stdin().read_line(&mut toogler);

                let mut args_vec: Vec<String> = Vec::new();
                'search: loop {
                    println!("Ecrivez la categorie de la recherche que vous souhaitez faire : 
                    (Ex: year / artist / album / title");
                    let _ = stdin().read_line(&mut category);
                    println!("Ecrivez la restriction que vous souhaitez appliquer");
                    let _ = stdin().read_line(&mut arguments);
                    args_vec.push(format!{"{}={}", category[0..category.len()-1].to_string(), arguments[0..arguments.len()-1].to_string()});
                    println!("Avez vous un autre argument ? y/n");
                    let _ = stdin().read_line(&mut toogler2);
                    match toogler2.as_str() {
                        "y\n" => {
                            println!("quel operateur souhaitez vous ajouter ? ( not / or / and)");
                            arguments.clear();
                            let _ = stdin().read_line(&mut arguments);
                            args_vec.push(format!("{}", arguments[0..category.len()-1].to_string()));
                            arguments.clear();
                            category.clear();
                            toogler2.clear();
                        },
                        "n\n" => {
                            match toogler.as_str() {
                                "n\n" => {
                                    println!("où souhaitez vous scanner les musiques ?");
                                    stdin().read_line(&mut path).expect("Path non reconnu");
                                    let path = std::path::Path::new(&path[0..path.len()-1]);
                                    let music_files = scan(&path);
                                    search(&music_files, &args_vec);
                                    output(&music_files);
                                    break 'search;
                                },
                                "y\n" => {
                                    let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                                    search(&deserialize, &args_vec);
                                    output(&deserialize);
                                    break 'search;
                                },
                                _ => {break;},
                            }
                        },
                        _ => {break;},
                    }
                    
                }
                break 'interact;},


                "tag\n" => {
                    println!("où souhaitez vous modifier les tags des musiques ?");
                    stdin().read_line(&mut path).expect("Path non reconnu");
                    let path = std::path::Path::new(&path[0..path.len()-1]);

                    println!("quel categorie de tag souhaitez vous modifier ?");
                    stdin().read_line(&mut category).expect("Path non reconnu");
                    println!("quel est le nouveau tag ?");
                    stdin().read_line(&mut arguments).expect("Path non reconnu");

                    scan_add_tag(&path, &category, &arguments);
                    break 'interact;
                },


                "scrap\n" => {
                    println!("souhaitez vous utiliser un fichier serialise ? y/n");
                    let _ = stdin().read_line(&mut toogler);

                    match toogler.as_str() {
                        "n\n" => {
                            println!("où souhaitez vous scanner les musiques ?");
                            stdin().read_line(&mut path).expect("Path non reconnu");
                            let path = std::path::Path::new(&path[0..path.len()-1]);
                            let music_files = scan(&path);
                            let _ = scrap(&music_files);
                            output(&music_files);
                            break 'interact;
                        },
                        "y\n" => {
                            let deserialize: Vec<MusicFile> = serde_json::from_str(&std::fs::read_to_string("seriafile.json").expect("msg")).expect("msg");
                            let _ = scrap(&deserialize);
                            output(&deserialize);
                            break 'interact;
                        },
                        _ => {break;},
                    }
                    
                },
            _ => {},
        }
        buf.clear();
        
    }
}