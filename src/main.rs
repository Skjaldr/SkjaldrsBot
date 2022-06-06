use std::io;
mod window_handler;
mod config;
mod discord;

#[tokio::main]
async fn main() {
    let mut wait = String::new();
    config::run_program();

    println!("To start the discord bot, press enter......");
    io::stdin().read_line(&mut wait).expect("FAILURE!!");
    discord::run().await;
}