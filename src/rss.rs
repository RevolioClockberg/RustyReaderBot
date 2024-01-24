use std::error::Error;
use chrono::DateTime;
use rss::*;
use dissolve::strip_html_tags;
use reqwest::{self, Url};
use reqwest::header::USER_AGENT;


// Check if an url can join RSS feeds list. 
pub async fn check_url(url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let err_url = url.clone();
    let url = Url::parse(url)?;
    let client = reqwest::Client::new();
    let content = client.get(url).header(USER_AGENT, "Rusty Bot").send().await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;

    match channel.items.first().unwrap().pub_date() {
        Some (_) => Ok(()),
        None => {
            let e: Box<dyn Error + Send + Sync> = format!("Can't get publication date from {}", err_url).into();
            return Err(e);
        },
    }
}


// Get RSS post date
pub async fn get_post_date(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Get all RSS objects (each posts from an url)
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    let mut publish_date = String::new();

    // Get the last post of feed
    if let Some(last_post) = channel.items().first() {
        publish_date = 
            match last_post.pub_date() {
                Some(result) => DateTime::parse_from_rfc2822(result).unwrap().format("%Y/%m/%d-%H:%M").to_string(),
                None => String::from("no publish date"),
            };
    }
    Ok(publish_date)
}


// Get all RSS post informations
pub async fn get_rss(url: &str, name:String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut msg = String::from("no post found");

    // Get all RSS objects (each posts from an url)
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;

    // Get the last post of feed
    if let Some(last_post) = channel.items().first() {

        // Get post publish date
        let publish_date = get_post_date(&url).await?;

        // Get post title
        let title = {
            match last_post.title() {
                Some(title) => title,
                None => "no title found",
            }
        };

        // Get post link
        let link = {
            match last_post.link() {
                Some(link) => link,
                None => last_post.guid().unwrap().value(),
            }
        };

        // Get post description
        let description = {
            match last_post.description() {
                Some(desc) => {
                    let mut desc_format = strip_html_tags(desc).join(" ").replace("  ", " ");
                    desc_format = truncate(&desc_format, 300).to_string();
                    let offset = desc_format.rfind('.').unwrap_or(desc_format.len());
                    desc_format = desc_format.drain(..offset).collect();
                    desc_format.push('.');
                    desc_format
                },
                None => String::from("no description"),
            }
        };

        // Format final message (Markdown)
        msg = format!("[{}]({})\n\n{}\n\n{} - {}", title, link, description, name, publish_date);
    }
    Ok(msg)
}


// Custom truncate function to reduce string length
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
