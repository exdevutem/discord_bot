mod commands;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    // Carga variables del ambiente desde el archivo .env
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Missing Discord Token");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(commands::Handler) // Revisar commands/mod.rs
        .await
        .expect("Error creating Client");

    if let Err(why) = client.start().await {
        println!("An error ocurred while running the client: {why:?}");
    }
}
