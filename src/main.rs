mod commands;
mod events;
mod server;
use commands::math::*;
use commands::reply::*;
use serenity::{framework::standard::StandardFramework, http::Http, Client};
use std::{collections::HashSet, thread};
// Start ////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").unwrap_or("none".to_string());
    thread::spawn(|| server::server());

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("{:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&MATH_GROUP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(events::Handler)
        .await
        .expect("error creating client");

    if let Err(why) = client.start_shards(3).await {
        println!("Client error: {:?}", why);
    }
}
// https://discord.com/api/oauth2/authorize?client_id=897082403003187210&permissions=536870387447&scope=bot
