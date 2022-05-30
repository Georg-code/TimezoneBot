use std::ops::Add;

use chrono::{NaiveTime, Timelike};
use serenity::model::{channel::ReactionType, guild::Guild};

// enum for the different digits / emote-groups
pub enum DigitType {
    HourTen,
    HourOne,
    MinuteTen,
    MinuteOne,
}

// returns the emote for a given digit
pub fn get_digit_emote(guild: &Guild, digit: u32, digit_type: &DigitType) -> Option<ReactionType> {
    // returns if the digit is invalid
    if digit > 9 {
        println!("Digit must be between 0 and 9");
        return None;
    }

    return match digit_type {
        // minute one digit uses the default emotes
        DigitType::MinuteOne => Some(ReactionType::Unicode(match digit {
            0 => "0️⃣".to_string(),
            1 => "1️⃣".to_string(),
            2 => "2️⃣".to_string(),
            3 => "3️⃣".to_string(),
            4 => "4️⃣".to_string(),
            5 => "5️⃣".to_string(),
            6 => "6️⃣".to_string(),
            7 => "7️⃣".to_string(),
            8 => "8️⃣".to_string(),
            9 => "9️⃣".to_string(),
            _ => "".to_string(),
        })),
        _ => {
            // the rest of the digits use custom emotes
            // first the name of the custom emote is constructed
            let name = match digit_type {
                DigitType::HourTen => "a",
                DigitType::HourOne => "b",
                DigitType::MinuteTen => "c",
                _ => "",
            }
            .to_string()
            .add(&digit.to_string());

            // the guild emotes are retrieved and filtered
            // TODO: this is a bit ugly, replace it with a filter function
            for (_, emoji) in &guild.emojis {
                if emoji.name == name {
                    // the emote is converted to a reaction type and returned
                    return Some(ReactionType::from(emoji.to_owned()));
                }
            }

            // if the emote is not found, None is returned
            None
        }
    };
}

// splits a NaiveTime into its unique digits and returns it as a Tuple
pub fn digits_from_naive_time(time: &NaiveTime) -> (u32, u32, u32, u32) {
    let hour = time.hour();
    let minute = time.minute();

    let hour_ten = hour / 10;
    let hour_one = hour % 10;

    let minute_ten = minute / 10;
    let minute_one = minute % 10;

    (hour_ten, hour_one, minute_ten, minute_one)
}
