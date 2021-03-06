pub mod math;
pub mod music;
pub mod reply;

#[macro_export]
macro_rules! mk_group {
    ($name: ident , $($coms: ident),*) => {
        #[group]
        #[commands($($coms,)*)]
        struct $name;
    };
}

#[macro_export]
macro_rules! cmd_ctx_msg {
    ($name: ident,$ctx: ident, $msg: ident , $args: ident , $($line:stmt)*) => {
        #[command]
        #[only_in(guilds)]
        async fn $name($ctx: &Context, $msg: &Message, mut $args: Args ) -> CommandResult {
            $($line)*;
            Ok(())
        }
    };
    ($name: ident,$ctx: ident, $msg: ident, $($line:stmt)*) => {
        #[command]
        #[only_in(guilds)]
        async fn $name($ctx: &Context, $msg: &Message ) -> CommandResult {
            $($line)*;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! reply {
    ($msg: ident, $ctx: ident, $ex: expr ) => {
        $msg.reply(&$ctx.http, $ex).await?;
    };
}

#[macro_export]
macro_rules! say {
    ($msg: ident, $ctx: ident, $ex: expr) => {
        $msg.channel_id.say(&$ctx.http, $ex).await?;
    };
}
