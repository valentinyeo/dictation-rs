mod audio;
mod config;
mod deepgram;
mod keyboard;
mod state;
mod tray;

use audio::{capture::AudioCapture, ducking::AudioDucker, vad::{VoiceActivityDetector, VadEvent}};
use config::Config;
use deepgram::client::DeepgramClient;
use keyboard::simulator::KeyboardSimulator;
use state::{AppState, StateManager};
use tray::icon::TrayManager;

use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Dictation App Starting ===");

    // Load configuration
    let config = match Config::load_or_create() {
        Ok(cfg) => {
            println!("[Config] Loaded from: {:?}", Config::get_config_path()?);
            cfg
        }
        Err(e) => {
            eprintln!("[Config] Error: {}", e);
            eprintln!("[Config] Please edit config.toml and add your Deepgram API key");
            std::thread::sleep(std::time::Duration::from_secs(5));
            return Err(e);
        }
    };

    let config = Arc::new(RwLock::new(config));
    let state_manager = Arc::new(StateManager::new());

    // Create channels
    let (audio_tx, audio_rx) = mpsc::unbounded_channel::<Vec<i16>>();
    let (text_tx, text_rx) = mpsc::unbounded_channel::<String>();

    // Start audio capture
    println!("[Main] Starting audio capture...");
    let _audio_capture = AudioCapture::start(audio_tx)?;

    // Start keyboard simulator
    println!("[Main] Starting keyboard simulator...");
    tokio::spawn(async move {
        KeyboardSimulator::start(text_rx).await;
    });

    // Start VAD processor and Deepgram client manager
    let state_manager_clone = state_manager.clone();
    let config_clone = config.clone();
    tokio::spawn(async move {
        vad_and_deepgram_manager(
            audio_rx,
            text_tx,
            state_manager_clone,
            config_clone,
        )
        .await;
    });

    // Create system tray
    println!("[Main] Creating system tray...");
    let mut tray_manager = TrayManager::new(state_manager.clone(), config.clone())?;

    println!("[Main] Dictation app ready!");
    println!("[Main] - Left-click tray icon to pause/resume");
    println!("[Main] - Right-click for menu options");

    // Main event loop
    loop {
        tray_manager.handle_events().await;

        let current_state = state_manager.get().await;
        tray_manager.update_icon(current_state).await;

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

async fn vad_and_deepgram_manager(
    mut audio_rx: mpsc::UnboundedReceiver<Vec<i16>>,
    text_tx: mpsc::UnboundedSender<String>,
    state_manager: Arc<StateManager>,
    config: Arc<RwLock<Config>>,
) {
    let cfg = config.read().await;
    let mut vad = VoiceActivityDetector::new(
        cfg.vad.energy_threshold,
        cfg.audio.silence_threshold_ms,
    );
    let mut ducker = AudioDucker::new(cfg.audio.duck_volume);
    drop(cfg);

    let mut deepgram_handle: Option<tokio::task::JoinHandle<()>> = None;
    let mut deepgram_tx: Option<mpsc::UnboundedSender<Vec<i16>>> = None;

    while let Some(audio_chunk) = audio_rx.recv().await {
        let current_state = state_manager.get().await;

        // Skip processing if paused
        if matches!(current_state, AppState::Paused | AppState::MicConflict) {
            continue;
        }

        let vad_event = vad.process(&audio_chunk);

        match vad_event {
            VadEvent::SpeechStarted => {
                println!("[VAD] Speech started");
                state_manager.set(AppState::Speaking).await;

                // Duck audio
                let _ = ducker.duck();

                // Start Deepgram connection
                let cfg = config.read().await;
                let client = DeepgramClient::new(
                    cfg.deepgram.api_key.clone(),
                    cfg.deepgram.language.clone(),
                    cfg.deepgram.model.clone(),
                );
                drop(cfg);

                let (dg_audio_tx, dg_audio_rx) = mpsc::unbounded_channel();
                let text_tx_clone = text_tx.clone();

                deepgram_handle = Some(tokio::spawn(async move {
                    if let Err(e) = client.start_streaming(dg_audio_rx, text_tx_clone).await {
                        eprintln!("[Deepgram] Error: {}", e);
                    }
                }));

                // Send this chunk
                let _ = dg_audio_tx.send(audio_chunk);
                deepgram_tx = Some(dg_audio_tx);
            }
            VadEvent::Speaking => {
                if matches!(current_state, AppState::Speaking) {
                    // Forward audio to Deepgram
                    if let Some(ref tx) = deepgram_tx {
                        let _ = tx.send(audio_chunk);
                    }
                }
            }
            VadEvent::SilenceDetected => {
                println!("[VAD] Silence detected");
                state_manager.set(AppState::AutoPaused).await;

                // Restore audio
                let _ = ducker.restore();

                // Close Deepgram connection
                deepgram_tx = None;
                if let Some(handle) = deepgram_handle.take() {
                    handle.abort();
                }

                vad.reset();
            }
            VadEvent::Silence => {
                // Continue listening
            }
        }
    }
}
