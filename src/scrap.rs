use reqwest::Error;
use serde_json::Value;
use crate::musicfile::MusicFile;
use urlencoding::encode;
use audiotags::Tag;

pub fn scrap(music_files: &Vec<MusicFile>) -> Result<(), Error> {

    for music in music_files {
        let split = music.path().split('/').collect::<Vec<&str>>();
        let split_data = split[split.len()-1].split('-'); 

        
        let artist = split_data.clone().collect::<Vec<&str>>()[0];
        let encode_artist = encode(artist);

        let tmp_title = split_data.clone().collect::<Vec<&str>>()[1];
        let title = tmp_title.split('.').collect::<Vec<&str>>()[0];
        let encode_title = encode(title);

        let url = format!("https://musicbrainz.org/ws/2/recording/?query={} AND artists:{}&fmt=json", encode_title, encode_artist);
        let find_data = reqwest::get(url.as_str())?
            .text()?;

        let data: Value = serde_json::from_str(&find_data).unwrap();
        
        let album = &data["recordings"][0]["releases"][0]["title"];
        let year = &data["recordings"][0]["releases"][0]["release-events"][0]["date"];

        let mut value = Tag::default().read_from_path(&music.path()).unwrap();
        value.set_title(title);
        value.set_year(std::str::FromStr::from_str(&year.to_string()[1..5]).unwrap());
        value.set_artist(artist);
        value.set_album_title(&album.as_str().unwrap());
        value.write_to_path(&music.path()).unwrap();
    }
    Ok(())
}