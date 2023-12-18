//mod window_handler;
use crate::window_handler;
use serenity::framework::standard::{Args, StandardFramework, CommandResult, macros::{command, group}};
use serenity::model::{channel::Message, gateway::Ready, id::ChannelId};
use serenity::prelude::*;
use serenity::*;
use std::env;

//define group and commands
#[group]
#[commands(about, ping, sum, inv)]
struct General;
struct Handler;

#[async_trait()]
impl EventHandler for Handler {

    //ready message displays in terminal that bot is online and active, the lets users in chat know bot is online and ready to accept commands.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let channel_id = ChannelId("Enter discord token here");
        channel_id.say(&ctx.http, "Summon bot is now online!  /sum for summons, /inv for guild tags").await.unwrap();
    }
}

//Simple about command
#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "/about command is a work in progress!").await?;
    Ok(())
}

// Ping command
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "PONG!").await?;
    Ok(())
}

#[command]
async fn inv(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let pname = args.rest();
    if !pname.is_empty() {
        window_handler::guild_inv(pname)
        .await;
        msg.channel_id.say(&ctx.http, &format!("Sending guild inv to {}", &pname))
        .await?;
    } else {
        msg.channel_id.say(&ctx.http, "Command use: /inv playername, example /inv fjarska").await?;
    };
    Ok(())
}

// Summon player command that summons players to the character connected to the bot.
#[command]
async fn sum(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    //if arg is not empty, we want to take the argument after the /sum command and summon the player
    //else send command prompt to user with instructions
    if !args.is_empty() {
        //execute summon_player function from disc_bots.rs
        window_handler::summon_player(args.rest())
        .await;
        msg.channel_id.say(&ctx.http, &format!("Summoning player: {}.  Have a safe Journey!", &args.rest()))
        .await?;
    } else {
        msg.channel_id.say(&ctx.http, "Command use: /sum playername.  Example: /sum fjarska").await?;
    };
    Ok(())
}

//main fn
pub async fn run() {
    println!("Am I running at the start?");
    // declare framework
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    //set token to env var, output if token not found
    let token = env::var("DISCORD_TOKEN").expect("Token not found.");

    //declare intents/permissions for the output/commands to execute in the discord chat
    let intents = 
        //GatewayIntents::privileged() | Not needed at the moment
        GatewayIntents::non_privileged() | 
        GatewayIntents::MESSAGE_CONTENT;

    // declare client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Expected a token in the environment.");

    // start client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    println!("Am I running?");
}

//main fn
