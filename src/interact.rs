use std::io::*;
use crate::scan::scan;

pub fn user_helper() {
    println!("Print all commands avaible");

    let mut buf = String::new();
    let mut toogler = String::new();

   
    loop {
        let _ = stdin().read_line(&mut buf);
        match buf.as_str() {
            "scan\n" => {
                println!("");

                println!("où souhaitez vous scanner les musiques ?");
                stdin().read_line(&mut buf).expect("Path non reconnu");
                let path = std::path::Path::new(&buf);
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
            "write2md\n" => {break;},
            "search\n" => {break;},
            _ => {},
        }
        println!("{}",buf);
        buf.clear();
        
    }
}