use crate::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use songbird::SerenityInit;
use url::Url;

mk_group!(Music, deafen, join, play);

cmd_ctx_msg!(deafen, ctx, msg, {
    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;

    let manager = songbird::get(ctx)
        .await
        .expect("Placed initialization")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(a) => a,
        None => {
            reply!(msg, ctx, "not in a voice channel");
            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        say!(msg, ctx, "Already deafened");
    } else {
        if let Err(e) = handler.deafen(true).await {
            say!(msg, ctx, format!("Failed {:?}", e));
        }
        say!(msg, ctx, "Deafened");
    }
});

cmd_ctx_msg!(join, ctx, msg, {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice| voice.channel_id);

    let connect_to = match channel_id {
        Some(c) => c,
        None => {
            reply!(msg, ctx, "Not in a vc");
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Placed initialization")
        .clone();
    let _ = manager.join(guild.id, connect_to).await;
});

cmd_ctx_msg!(play, ctx, msg, args, {
    let url = args.single::<String>()?;
    if !url.starts_with("http") {
        say!(msg, ctx, "must provide a url");
        return Ok(());
    }

    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;

    let manager = songbird::get(ctx)
        .await
        .expect("Placed initialization")
        .clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let source = songbird::ytdl(&url).await?;

        handler.play_source(source);
        say!(msg, ctx, format!("Playing song -> {}", url));
    } else {
        say!(msg, ctx, "Not in a voice channel");
    }
});
