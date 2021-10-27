use medman::cli::CliArguments;
use medman::scan::scan;
use markdown_gen::markdown::*;
use std::fs::File;

fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    
    if args.command() == "scan" {
            let music_files = scan(args.path());

            for music_file in music_files {
                println!("{:#?}", music_file);
                let val = serde_json::to_string(&music_file).unwrap();
                println!("{:#?}",val);
                let file = File::create("test.md").unwrap();
                let mut md = Markdown::new(file);
                md.write(val.as_str()).unwrap();
            }
    }
    else {
        panic!("Bad command");
    }
}
