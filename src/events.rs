use serenity::{model::prelude::*, prelude::*};

pub struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content.contains("@everyone") {
    //         println!("user @everyone",);
    //         if msg.author.has_role(&ctx.http){

    //         }
    //     }

    //     if msg.content.contains("!msg") {
    //         let cn = msg.content.clone();
    //         let cn = cn.trim_start_matches("!msg");
    //         let _ = msg.author.dm(&context.http, |m| m.content(cn)).await;
    //     }
    //     if msg.content.contains("!sum") {
    //         let cn = msg.content.clone();
    //         let cn = cn.trim_start_matches("!sum");
    //         println!("{}", &cn);
    //         let cn = cn.trim();
    //         println!("{}", &cn);
    //         let a = cn.to_string().remove(0);
    //         println!("{}", a);
    //         let b = cn.trim_start_matches(a);
    //         // println!("{}", b);
    //         // let a = a;
    //         // let b = b;
    //         // println!("{}", a + b);
    //         // let _ = msg.channel_id.say(&context.http, a + b).await;
    //     }
    // }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("bot : {} ready", ready.user.name);
    }
}
