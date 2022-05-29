extern crate dotenv;
extern crate chrono;
extern crate chrono_tz;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;

use serenity::model::prelude::Ready;

use serenity::model::id::EmojiId;

use serenity::prelude::*;
use std::env;
use serenity::model::prelude::ReactionType;

use chrono::{NaiveTime, Timelike};
use chrono_tz::Asia::Tokyo;

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


async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    let naive_time: NaiveTime = msg.timestamp.with_timezone(&Tokyo).time();

    msg.reply(ctx, naive_time.hour()).await?;
    msg.reply(ctx, naive_time.minute()).await?;

    for (emoji_id, emoji) in msg.guild(ctx).unwrap().emojis {
        if emoji.name == "a1" {
            msg.react(ctx, emoji).await?;
            break;
        }
    } 

    Ok(())
}


enum DigitType {
    HourTen,
    HourOne,
    MinuteTen,
    MinuteOne,
}

// fn get_digit_emote(digit: i32, digit_type: DigitType) -> ReactionType {
//     match digit_type {
//         DigitType::HourTen => match digit {
//             0 => ReactionType::Unicode("🇰".to_string()),
//             1 => ReactionType::Unicode("🇱".to_string()),
//             2 => ReactionType::Unicode("🇲".to_string()),
//             3 => ReactionType::Unicode("🇳".to_string()),
//             4 => ReactionType::Unicode("🇴".to_string()),
//             5 => ReactionType::Unicode("🇵".to_string()),
//             6 => ReactionType::Unicode("🇶".to_string()),
//             7 => ReactionType::Unicode("🇷".to_string()),
//             8 => ReactionType::Unicode("🇸".to_string()),
//             9 => ReactionType::Unicode("🇹".to_string()),
//             _ => ReactionType::Unicode("🇺".to_string()),
//         },
//         DigitType::HourOne => match digit {
//             0 => ReactionType::Unicode("🇦".to_string()),
//             1 => ReactionType::Unicode("🇧".to_string()),
//             2 => ReactionType::Unicode("🇨".to_string()),
//             3 => ReactionType::Unicode("🇩".to_string()),
//             4 => ReactionType::Unicode("🇪".to_string()),
//             5 => ReactionType::Unicode("🇫".to_string()),
//             6 => ReactionType::Unicode("🇬".to_string()),
//             7 => ReactionType::Unicode("🇭".to_string()),
//             8 => ReactionType::Unicode("🇮".to_string()),
//             9 => ReactionType::Unicode("🇯".to_string()),
//             _ => ReactionType::Unicode("🇰".to_string()),
//         },
//         DigitType::MinuteTen => match digit {
//             0 => ReactionType::Unicode("🇦".to_string()),
//             1 => ReactionType::Unicode("🇧".to_string()),
//             2 => ReactionType::Unicode("🇨".to_string()),
//     }
// }

