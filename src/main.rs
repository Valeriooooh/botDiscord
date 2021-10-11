use serenity::{model::prelude::*, prelude::*, Client};
use std::net::TcpListener;
use std::thread;

struct Handler;

fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        println!("coonection",);
        let _stream = stream.unwrap();
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.contains("!ping") {
            println!("Shard {}", context.shard_id);

            let _ = msg.channel_id.say(&context.http, "Pong!").await;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").unwrap_or("none".to_string());
    thread::spawn(|| server());
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("error creating client");

    if let Err(why) = client.start_shards(3).await {
        println!("Client error: {:?}", why);
    }
}

// https://discord.com/api/oauth2/authorize?client_id=897082403003187210&permissions=536870387447&scope=bot
