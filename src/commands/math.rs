use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(sum, sub, mul, div)]
#[prefixes("m", "math")]
struct Math;

#[macro_export]
macro_rules! math_fn {
    ($name: ident,$ctx: ident, $msg: ident, $s:tt, $ss:tt, $args: ident ) => {
        #[command]
        async fn $name($ctx: &Context, $msg: &Message, mut $args: Args) -> CommandResult {
            let a = $args.single::<f32>()?;
            let b = $args.single::<f32>()?;
            $msg.reply(&$ctx.http,format!("{} {} {} = {}", a, $ss, b, a $s b))
                .await?;

            Ok(())
        }
    };
}

math_fn!(sum,ctx,msg,+,"+",args);

math_fn!(sub,ctx,msg,-,"-",args);
math_fn!(mul,ctx,msg,*,"*",args);
math_fn!(div,ctx,msg,/,":",args);
