use std::io::*;
use crate::scan::scan;

pub fn user_helper() {
    println!("Print all commands avaible");
    println!("Scan : permet de scan un dossier pour y enregistrer 
    toutes les musiques et par la suite, modifier les informations");
    println!("write2md : permet de convertir un scan ou un fichier 
    sérialisé en fichier markdown");
    println!("search : permet de trier un scan ou un fichier sérialisé
    avec différents argument ( year / title / artist / album ) et differents
    modificateur ( not / and / or)");
    println!("write2playlist : permet de convertir un scan ou un 
    fichier serialisé en un playlist en .pls");
    println!("tag : permet d'ajouter / modifier une metadata ( tags ) qui sont
    album / year / title / artist");

    let mut buf = String::new();
    let mut toogler = String::new();
    let mut path = String::new();

   
    loop {
        let _ = stdin().read_line(&mut buf);
        match buf.as_str() {
            "scan\n" => {
                println!("");

                println!("où souhaitez vous scanner les musiques ?");
                stdin().read_line(&mut path).expect("Path non reconnu");
                let path = std::path::Path::new(&path[0..path.len()-1]);
                let music_files = scan(&path);

                println!("Souhaitez vous l'enregistrer en json ? ( y / n) ");
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
                break;},

            "write2md\n" => {
                break;},
            "search\n" => {break;},
            _ => {},
        }
        println!("{}",buf);
        buf.clear();
        
    }
}