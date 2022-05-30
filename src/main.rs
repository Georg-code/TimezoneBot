extern crate chrono;
extern crate chrono_tz;
extern crate dotenv;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;

use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

mod commands;
mod digit_emotes;
mod event_handler;

use commands::time::display_time;
use event_handler::Handler;

#[group]
#[commands(time)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
pub async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    display_time(ctx, msg).await?;
    Ok(())
}
