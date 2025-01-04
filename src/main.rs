mod audio_recorder;
mod speech_to_text;
mod chat_completion;

use audio_recorder::AudioRecorder;
use speech_to_text::SpeechToText;
use chat_completion::ChatCompletion;
use std::env;
use std::io;
use dotenv::dotenv;
use anyhow::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_token = env::var("API_TOKEN").unwrap_or_else(|_| "hardcoded_apikey4_debug".to_string());
    let audio_file_path = "tmp_recordedAudio.wav";

    // record audio
    let mut recorder = AudioRecorder::new();
    recorder.start_recording(audio_file_path);
    println!("Recording... Press Enter to stop.");
    io::stdin().read_line(&mut String::new()).unwrap();
    recorder.stop_recording();

    // transcribe audio
    let stt = SpeechToText::new(api_token.clone());
    let transcription = match stt.transcribe_audio(audio_file_path).await {
        Ok(text) => {
            println!("Transcription: {}", text);
            text
        }
        Err(e) => {
            eprintln!("Failed to transcribe audio: {}", e);
            return Ok(());
        }
    };

    // translate text
    let chat_completion = ChatCompletion::new(api_token);
    let target_language = "French"; // Example: Translate to French
    match chat_completion.translate_using_chat_completion(transcription, target_language).await {
        Ok(text) => {
            println!("Translated Text: {}", text);
        }
        Err(e) => {
            eprintln!("Failed to translate text: {}", e);
            return Ok(());
        }
    };

    Ok(())
}