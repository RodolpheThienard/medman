use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // https://stackoverflow.com/questions/60717253/structopt-how-to-combine-all-arguments-in-a-single-string
    
    
    #[structopt(short = "s", long = "search", default_value = "None")]
    search: String,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }

    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }
}
