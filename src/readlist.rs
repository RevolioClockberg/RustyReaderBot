use std::collections::HashMap;
use std::fs;
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
    let file_path = "files/list.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    
    // Parse the file content to map JSON on Feed object
    let all_feeds: Vec<Feed> = serde_json::from_str(&contents)?;

    Ok(all_feeds)
}


pub fn get_feeds_url() -> Result<Vec<String>> {
    // Get Feeds objects
    let all_feeds = get_feeds()?;

    let mut urls: Vec<String> = Vec::new();

    // Parse all feeds to display them
    for i in all_feeds.iter() {
        let j = i.url.clone();
        urls.push(j);
    }
    Ok(urls)
}


pub fn get_feeds_date(url: &str) -> Result<String> {
    // Get Feeds objects
    let all_feeds = get_feeds()?;

    let mut date = String::new();

    for i in all_feeds.iter() {
        if i.url == url {
            date = i.last_post.clone();
        }
    }
    Ok(date)
}


pub fn get_feeds_name(url: &str) -> Result<String> {
    // Get Feeds objects
    let all_feeds = get_feeds()?;

    let mut name = String::new();

    for i in all_feeds.iter() {
        if i.url == url {
            name = i.name.clone();
        }
    }
    Ok(name)
}


pub fn update_posts_date(new_posts_date: HashMap<String, String>) -> Result<()> {
    // Get Feeds objects
    let mut all_feeds = get_feeds()?;

    for i in all_feeds.iter_mut() {
        if new_posts_date.get(&i.url).is_some() {
            i.last_post = new_posts_date.get(&i.url).unwrap().clone();
        }
    }

    let data = serde_json::to_string_pretty(&all_feeds)?;

    // Write JSON file
    let file_path = "files/list.json".to_owned();
    fs::write(file_path, data).unwrap();

    Ok(())
}