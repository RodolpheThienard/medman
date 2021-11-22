use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {

    #[structopt(default_value="")]
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str),default_value="")]
    path: std::path::PathBuf,

    #[structopt(long = "deserialize", help("to use a serialise json file"))]
    deserialize: bool, // use json

    #[structopt(long="markdown", help("to export in markdown file"))]
    markdown: bool,

    #[structopt( long="playlist", help("to export in pls file"))]
    playlist: bool,

    #[structopt(long = "serialize", help("to serialise a scan into a json file"))]
    serialize: bool, // Json 

    #[structopt(long = "category", required_if("command", "playlist"), help("Category option is used to give the category you would like to change.\nLike tag option, use it only with the command : playlist"))]
    category: Option<String>,

    #[structopt(long = "tag", required_if("command", "playlist"), help("Tag option is used if you want to change a files' tag \nYou have to use it with --category\nExample: --tag value "))]
    tag: Option<String>,

    #[structopt(long = "search", required_if("command", "search"), help("exemple search : artist=name, \nyou can use operator \" and / or / not \" to filter \nexample : artist=name and year=2001, \n/!\\ To put a compose name ( with space ) add : \\: name=composed\\ name"))]
    search: Option<Vec<String>>,


} 

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }
    
    pub fn seria(&self) ->  bool {
        self.serialize 
    }

    pub fn deseria(&self) ->  bool {
        self.deserialize
    }

    pub fn playlist(&self) ->  bool {
        self.playlist
    }

    pub fn markdown(&self) ->  bool {
        self.markdown
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

    pub fn category(&self) -> String {
        self.category.clone().unwrap()
    }

}