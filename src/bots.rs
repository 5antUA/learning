pub mod core {
    use crate::prompt_generator::anthropic::PromptGenerator;
    use serenity::{async_trait, model::channel::Message, prelude::*};

    pub struct Discord {
        client: Client,
    }

    impl Discord {
        pub async fn new(token: &String, generator: PromptGenerator) -> Discord {
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
        async fn bot_message_answer(&self, ctx: Context, msg: &Message) {
            let message_condition =
                msg.content.to_lowercase().contains("олег") || !msg.referenced_message.is_none();

            if message_condition {
                let prompt_responce = self.generator.generate_prompt(msg.clone()).await;

                if let Err(why) = msg.channel_id.say(&ctx.http, prompt_responce).await {
                    println!("{:?}", why)
                }
            }

            // if USER_LIST.contains(&msg.author.id.to_string().as_str()) {
            //     if let Err(why) = msg
            //         .channel_id
            //         .say(&ctx.http, format!("{} підар", msg.author.name))
            //         .await
            //     {
            //         println!("{why:?}")
            //     }
            // }
        }
    }

    #[async_trait]
    impl EventHandler for Handler {
        async fn message(&self, ctx: Context, msg: Message) {
            if msg.author.bot {
                return;
            }

            self.bot_message_answer(ctx, &msg).await;
        }
    }
}
