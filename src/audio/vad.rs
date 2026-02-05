use std::time::{Duration, Instant};

pub struct VoiceActivityDetector {
    energy_threshold: f32,
    silence_threshold: Duration,
    last_speech_time: Option<Instant>,
    is_speaking: bool,
}

impl VoiceActivityDetector {
    pub fn new(energy_threshold: f32, silence_threshold_ms: u64) -> Self {
        Self {
            energy_threshold,
            silence_threshold: Duration::from_millis(silence_threshold_ms),
            last_speech_time: None,
            is_speaking: false,
        }
    }

    pub fn process(&mut self, audio_chunk: &[i16]) -> VadEvent {
        let is_speech = self.detect_speech(audio_chunk);

        if is_speech {
            self.last_speech_time = Some(Instant::now());

            if !self.is_speaking {
                self.is_speaking = true;
                return VadEvent::SpeechStarted;
            }
            return VadEvent::Speaking;
        } else {
            // Silence detected
            if let Some(last_speech) = self.last_speech_time {
                let silence_duration = Instant::now().duration_since(last_speech);

                if self.is_speaking && silence_duration > self.silence_threshold {
                    self.is_speaking = false;
                    return VadEvent::SilenceDetected;
                }
            }

            if self.is_speaking {
                return VadEvent::Speaking;
            }
            return VadEvent::Silence;
        }
    }

    fn detect_speech(&self, audio_chunk: &[i16]) -> bool {
        if audio_chunk.is_empty() {
            return false;
        }

        // Calculate RMS energy
        let rms = (audio_chunk
            .iter()
            .map(|&s| {
                let normalized = s as f32 / i16::MAX as f32;
                normalized * normalized
            })
            .sum::<f32>()
            / audio_chunk.len() as f32)
            .sqrt();

        rms > self.energy_threshold
    }

    pub fn reset(&mut self) {
        self.is_speaking = false;
        self.last_speech_time = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VadEvent {
    SpeechStarted,
    Speaking,
    SilenceDetected,
    Silence,
}
