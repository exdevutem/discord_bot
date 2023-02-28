mod commands;
mod handler;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    // Carga variables del ambiente desde el archivo .env
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Missing Discord Token");

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_PRESENCES;

    let mut client = Client::builder(token, intents)
        .event_handler(handler::Handler) // Revisar handler.rs
        .await
        .expect("Error creating Client");

    if let Err(why) = client.start().await {
        println!("An error ocurred while running the client: {why:?}");
    }
}
