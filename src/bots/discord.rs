use crate::prompt_generator::PromptGenerator;
use serenity::{async_trait, model::channel::Message, prelude::*};

pub struct Discord {
    client: Client,
}

impl Discord {
    pub async fn new(token: String, generator: PromptGenerator) -> Discord {
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let client = Client::builder(token, intents)
            .event_handler(Handler { generator })
            .await
            .expect("Err creating client");

        Self { client }
    }

    pub async fn start(&mut self) {
        if let Err(why) = self.client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}

struct Handler {
    generator: PromptGenerator,
}

impl Handler {
    async fn bot_message_answer(&self, ctx: &Context, msg: &Message) {
        let author_nick = if let Some(guild_id) = msg.guild_id {
            match msg.author.nick_in(&ctx.http, guild_id).await {
                Some(nick) => nick,
                None => {
                    println!("Nickname not found, falling back to username!");
                    msg.author.name.clone()
                }
            }
        } else {
            println!("No guild ID, falling back to username! sho");
            msg.author.name.clone()
        };

        if self.message_condition(&msg, &ctx).await {
            let ref_message: Option<&str> = if let Some(ref_msg) = &msg.referenced_message {
                Some(ref_msg.content.as_str())
            } else {
                None
            };

            let prompt_responce = match self
                .generator
                .generate_prompt(&msg.content, &author_nick, ref_message)
                .await
            {
                Ok(prompt_responce) => prompt_responce,
                Err(why) => {
                    println!("Error generating prompt: {}", why);
                    return;
                }
            };

            if let Err(why) = msg.reply(&ctx.http, prompt_responce).await {
                println!("{:?}", why)
            }
        }
    }

    async fn message_condition(&self, msg: &Message, ctx: &Context) -> bool {
        let current_bot_user = match ctx.http.get_current_user().await {
            Ok(value) => value,
            Err(why) => {
                println!("Message condition error: {:?}", why);
                return false;
            }
        };

        msg.content.to_lowercase().contains("олег")
            || (msg
                .referenced_message
                .as_ref()
                .is_some_and(|ref_msg| ref_msg.author.id.eq(&current_bot_user.id)))
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        self.bot_message_answer(&ctx, &msg).await;
    }
}
