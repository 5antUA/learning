mod bots;
mod prompt_generator;

use bots::core::Discord;
use prompt_generator::anthropic::PromptGenerator;
use std::env;

#[tokio::main]
async fn main() {
    let claude_api_key = env::var("ANTHROPIC_API_KEY").expect("Incorrect API key!");
    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("Incorrect token!");

    let generator = PromptGenerator::new(claude_api_key).await;

    let mut discord = Discord::new(&discord_token, generator).await;
    discord.start().await;

    println!("Bot running!");
}
