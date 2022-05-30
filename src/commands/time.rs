use crate::digit_emotes::digits_from_naive_time;
use crate::digit_emotes::{get_digit_emote, DigitType};
use chrono_tz::Asia::Tokyo;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::ReactionType;

// command to react to a message with the time in the specified timezone
#[command]
pub async fn time(ctx: &Context, msg: &Message) -> CommandResult {
    display_time(ctx, msg).await?;
    Ok(())
}

// this function reads the time and then calls the react_time function
pub async fn display_time(ctx: &Context, msg: &Message) -> CommandResult {
    // time is retrieved and converted to a different timezone
    let naive_time = msg.timestamp.with_timezone(&Tokyo).time();

    let d = digits_from_naive_time(&naive_time);

    react_time(ctx, msg, d.0, d.1, d.2, d.3).await?;

    Ok(())
}

// this function reacts to a message with the time using the specified digits
async fn react_time(ctx: &Context, msg: &Message, a: u32, b: u32, c: u32, d: u32) -> CommandResult {
    let guild = match msg.guild(ctx) {
        Some(g) => g,
        None => return Err(CommandError::from("Could not get guild")),
    };

    let mut emojis: Vec<ReactionType> = Vec::new();

    // an array of tuples containing the digits for iteration
    let digits: [(u32, DigitType); 4] = [
        (a, DigitType::HourTen),
        (b, DigitType::HourOne),
        (c, DigitType::MinuteTen),
        (d, DigitType::MinuteOne),
    ];

    // loop over the digits and retrieve the corresponding emotes
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

    // react to the message with the emojis
    for emoji in emojis.into_iter() {
        msg.react(ctx, emoji).await?;
    }

    Ok(())
}
