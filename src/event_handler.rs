use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::{self, CacheHttp},
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
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
        if msg.author.id == 246941577904128000 {
            // if the message is from a certain user
            match display_time(&ctx, &msg).await {
                // display the time using reactions and catch errors
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }

        // println!("{}", msg.channel(&ctx));
    }

    // called when a reaction is added to a message
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let emoji = reaction.emoji.to_string();
        let message = reaction.message(&ctx).await.unwrap();
        message.delete_reactions(&ctx);
    }
}