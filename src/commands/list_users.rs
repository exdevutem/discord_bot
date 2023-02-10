use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("users")
        .description("Lista los usuarios del servidor")
}

pub async fn run(
    guild_id: Option<GuildId>,
    _ctx: &Context,
    _options: &[CommandDataOption],
) -> String {
    match guild_id {
        Some(id) => {
            let members = id.members(&_ctx.http, None, None).await;

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
        None => String::from("No pude recuperar la id de esta guild! No puedo ver los miembros :("),
    }
}
