use super::types::TranscriptResponse;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct DeepgramClient {
    api_key: String,
    language: String,
    model: String,
}

impl DeepgramClient {
    pub fn new(api_key: String, language: String, model: String) -> Self {
        Self {
            api_key,
            language,
            model,
        }
    }

    pub async fn start_streaming(
        &self,
        mut audio_rx: mpsc::UnboundedReceiver<Vec<i16>>,
        text_tx: mpsc::UnboundedSender<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "wss://api.deepgram.com/v1/listen?model={}&encoding=linear16&sample_rate=16000&channels=1&language={}",
            self.model, self.language
        );

        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .body(())?;

        println!("[Deepgram] Connecting to {}", url);

        let (ws_stream, _) = connect_async(request).await?;
        let (mut write, mut read) = ws_stream.split();

        println!("[Deepgram] Connected");

        // Spawn task to send audio
        let send_task = tokio::spawn(async move {
            while let Some(audio_chunk) = audio_rx.recv().await {
                // Convert i16 to bytes (little-endian)
                let bytes: Vec<u8> = audio_chunk
                    .iter()
                    .flat_map(|&sample| sample.to_le_bytes())
                    .collect();

                if write.send(Message::Binary(bytes)).await.is_err() {
                    break;
                }
            }
        });

        // Receive transcripts
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(response) = serde_json::from_str::<TranscriptResponse>(&text) {
                        if let Some(transcript) = response.get_text() {
                            println!("[Deepgram] Transcript: {}", transcript);
                            let _ = text_tx.send(transcript.to_string());
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("[Deepgram] Connection closed");
                    break;
                }
                Err(e) => {
                    eprintln!("[Deepgram] Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        send_task.abort();
        Ok(())
    }
}
