use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AppState {
    Active,      // Listening, VAD running (grey icon - no speech yet)
    Paused,      // User paused, nothing running (red icon)
    AutoPaused,  // VAD silence-paused, ready to resume (grey icon)
    Speaking,    // Actively transcribing (blue icon)
    MicConflict, // Other app using microphone (red icon)
}

impl AppState {
    pub fn icon_color(&self) -> &'static str {
        match self {
            AppState::Active | AppState::AutoPaused => "grey",
            AppState::Speaking => "blue",
            AppState::Paused | AppState::MicConflict => "red",
        }
    }

    pub fn is_transcribing(&self) -> bool {
        matches!(self, AppState::Speaking)
    }

    pub fn is_listening(&self) -> bool {
        matches!(self, AppState::Active | AppState::Speaking)
    }
}

#[derive(Clone)]
pub struct StateManager {
    state: Arc<RwLock<AppState>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::Active)),
        }
    }

    pub async fn get(&self) -> AppState {
        *self.state.read().await
    }

    pub async fn set(&self, new_state: AppState) {
        let mut state = self.state.write().await;
        if *state != new_state {
            println!("[State] {:?} -> {:?}", *state, new_state);
            *state = new_state;
        }
    }

    pub async fn toggle_pause(&self) -> AppState {
        let mut state = self.state.write().await;
        let new_state = match *state {
            AppState::Paused => AppState::Active,
            _ => AppState::Paused,
        };
        println!("[State] Toggle: {:?} -> {:?}", *state, new_state);
        *state = new_state;
        new_state
    }
}
