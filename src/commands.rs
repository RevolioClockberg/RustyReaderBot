use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{delay_for, Duration};
use tbot::{contexts::{Command, Text}, types::parameters};

use crate::readlist;
use crate::rss;
use crate::logs;


pub async fn send_notif(context: Arc<Command<Text>>) {
    tokio::spawn(async move {
        loop {
            // Hasmap to get and update last post send
            let mut new_posts_date: HashMap<String, String> = HashMap::new();

            // Get RSS URLs
            match readlist::get_feeds_url() {
                Ok(feeds_urls) => {
                    // Parse URLs
                    for url in feeds_urls {
                        if rss::check_url(&url).await {
                            // Get flux names
                            match readlist::get_feeds_name(&url) {
                                Ok(name) => {
                                    // Get date of last post send by message
                                    match readlist::get_feeds_date(&url) {
                                        Ok(last_posted_msg) => {
                                            // Get date of last post from site
                                            match rss::get_post_date(&url).await{
                                                Ok(last_post) => {
                                                    // Compare date
                                                    if last_posted_msg != last_post {
                                                        // Keep new date for update
                                                        new_posts_date.insert(url.clone(), last_post.clone());
                                                        // Get post and send it by message
                                                        match rss::get_rss(&url, name).await {
                                                            Ok(result) => {
                                                                let _ = context.bot.send_message(
                                                                    context.chat.id,
                                                                    parameters::Text::with_markdown(&result)
                                                                ).is_web_page_preview_disabled(true).call().await;
                                                            },
                                                            Err(e) => logs::write_logs(e.to_string()),
                                                        }
                                                    }
                                                    delay_for(Duration::from_secs(10)).await;
                                                },
                                                Err(e) => logs::write_logs(e.to_string()),
                                            }
                                        },
                                        Err(e) => logs::write_logs(e.to_string()),
                                    }
                                },
                                Err(e) => logs::write_logs(e.to_string()),
                            }
                        }
                    }
                },
                Err(e) => logs::write_logs(e.to_string()),
            }
            // Update new dates from new posts
            match readlist::update_posts_date(new_posts_date) {
                Ok(_result) => delay_for(Duration::from_secs(600)).await, // Wait 10min before next check
                Err(e) => logs::write_logs(e.to_string()),
            } 
        }
    });
}

pub async fn list(context: Arc<Command<Text>>) {
    let mut list_feeds_name = String::new();
    // Get RSS URLs
    match readlist::get_feeds_url() {
        Ok(feeds_urls) => {
            // Parse URLs
            for url in feeds_urls {
                // Get flux names
                match readlist::get_feeds_name(&url) {
                    Ok(name) => list_feeds_name.push_str(&format!("- {}\n", name)),
                    Err(e) => logs::write_logs(e.to_string()),
                }
            }
        },
        Err(e) => logs::write_logs(e.to_string()),
    }
    
    let _ =context.bot.send_message(context.chat.id, &list_feeds_name).call().await;
}