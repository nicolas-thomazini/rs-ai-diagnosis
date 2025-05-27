use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MistralRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct MistralResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

pub async fn mistral_chat(question: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("MISTRAL_API_KEY")?;
    let url = "https://api.mistral.ai/v1/chat/completions";

    let client = Client::new();

    let body = MistralRequest {
        model: "mistral-small".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: question.to_string(),
        }],
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let answer: MistralResponse = res.json().await?;
    let content = &answer.choices[0].message.content;

    Ok(content.clone())
}