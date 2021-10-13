use crate::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

mk_group!(General, ping, say);

cmd_ctx_msg!(ping, ctx, msg, {
    println!("user : {}", msg.author);
    // msg.reply(&ctx.http, "Pong!").await?;
    reply!(msg, ctx, "Pong!");
});

cmd_ctx_msg!(say, ctx, msg, args, {
    println!("user : {}", msg.author);
    println!("args :{:?}", args);
    msg.channel_id.say(&ctx.http, "<3").await?;
});
