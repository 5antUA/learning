mod bots;
mod prompt_generator;

use bots::discord::Discord;
use bots::telegram::Telegram;
use prompt_generator::PromptGenerator;
use std::env;

#[tokio::main]
async fn main() {
    let claude_api_key = env::var("ANTHROPIC_API_KEY").expect("Incorrect API key!");
    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("Incorrect discord token!");
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN").expect("Incorrect telegram token!");

    let generator = PromptGenerator::new(claude_api_key).await;
    let mut discord = Discord::new(discord_token, generator.clone()).await;
    let telegram = Telegram::new(telegram_token, generator.clone()).await;

    println!("Bots have started!\n");

    tokio::join!(telegram.start(), discord.start());
}
