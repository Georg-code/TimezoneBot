use std::ops::Add;

use serenity::model::{channel::ReactionType, guild::Guild};

pub enum DigitType {
    HourTen,
    HourOne,
    MinuteTen,
    MinuteOne,
}

pub fn get_digit_emote(guild: &Guild, digit: u32, digit_type: &DigitType) -> Option<ReactionType> {
    if digit > 9 {
        println!("Digit must be between 0 and 9");
        return None;
    }

    return match digit_type {
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
            let name = match digit_type {
                DigitType::HourTen => "a",
                DigitType::HourOne => "b",
                DigitType::MinuteTen => "c",
                _ => "",
            }
            .to_string()
            .add(&digit.to_string());

            // println!("name: {}", name);
            for (_, emoji) in &guild.emojis {
                // println!("{:#?}", emoji);
                // println!("{}", emoji.name == name);

                if emoji.name == name {
                    return Some(ReactionType::from(emoji.to_owned()));
                }
            }

            None
        }
    };
}
