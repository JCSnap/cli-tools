use dotenv::dotenv;
use std::env;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make sure that you have a .env file in the root directory of the project, with your OPENAI_API_KEY
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let client = Client::new(api_key);

    let req = ChatCompletionRequest::new(
        chat_completion::GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("Hello OpenAI!"),
            name: None,
            function_call: None,
        }],
        );
    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);
    Ok(())
}

