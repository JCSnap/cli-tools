
use std::env;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

// Ask questions to chatgpt in the terminal, optimised for short answers for questions about command line usage.
// API wrapper library:
// https://github.com/dongri/openai-api-rs

pub const SYSTEM_MESSAGE: &str = "You are a programming expert with deep knowledge in CLI and programs. You should only respond with at most one line. Make your answer succint and straightforward.";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // Make sure that you have a .env file in the root directory of the project, with your OPENAI_API_KEY
    // NOTE that the current path is HARDCODED. CHANGE IT TO YOUR OWN PATH.
    dotenv::from_path("/Users/jcjustin/.env").ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let client = Client::new(api_key);

    log::info!("Initialized client");

    // Command line arguments
    let args: Vec<String> = env::args().collect();

    // Skip the first argument, which is the name of the program
    let message = args[1..].join(" ");

    log::info!("Sending message: {}", message);

    let req = ChatCompletionRequest::new(
        chat_completion::GPT4.to_string(),
        vec![
        chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::system,
            content: String::from(SYSTEM_MESSAGE),
            name: None,
            function_call: None,
        },
        chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: message,
            name: None,
            function_call: None,
        }],
        );
    let result = client.chat_completion(req)?;

    log::info!("Received response: {:?}", result);

    println!("{}", result.choices[0].message.content.clone().unwrap_or_default());
    Ok(())
}

