use reqwest::Error;
use serde_json::Value;
use crate::musicfile::MusicFile;
use urlencoding::encode;
use audiotags::Tag;

/// scrap est une fonction qui permet à l'utilisateur de changer les tags de chaque morceau. 
/// 
/// Le fichier a modifier doit être sous la forme : Artist - titre
/// 
/// scrap prend en entrée un vecteur de musicfile et return un Result avec soit 1 soit une erreur
/// 
/// # Examples:
/// ```ignore
/// let music_files: Vec<MusicFile> = scan(std::path::Path::new("location"));
/// 
/// let result = scrap(&music_files);
/// 
/// assert_eq!(result, 1); 
/// ```

pub fn scrap(music_files: &[MusicFile]) -> Result<Vec<&str>, Error> {

    let mut path_list: Vec<&str> = Vec::new();
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
        value.set_album_title(album.as_str().unwrap());
        value.write_to_path(music.path()).unwrap();
        path_list.push(music.path());
    }
    Ok(path_list)
}