use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Guild;
use serenity::prelude::*;

use crate::commands;

pub struct Handler;

impl Handler {
    /// Setea los comandos de la app / bot en una Guild en particular.
    /// Se utiliza al iniciar para que todos los servidores tengan acceso a los commandos
    /// o al unirse a una nueva guild.
    async fn set_app_commands(_guild_id: &GuildId, _ctx: &Context) {
        GuildId::set_application_commands(_guild_id, &_ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::list_users::register(command))
        })
        .await
        .map_err(|err| println!("Got an error while setting app commands! {err}"))
        .ok();
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Booting up...");
        for guild in ready.guilds.iter() {
            Handler::set_app_commands(&guild.id, &ctx).await;
        }

        println!("All set up.");
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = _interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "users" => {
                    commands::list_users::run(command.guild_id, &_ctx, &command.data.options).await
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&_ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn guild_create(&self, _ctx: Context, _guild: Guild, _is_new: bool) {
        println!("Joined a server!");

        if _is_new {
            println!("It is a new server too!");
        }
    }
}
