use medman::cli::CliArguments;
use medman::scan::scan;

fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    
    if args.command() == "scan" {
            let music_files = scan(args.path());

            for music_file in music_files {
                println!("{:#?}", music_file);
            }
    }
    else {
        panic!("Bad command");
    }
}
