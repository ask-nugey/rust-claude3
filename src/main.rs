use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct Content {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ResponseBody {
    content: Vec<Content>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    const MODEL: &str = "claude-3-opus-20240229";
    const MAX_TOKENS: u32 = 1024;
    const ROLE_USER: &str = "user";
    const ANTHROPIC_VERSION: &str = "2023-06-01";

    let api_key = env::var("CLAUDE_API_KEY").expect("CLAUDE_API_KEY not set");
    let request_url = "https://api.anthropic.com/v1/messages";

    let content = "ヌギーって知ってる？";

    let request_body = RequestBody {
        model: MODEL.to_string(),
        max_tokens: MAX_TOKENS,
        messages: vec![Message {
            role: ROLE_USER.to_string(),
            content: content.to_string(),
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(request_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_body: ResponseBody = response.json().await?;
    if let Some(content) = response_body.content.get(0) {
        println!("Response Text: {}", content.text);
    } else {
        println!("No content found in the response.");
    }

    Ok(())
}
