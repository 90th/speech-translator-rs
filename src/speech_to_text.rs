use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::fs;
use anyhow::{Result, Context};

pub struct SpeechToText {
    client: Client,
    api_token: String,
}

impl SpeechToText {
    pub fn new(api_token: String) -> Self {
        SpeechToText {
            client: Client::new(),
            api_token,
        }
    }

    pub async fn transcribe_audio(&self, audio_file_path: &str) -> Result<String> {
        let api_url = "https://api.groq.com/openai/v1/audio/transcriptions";


        let form = Form::new()
            .text("model", "whisper-large-v3-turbo")
            .text("response_format", "verbose_json")
            .part("file", Part::bytes(fs::read(audio_file_path)?).file_name("audio.wav"));

        let res = self.client
            .post(api_url)
            .bearer_auth(&self.api_token)
            .multipart(form)
            .send()
            .await
            .context("Failed to send transcription request")?;


        let body = res.text().await.context("Failed to read response body")?;
        let json: serde_json::Value = serde_json::from_str(&body).context("Failed to parse response body as JSON")?;

        // Delete the audio file after sending the request
        fs::remove_file(audio_file_path).context("Failed to delete audio file")?;

        Ok(json["text"].as_str().unwrap_or_default().to_string())
    }
}