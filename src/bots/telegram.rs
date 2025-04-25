use crate::prompt_generator::PromptGenerator;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

pub struct Telegram {}

impl Telegram {
    pub async fn new(token: String, generator: PromptGenerator) -> Telegram {
        let bot = Bot::new(token);

        Dispatcher::builder(
            bot,
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(
                    dptree::case![State::ReceiveLocation { full_name, age }]
                        .endpoint(send_prompt_responce),
                ),
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

        Self {}
    }

    pub async fn start(&self) {}

    async fn send_prompt_responce() -> HandlerResult {
        Ok(())
    }
}
