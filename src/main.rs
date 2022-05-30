extern crate chrono;
extern crate chrono_tz;
extern crate dotenv;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;

use serenity::model::prelude::Ready;

use serenity::model::id::EmojiId;

use serenity::model::prelude::ReactionType;
use serenity::prelude::*;
use std::env;

use chrono::{NaiveTime, Timelike};
use chrono_tz::Asia::Tokyo;

use crate::digit_emotes::{get_digit_emote, DigitType};

mod digit_emotes;

#[group]
#[commands(time)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == 246941577904128000 {
            msg.reply(&ctx, "Dieser Benutzer stinkt").await.unwrap();
        }
    }
}

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
async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let naive_time = msg.timestamp.with_timezone(&Tokyo).time();

    let d = digits_from_naive_time(&naive_time);

    react_time(ctx, msg, d.0, d.1, d.2, d.3).await;

    msg.reply(ctx, naive_time.hour()).await?;
    msg.reply(ctx, naive_time.minute()).await?;

    Ok(())
}

fn digits_from_naive_time(time: &NaiveTime) -> (u32, u32, u32, u32) {
    let hour = time.hour();
    let minute = time.minute();

    let hour_ten = hour / 10;
    let hour_one = hour % 10;

    let minute_ten = minute / 10;
    let minute_one = minute % 10;

    (hour_ten, hour_one, minute_ten, minute_one)
}

async fn react_time(ctx: &Context, msg: &Message, a: u32, b: u32, c: u32, d: u32) {
    let guild = msg.guild(ctx).unwrap();

    let mut emojis: Vec<ReactionType> = Vec::new();

    let digits: [(u32, DigitType); 4] = [
        (a, DigitType::HourTen),
        (b, DigitType::HourOne),
        (c, DigitType::MinuteTen),
        (d, DigitType::MinuteOne),
    ];

    for digit in digits.iter() {
        match get_digit_emote(&guild, digit.0, &digit.1) {
            Some(emoji) => {
                emojis.push(emoji);
            }
            None => {
                println!("No emoji found for digit {}", digit.0);
            }
        }
    }

    for emoji in emojis.into_iter() {
        msg.react(ctx, emoji).await.unwrap();
    }
}
