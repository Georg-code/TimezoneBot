use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::{CacheHttp, self},
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
    },
};

use crate::commands::time::display_time;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == 246941577904128000 {
            match display_time(&ctx, &msg).await {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }
    }
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let emoji = reaction.emoji.to_string();
        let message = reaction.message(ctx).await.unwrap();
        message.delete_reactions(ctx);

    }
}
