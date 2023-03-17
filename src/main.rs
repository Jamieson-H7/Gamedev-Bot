use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::{io::BufReader, sync::atomic::Ordering};

use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    model::{application::interaction::Interaction, prelude::*},
    prelude::*,
    Client,
};

#[derive(Deserialize)]
struct Handler {
    token: String,
    #[serde(rename = "guildId")]
    guild_id: GuildId,
    #[serde(rename = "clientId")]
    client_id: String,
    #[serde(rename = "readRulesRoleId")]
    read_rules_role_id: RoleId,
    #[serde(rename = "rulesMessageId")]
    rules_message_id: MessageId,
    #[serde(rename = "gamersRoleId")]
    gamers_role_id: RoleId,
    #[serde(rename = "masterKeyB4A")]
    master_key_b4a: String,
    #[serde(default, skip)]
    throwable: AtomicBool,
}

#[derive(Serialize, Deserialize)]
struct Users {
    #[serde(rename = "ballUsers")]
    ball_users: Vec<BallUser>,
}

#[derive(Serialize, Deserialize)]
struct BallUser {
    #[serde(rename = "userID")]
    user_id: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, mut member: Member) {
        println!("New Person Joined!");
        member
            .add_role(&ctx.http, self.read_rules_role_id)
            .await
            .unwrap();
        println!("Assigned Role");
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if reaction.emoji.unicode_eq("👍") && reaction.message_id == self.rules_message_id {
            if let Some(member) = reaction.member {
                if let Some(ref user) = member.user {
                    ctx.http
                        .remove_member_role(
                            member.guild_id.unwrap().0,
                            user.id.0,
                            self.read_rules_role_id.0,
                            None,
                        )
                        .await
                        .unwrap();
                    ctx.http
                        .add_member_role(
                            member.guild_id.unwrap().0,
                            user.id.0,
                            self.gamers_role_id.0,
                            None,
                        )
                        .await
                        .unwrap();
                    if user.direct_message(ctx.http.clone(), |message| {
                        message.content("You now have access to the CGC Discord! Thanks for deciding to join the community!")
                    }).await.is_err() {
                        ChannelId(101977672400477802).send_message(&ctx.http, |message| {
                            message.content(format!("Welcome {}!", user.name))
                        }).await.unwrap();
                    }
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            match cmd.data.name.as_str() {
                "ping" => cmd
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(interaction::InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("Pong!"))
                    })
                    .await
                    .unwrap(),
                "ball" => {
                    if !self.throwable.load(Ordering::Acquire) {
                        self.throwable.store(
                            {
                                cmd.create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(
                                            interaction::InteractionResponseType::ChannelMessageWithSource,
                                        )
                                        .interaction_response_data(|message| {
                                            message.content("Now playing ball!")
                                        })
                                })
                                .await
                                .unwrap();

                                let mut message = cmd
                                    .channel_id
                                    .send_message(&ctx.http, |message| {
                                        message.content("o----\n-----\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n-o---\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n-----\n--o--")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n---o-\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("----o\n-----\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message.react(ctx.http.clone(), "🖐️'".parse::<ReactionType>().unwrap()).await.unwrap();
                                cmd.create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(
                                            interaction::InteractionResponseType::ChannelMessageWithSource,
                                        )
                                        .interaction_response_data(|message| {
                                            message.content("Nice catch! Now throw it back with /throw")
                                        })
                                })
                                .await
                                .unwrap();

                                true
                            },
                            Ordering::Release,
                        )
                    } else {
                        cmd.create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(
                                    interaction::InteractionResponseType::ChannelMessageWithSource,
                                )
                                .interaction_response_data(|message| {
                                    message.content("I don't have ball back yet!")
                                })
                        })
                        .await
                        .unwrap();
                    }
                }
                "throw" => {
                    if self.throwable.load(Ordering::Acquire) {
                        self.throwable.store(
                            {
                                cmd.create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(
                                            interaction::InteractionResponseType::ChannelMessageWithSource,
                                        )
                                        .interaction_response_data(|message| {
                                            message.content("*You passed it back*")
                                        })
                                })
                                .await
                                .unwrap();

                                let mut message = cmd
                                    .channel_id
                                    .send_message(&ctx.http, |message| {
                                        message.content("----o\n-----\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n---o-\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n-----\n--o--")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("-----\n-o---\n-----'")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message
                                    .edit(ctx.http.clone(), |message| {
                                        message.content("o----\n-----\n-----")
                                    })
                                    .await
                                    .unwrap();

                                tokio::time::sleep(Duration::from_millis(500)).await;

                                message.react(ctx.http.clone(), "🖐️'".parse::<ReactionType>().unwrap()).await.unwrap();

                                cmd.create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(
                                            interaction::InteractionResponseType::ChannelMessageWithSource,
                                        )
                                        .interaction_response_data(|message| {
                                            message.content("Good throw! I'll bounce it back when you say /ball")
                                        })
                                })
                                .await
                                .unwrap();

                                true
                            },
                            Ordering::Release,
                        )
                    } else {
                        cmd.create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(
                                    interaction::InteractionResponseType::ChannelMessageWithSource,
                                )
                                .interaction_response_data(|message| {
                                    message.content("You haven't been thrown the ball yet!")
                                })
                        })
                        .await
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        self.guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        command.name("ping").description("Replies with pong!")
                    })
                    .create_application_command(|command| {
                        command.name("ball").description("Bounces a ball to you!")
                    })
                    .create_application_command(|command| {
                        command.name("throw").description("Throws the ball back!")
                    })
            })
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let config: Handler =
        serde_json::from_reader(BufReader::new(std::fs::File::open("config.json").unwrap()))
            .unwrap();
    let mut client = Client::builder(
        config.token.clone(),
        GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS
            | GatewayIntents::GUILD_MEMBERS,
    )
    .event_handler(config)
    .await
    .unwrap();

    client.start().await.unwrap();
}
