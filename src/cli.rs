use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    #[structopt(default_value="")]
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str),default_value="")]
    path: std::path::PathBuf,

    // https://stackoverflow.com/questions/60717253/structopt-how-to-combine-all-arguments-in-a-single-string
    #[structopt(short = "f", long = "file")]
    deserialize: Option<String>, // use json

    #[structopt(long = "save")]
    serialize: Option<String>, // Json 

    #[structopt(short = "d", long = "display")]
    diplay: Option<String>, // console / Markdown 

    #[structopt(short = "pl", long = "playlist")]
    playlist: Option<String>,

    #[structopt(long = "cat")]
    cat: Option<String>,
    #[structopt(long = "tag")]
    tag: Option<String>,

    #[structopt(short = "s", long = "search", required_if("command", "search"), help("exemple search : artist=name, \nyou can use operator \" and / or / not \" to filter \nexample : artist=name and year=2001, \n/!\\ To put a compose name ( with space ) add : \\: name=composed\\ name"))]
    search: Option<Vec<String>>,
} 

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }
    
    pub fn save(&self) ->  Option<String> {
        self.serialize.clone()
    }

    pub fn file(&self) ->  Option<String> {
        self.deserialize.clone()
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

    pub fn search(&self) -> Vec<String> {
        self.search.clone().unwrap()
    }

    pub fn tag(&self) -> String {
        self.tag.clone().unwrap()
    }

    pub fn cat(&self) -> String {
        self.cat.clone().unwrap()
    }

}
