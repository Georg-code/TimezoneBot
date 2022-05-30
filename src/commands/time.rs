use crate::digit_emotes::digits_from_naive_time;
use crate::digit_emotes::{get_digit_emote, DigitType};
use chrono_tz::Asia::Tokyo;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::prelude::ReactionType;

pub async fn display_time(ctx: &Context, msg: &Message) -> CommandResult {
    let naive_time = msg.timestamp.with_timezone(&Tokyo).time();

    let d = digits_from_naive_time(&naive_time);

    react_time(ctx, msg, d.0, d.1, d.2, d.3).await?;

    // msg.reply(ctx, naive_time.hour()).await?;
    // msg.reply(ctx, naive_time.minute()).await?;

    Ok(())
}

async fn react_time(ctx: &Context, msg: &Message, a: u32, b: u32, c: u32, d: u32) -> CommandResult {
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

    Ok(())
}
