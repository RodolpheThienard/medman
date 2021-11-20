use reqwest::Error;
use serde_json::Value;
use crate::musicfile::MusicFile;
use urlencoding::encode;

pub fn scrap(music_files: Vec<MusicFile>) -> Result<i32, Error> {

    for music in music_files {
        let split = music.path().split('/').collect::<Vec<&str>>();
        let split_data = split[split.len()-1].split('-'); 

        let encode_artist = encode(split_data.clone().collect::<Vec<&str>>()[0]);
        let tmp_title = encode(split_data.clone().collect::<Vec<&str>>()[1]);
        let encode_title = tmp_title.split('.').collect::<Vec<&str>>()[0];
        let url = format!("https://musicbrainz.org/ws/2/recording/?query={} AND artists:{}&fmt=json", encode_title, encode_artist);
        let find_data = reqwest::get(url.as_str())?
            .text()?;

        let data: Value = serde_json::from_str(&find_data).unwrap();
        
        let test = &data["recordings"][0];
        
        println!("body = {:#?}", test);
    }
    Ok(3)
}