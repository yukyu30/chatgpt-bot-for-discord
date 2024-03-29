use reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<RequestMessage<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RequestMessage<'a> {
    pub role: &'a str,
    pub content: String,
}

pub async fn get_gpt_response(
    messages: Vec<RequestMessage<'_>>,
    gpt_token: &str,
    client: &reqwest::Client,
) -> Result<String, Error> {
    const URL: &str = "https://api.openai.com/v1/chat/completions";
    const GPT_MODEL: &str = "gpt-3.5-turbo";

    let request_body = ChatRequest {
        model: GPT_MODEL,
        messages,
    };

    let response = client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", gpt_token))
        .json(&request_body)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    return Ok(response.choices[0].message.content.clone());
}
