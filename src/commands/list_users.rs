use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("users")
        .description("Lista los usuarios del servidor")
}

pub async fn run(_ctx: &Context, _options: &[CommandDataOption]) -> String {
    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );

    let members = guild_id.members(&_ctx.http, None, None).await;

    match members {
        Ok(members) => {
            let mut msg = "**Lista de Usuarios:**\n".to_string();
            for member in members {
                let user = member.user;

                msg.push_str(
                    format!("- {}, ID: {}\n", user.tag(), user.id)
                        .to_owned()
                        .as_str(),
                );
            }
            msg
        }
        Err(e) => e.to_string(),
    }
}
