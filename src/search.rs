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

pub fn search(music_files: &Vec<MusicFile>, request: &Vec<String>) -> Vec<MusicFile> {
    let mut searched: Vec<MusicFile> = Vec::new();
    let mut pile = Pile::new();
    for req in request {
        match req.as_str() {
            "not" => {pile.op.push(Operator::Not)},
            "and" => {pile.op.push(Operator::And)},
            "or" => {pile.op.push(Operator::Or)},
            _ => {pile.value.push(music_filter(&music_files, req))},
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

fn music_filter(music_files: &Vec<MusicFile>, req: &String) -> Vec<i32> {
    let mut tmp : Vec<i32> = vec![0;music_files.len() as usize];
    let mut index = 0;
    let split = req.split("=");
    let value_category = split.clone().collect::<Vec<&str>>()[0];
    let value = split.clone().collect::<Vec<&str>>()[1];
    for music in music_files {
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
            _ => {"";},
        };
        index += 1;
    };
    tmp
}

fn depile(mut pile: Pile) -> Vec<i32> {
    let mut one = pile.value.pop().unwrap();
    let mut operator: Option<Operator> = None;
    let mut two: Vec<i32> = Vec::new();
    while pile.op.len() !=0 || pile.value.len() !=0 {
        if pile.op.len() !=0 {operator = pile.op.pop();}
        if pile.value.len() !=0 {two = pile.value.pop().unwrap();}
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