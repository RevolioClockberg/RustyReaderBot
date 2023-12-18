mod commands;
pub mod readlist;
pub mod rss;
pub mod logs;

#[tokio::main]
async fn main() {
    // Get bot token from env
    let main_bot = tbot::Bot::from_env("TELEGRAM_BOT_TOKEN");
    // Get bot name
    let bot_name = main_bot.get_me().call().await.unwrap().user.username.clone().unwrap();

    // Event loop to manage update message with Telegram
    let mut event_loop = main_bot.event_loop();
    // Bot can get command with @BotName
    event_loop.username(bot_name);

    // Associate functions with commands.
    event_loop.command("send_notif", | context | async move { 
        commands::send_notif(context.clone()).await;
        //context.bot.send_message(context.chat.id, "Bonjour").call().await;
    });
    event_loop.command("list", |context| commands::list(context));

    // Start the loop event
    event_loop.polling().start().await.unwrap();
}
