use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::{self, CacheHttp},
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
        id::MessageId,
    },
};

use crate::commands::time::display_time;

pub struct Handler;

// bot event handlers must be registered here, see documentation for available events
#[async_trait]
impl EventHandler for Handler {
    // called when the bot is done initializing
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // called when a message is sent in a channel
    async fn message(&self, ctx: Context, msg: Message) {
        message_grouping_test(&ctx, &msg).await;

        if msg.author.id == 246941577904128000 {
            // if the message is from a certain user
            match display_time(&ctx, &msg).await {
                // display the time using reactions and catch errors
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        };

        println!("{}", msg.channel(&ctx).await.unwrap());
    }

    // called when a reaction is added to a message
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let emoji = reaction.emoji.to_string();
        let message = reaction.message(&ctx).await.unwrap(); // Try to not use unwrap() Hohi, use match instead;
        message.delete_reactions(&ctx);
    }
}

async fn message_grouping_test(ctx: &Context, msg: &Message) {
    let guild_channel = msg.channel(&ctx).await.unwrap().guild().unwrap();

    let last_message_id = guild_channel.last_message_id.unwrap().0;

    let last_messages = guild_channel
        .messages(ctx, |retriever| {
            retriever.after(MessageId(last_message_id)).limit(5)
        })
        .await
        .unwrap();

    println!("{:?}", last_messages.len());

    for message in last_messages {
        for reaction in message.reactions.iter() {
            if reaction.me {
                message
                    .delete_reaction_emoji(ctx, reaction.reaction_type.to_owned())
                    .await;
            }
        }
    }
}
