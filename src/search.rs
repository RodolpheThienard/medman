use crate::musicfile::MusicFile;

/// Making Struct pile 
#[derive(Debug)]
struct Pile {
    value: Vec<Vec<i32>>,
    op: Vec<Operator>,
}
#[derive(Debug)]
enum Operator {
    Not,
    And,
    Or,
}

impl Pile {
    fn new() -> Self {
        Pile{value: Vec::new(),op: Vec::new()}
    }
}

/// search est une fonction qui permet de trier un Vec<MusicFile> avec des requetes de tag sous la forme :
/// year=2000 and title=i\\love\\rock\\and\\roll
/// 
/// La requete peut contenir autant d'argument que désiré. Il y a 3 operateur : NOT / OR / AND
/// 
/// Si l'argument de la requete est un mot composé, il faut utiliser "\\" pour remplacer l'espace
/// 
/// La fonction renvoie un Vec<MusicFile> trié
/// 
/// # Examples: 
/// ```
/// let scaned_files: Vec<MusicFile> = scan(std::path::Path::new("location"));
/// let request = Vec::new();
/// request.push("year=2000".to_string());
/// request.push("and".to_string());
/// request.push("artist=acdc".to_string());
/// 
/// let music_files = search(&scaned_files, &request);
/// ```

pub fn search(music_files: &[MusicFile], request: &[String]) -> Vec<MusicFile> {
    let mut searched: Vec<MusicFile> = Vec::new();
    let mut pile = Pile::new();
    for req in request {
        match req.as_str() {
            "not" => {pile.op.push(Operator::Not)},
            "and" => {pile.op.push(Operator::And)},
            "or" => {pile.op.push(Operator::Or)},
            _ => {pile.value.push(music_filter(music_files, req))},
        }
        
    }
    let tmp = depile(pile);

    for i in 0..music_files.len() {
        if tmp[i] == 1 {
            searched.push(music_files[i].clone());
        }
    }
    searched
}

fn music_filter(music_files: &[MusicFile], req: &str) -> Vec<i32> {
    let mut tmp : Vec<i32> = vec![0;music_files.len() as usize];
    let split = req.split('=');
    let value_category = split.clone().collect::<Vec<&str>>()[0];
    let value = split.clone().collect::<Vec<&str>>()[1];
    for (index, music) in music_files.iter().enumerate() {
        match value_category {
            "title" => {
                if music.title() == value {
                    tmp[index] = 1;
                }
            },
            "artist" => {
                if music.artist() == value {
                    tmp[index] = 1;
                }
            },
            "year" => {
                if music.year() == value {
                    tmp[index] = 1;
                }
            },
            "album" => {
                if music.album() == value {
                    tmp[index] = 1;
                }
            },
            "path" => {
                if music.path() == value {
                    tmp[index] = 1;
                }
            },
            _ => {},
        };
    };
    tmp
}

fn depile(mut pile: Pile) -> Vec<i32> {
    let mut one = pile.value.pop().unwrap();
    let mut operator: Option<Operator> = None;
    let mut two: Vec<i32> = Vec::new();
    while !pile.op.is_empty() || !pile.value.is_empty() {
        if !pile.op.is_empty() {operator = pile.op.pop();}
        if !pile.value.is_empty() {two = pile.value.pop().unwrap();}
        match operator {
            Some(Operator::Not) => {
                for i in 0..one.len() {
                    if one[i] == 0 {one[i] = 1}
                    else { one[i] = 0}
                };
            },
            Some(Operator::And) => {
                for i in 0..one.len() {
                    if one[i] == 1 && two[i] == 1 {one[i] = 1}
                    else {one[i] = 0};
                }
            },
            Some(Operator::Or) => {
                for i in 0..one.len() {
                    if one[i] == 0 && two[i] == 0 {one[i] = 0}
                    else {one[i] = 1};
                }
            },
            None => panic!("Err from depile function"),
        };
        

    }
    one
}