use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TranscriptResponse {
    pub channel: Channel,
    pub is_final: bool,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub alternatives: Vec<Alternative>,
}

#[derive(Debug, Deserialize)]
pub struct Alternative {
    pub transcript: String,
}

impl TranscriptResponse {
    pub fn get_text(&self) -> Option<&str> {
        if !self.is_final {
            return None;
        }

        self.channel
            .alternatives
            .first()
            .map(|alt| alt.transcript.as_str())
            .filter(|s| !s.is_empty())
    }
}
