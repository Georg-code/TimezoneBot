// loads crates
extern crate chrono;
extern crate chrono_tz;
extern crate dotenv;

// imports from crates
use dotenv::dotenv;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;
use std::env;

// declare module tree
mod commands;
mod digit_emotes;
mod event_handler;

// imports from module tree
use commands::time::TIME_COMMAND;
use event_handler::Handler;

// general struct: more documentation needed
#[group]
#[commands(time)] // commands need to be listed here
struct General;

// main function (initializes bot)
#[tokio::main]
async fn main() {
    dotenv().ok(); // loads environment variables from .env file
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with the bot token from the environment variables
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
