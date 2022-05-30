use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
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
}
