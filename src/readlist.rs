use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Feed {
    name: String,
    url: String,
    last_post: String,
}

use serde_json::Result;


fn get_feeds() -> Result<Vec<Feed>> {
    // Grab JSON file
    let file_path = "/var/www/RustyReaderBot/files/list.json";
    //let file_path = "files/list.json";  // for debug
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    
    // Parse the file content to map JSON on Feed object
    let all_feeds: Vec<Feed> = serde_json::from_str(&contents)?;

    Ok(all_feeds)
}


pub fn get_feeds_url() -> Result<Vec<String>> {
    // Get all feed url's
    let urls: Vec<String> = get_feeds()?.iter()
                                        .map(|i| i.url.clone())
                                        .collect();
    Ok(urls)
}


pub fn get_feeds_date(url: &str) -> Result<String> {
    // Get last publish post date
    let date = get_feeds()?.iter()
                                    .filter(|u| u.url == url)
                                    .map(|u| u.last_post.clone())
                                    .collect();
    Ok(date)
}


pub fn get_feeds_name(url: &str) -> Result<String> {
    // Get name of a feed
    let name = get_feeds()?.iter()
                                    .filter(|u| u.url == url)
                                    .map(|u| u.name.clone())
                                    .collect();
    Ok(name)
}


pub fn update_posts_date(new_posts_date: HashMap<String, String>) -> Result<()> {
    // Get Feeds objects
    let mut all_feeds: Vec<Feed> = get_feeds()?;

    for i in all_feeds.iter_mut() {
        if new_posts_date.get(&i.url).is_some() {
            i.last_post = new_posts_date.get(&i.url).unwrap().to_owned();
        }
    }

    let data = serde_json::to_string_pretty(&all_feeds)?;

    // Write JSON file
    let file_path = "/var/www/RustyReaderBot/files/list.json";
    // let file_path = "files/list.json";  // for debug
    fs::write(file_path, data).unwrap();

    Ok(())
}