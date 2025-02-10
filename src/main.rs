use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{delay_for, Duration};
use tbot::{contexts::{Command, Text}, types::parameters};


pub mod readlist;
pub mod rss;
pub mod logs;

#[tokio::main]
async fn main() {
    // Create bot from token
    let bot = tbot::Bot::from_env("TELEGRAM_BOT_TOKEN");
    // Get bot name
    let bot_name = bot.get_me().call().await.unwrap().user.username.unwrap();
    
    // Event loop to manage message with Telegram
    let mut event_loop = bot.event_loop();

    // Bot can get command with @BotName
    event_loop.username(bot_name);

    // Associate functions with commands
    event_loop.command("start", |context| start(context));
    event_loop.command("list", |context| list(context));

    // Start the loop event
    event_loop.polling().start().await.unwrap();
}


async fn start(context: Arc<Command<Text>>) {
    let _ = context.bot.send_message(
        context.chat.id.clone(),
        parameters::Text::with_markdown("Ok, I'm Starting !")
    ).is_web_page_preview_disabled(true).call().await;
    loop {
        // Hashmap to get and update last post send
        let mut new_posts_date: HashMap<String, String> = HashMap::new();

        // Get RSS URLs
        match readlist::get_feeds() {
            Ok(all_feeds) => {
                // Parse URLs
                for feed in all_feeds {
                    match rss::check_url(&feed.url).await {
                        Ok (_) => {
                            // Get date of last post from site
                            match rss::get_post_date(&feed.url).await{
                                Ok(last_publication) => {
                                    // Compare date
                                    if (&feed.last_post != &last_publication) || (&feed.last_post == "no publish date") {
                                        // Keep new date for update
                                        new_posts_date.insert(feed.url.clone(), last_publication.clone());
                                        // Get post and send it by message
                                        match rss::get_rss(&feed.url, &feed.name, &last_publication).await {
                                            Ok(result) => {
                                                if Ok(String::from("TRUE")) == env::var("RUSTY_BOT_LOGS") {
                                                    logs::write_debug(format!("Send message for {} RSS feed with {} URL.", &feed.name, &feed.url))
                                                }
                                                let _ = context.bot.send_message(
                                                    context.chat.id.clone(),
                                                    parameters::Text::with_markdown(&result)
                                                ).is_web_page_preview_disabled(true).call().await;
                                            },
                                            Err(e) => logs::write_logs(format!("Error with {} feed --- {}", &feed.name, e.to_string())),
                                        }
                                    }
                                    delay_for(Duration::from_secs(30)).await;
                                },
                                Err(e) => logs::write_logs(format!("Error with {} feed --- {}", &feed.name, e.to_string())),
                            }
                        },
                        Err(e) => logs::write_logs(format!("Error with {} feed --- {}", &feed.name, e.to_string())),
                    }
                }
            },
            Err(e) => logs::write_logs(format!("Error to get feeds on local file --- {}", e.to_string())),
        }
        // Update new dates from new posts
        match readlist::update_posts_date(new_posts_date) {
            // Wait 10min before next check
            Ok(_result) => delay_for(Duration::from_secs(600)).await,
            Err(e) => logs::write_logs(format!("Error to update dates on local file --- {}", e.to_string())),
        } 
    }
}


async fn list(context: Arc<Command<Text>>) {
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
    let _ = context.bot.send_message(context.chat.id, &list_feeds_name).call().await;
}