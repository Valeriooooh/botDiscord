use crate::*;
use serenity::{
    framework::standard::{
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions, *,
    },
    model::channel::Message,
    model::id::UserId,
    prelude::*,
};
use std::collections::HashSet;

mk_group!(General, ping, say);

cmd_ctx_msg!(ping, ctx, msg, {
    println!("user : {}", msg.author);
    // msg.reply(&ctx.http, "Pong!").await?;
    reply!(msg, ctx, "Pong!");
});

cmd_ctx_msg!(say, ctx, msg, args, {
    println!("user : {}", msg.author);
    println!("args :{:?}", args);
    say!(msg, ctx, "<3");
});

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
