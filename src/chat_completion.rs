use reqwest::Client;
use serde_json::json;
use anyhow::{Result, Context};

pub struct ChatCompletion {
    client: Client,
    api_token: String,
}

impl ChatCompletion {
    pub fn new(api_token: String) -> Self {
        ChatCompletion {
            client: Client::new(),
            api_token,
        }
    }

    pub async fn translate_using_chat_completion(
        &self,
        input_text: String,
        target_language: &str,
    ) -> Result<String> {
        let api_url = "https://api.groq.com/openai/v1/chat/completions";



        let request_body = json!({
            "model": "llama-3.1-8b-instant",
            "messages": [
                {
                    "role": "system",
                    "content": format!("You are a professional translator. Translate the following text to {}. Provide only the translated text, with no additional commentary or explanations.", target_language),
                },
                {
                    "role": "user",
                    "content": input_text,
                }
            ],
            "temperature": 1,
            "max_tokens": 1024,
            "top_p": 1,
            "stream": false,
            "stop": null
        });

        let res = self.client
            .post(api_url)
            .bearer_auth(&self.api_token)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send translation request")?;

        let body = res.text().await.context("Failed to read response body")?;
        let json: serde_json::Value = serde_json::from_str(&body).context("Failed to parse response body as JSON")?;
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string())
    }
}