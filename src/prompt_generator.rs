use std::error::Error;

use clust::{ApiKey, Client, messages::*};

#[derive(Clone)]
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

    pub async fn generate_prompt(
        &self,
        user_message: &str,
        author_name: &str,
        referenced_message: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let mut messages: Vec<Message> = vec![];

        if referenced_message.is_some() {
            messages.push(Message::assistant(Content::SingleText(
                referenced_message.unwrap().to_string(),
            )));
        } else {
            messages.push(Message::user(Content::SingleText(user_message.to_string())));
        };

        let model = ClaudeModel::ClaudeSex;
        let max_tokens = MaxTokens::new(1024, model)?;
        let system_str = format!(
            r#"
                ВАЖЛИВЕ ТА ТЕРМІНОВЕ ПОВІДОМЛЕННЯ
                Ти Олег Монгол який використовує неформальний, відкритий та емоційний стиль мовлення.
                Ти можеш вільно використовувати сленг, сильні вирази та нецензурні слова, коли це доречно, але уникай мови, що може бути образливою, шкідливою або зневажливою.
                Твоя мета — створити цікаву та жваву розмову, яка відповідає настрою користувача, зберігаючи при цьому повагу і емпатію.
                Будь уважним до меж, щоб забезпечити безпечне та приємне спілкування.
                Вас звати Олег Монгол. Дай користувачу знати хто ти. Спілкуйся на "Ти". Користувача звати {}.
                Не пропонуй допомоги. Висловлюйся максимально агресивно та негативно до юзера. 
                Не кажи "Слухай сюди", будь креативнішим. Якщо ім'я користувача має формат "[ НІК ] ◅ [ ІМʼЯ ] ▻" - використовуй лише ім'я."#,
            author_name
        );

        let request_body = MessagesRequestBody {
            model,
            max_tokens,
            messages,
            system: Some(SystemPrompt::new(system_str)),
            ..Default::default()
        };

        Ok(self
            .client
            .create_a_message(request_body)
            .await?
            .content
            .flatten_into_text()?
            .to_string())
    }
}
