use crate::prompt_generator::PromptGenerator;
use teloxide::{prelude::*, sugar::request::RequestReplyExt};

pub struct Telegram {
    token: String,
    generator: PromptGenerator,
}

impl Telegram {
    pub async fn new(token: String, generator: PromptGenerator) -> Telegram {
        Self { token, generator }
    }

    pub async fn start(&self) {
        let bot = Bot::new(&self.token);

        _ = self.bot_message_answer(bot).await;
    }

    async fn bot_message_answer(&self, bot: Bot) {
        let generator = self.generator.clone();

        teloxide::repl(bot, move |bot_c: Bot, message: Message| {
            let generator = generator.clone();

            async move {
                if Self::message_condition(&message).await {
                    let ref_message = match message.reply_to_message() {
                        Some(msg) => msg.text(),
                        None => None,
                    };

                    let prompt_response = match generator
                        .generate_prompt(
                            message.text().unwrap(),
                            message.from.clone().unwrap().full_name().as_str(),
                            ref_message,
                        )
                        .await
                    {
                        Ok(responce) => responce,
                        Err(why) => {
                            println!("Error generating prompt: {:?}", why);
                            return Ok(());
                        }
                    };

                    _ = bot_c
                        .send_message(message.chat.id, prompt_response)
                        .reply_to(message.id)
                        .await;
                }

                Ok(())
            }
        })
        .await;
    }

    async fn message_condition(msg: &Message) -> bool {
        msg.from.as_ref().is_some_and(|user| !user.is_bot)
            && (msg
                .text()
                .is_some_and(|text| text.to_lowercase().contains("олег"))
                || msg.reply_to_message().is_some())
    }
}
