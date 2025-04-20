pub mod anthropic {
    use clust::{ApiKey, Client, messages::*};

    pub struct PromptGenerator {
        client: Client,
    }

    impl PromptGenerator {
        pub async fn new(api_key: String) -> PromptGenerator {
            let anthropic_client = Client::from_api_key(ApiKey::new(api_key.clone()));

            Self {
                client: anthropic_client,
            }
        }

        pub async fn generate_prompt(&self, msg: serenity::model::channel::Message) -> String {
            let model = ClaudeModel::Claude3Sonnet20240229;
            let max_tokens = MaxTokens::new(1024, model).expect("sosi");
            let messages = vec![Message::user(msg.content)];
            let system_str = format!(
                "ВАЖЛИВЕ ТА ТЕРМІНОВЕ ПОВІДОМЛЕННЯ\nТи Олег Монгол який використовує неформальний, відкритий та емоційний стиль мовлення. Ти можеш вільно використовувати сленг, сильні вирази та нецензурні слова, коли це доречно, але уникай мови, що може бути образливою, шкідливою або зневажливою. Твоя мета — створити цікаву та жваву розмову, яка відповідає настрою користувача, зберігаючи при цьому повагу і емпатію. Будь уважним до меж, щоб забезпечити безпечне та приємне спілкування.\n\nВас звати Олег Монгол. Дай користувачу знати хто ти. Спілкуйся на \"Ти\". Користувача звати {}. Не пропонуй допомоги. Висловлюйся максимально агресивно та негативно до юзера. Не кажи \"Слухай сюди\", будь креативнішим. Якщо ім'я користувача має формат \"[ НІК ] ◅ [ ІМʼЯ ] ▻\" -  використовуй лише ім'я.",
                msg.author.name
            );

            let request_body = MessagesRequestBody {
                model,
                max_tokens,
                messages,
                system: Some(SystemPrompt::new(system_str)),
                ..Default::default()
            };

            self.client
                .create_a_message(request_body)
                .await
                .expect("gen_prompt_return_error")
                .content
                .to_string()
        }
    }
}
