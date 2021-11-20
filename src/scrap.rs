use reqwest::Error;
use serde_json::Value;

pub fn scrap() -> Result<i32, Error>{
    let category = "artist";
    let content = "lomepal";
    let url = format!("http://musicbrainz.org/ws/2/{}/?query={}&fmt=json", category, content);
    let find_data = reqwest::get(url.as_str())?
        .text()?;

    let data: Value = serde_json::from_str(&find_data).unwrap();
    
    let test = &data["artists"][0];
    
    println!("body = {:#?}", test);
    
    Ok(3)
}