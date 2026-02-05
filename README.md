# Dictation App

Ultra-lightweight dictation application using Rust with Deepgram Nova2/3 streaming transcription.

## Features

- ✅ Real-time speech-to-text transcription using Deepgram
- ✅ Types transcribed text directly into any active window
- ✅ System tray control with pause/unpause
- ✅ Auto-pause on silence detection (saves API credits)
- ✅ Audio ducking - lowers other app volumes when speaking
- ✅ Microphone-only capture - ignores system audio (YouTube, Spotify, etc.)
- ✅ Support for English and German languages
- ✅ Start on boot option

## System Requirements

- Windows 10/11
- Rust toolchain (for building)
- Deepgram API key ([Get one here](https://console.deepgram.com/))

## Installation

### From Source

1. Clone the repository:
```bash
git clone <repository-url>
cd dictation-rs
```

2. Build the release version:
```bash
cargo build --release
```

3. The executable will be at `target/release/dictation-rs.exe`

### Optional: Size Optimization

For even smaller binary size (2-3MB):
```bash
cargo build --release
strip target/release/dictation-rs.exe
upx --best --lzma target/release/dictation-rs.exe
```

## Configuration

On first run, the app creates a configuration file at:
```
%APPDATA%\dictation\config.toml
```

Edit this file to add your Deepgram API key:

```toml
[deepgram]
api_key = "your_deepgram_api_key_here"
language = "en"  # "en" or "de"
model = "nova-2"

[audio]
silence_threshold_ms = 3000  # Auto-pause after 3 seconds of silence
duck_volume = 0.2  # Other apps volume during dictation (20%)

[vad]
energy_threshold = 0.02  # Speech detection sensitivity
```

## Usage

### System Tray Controls

**Left-click icon**: Toggle pause/unpause

**Right-click icon**: Open menu
- Resume/Pause
- Language: English / German (toggle)
- Start on Boot (checkbox)
- Settings (opens config.toml)
- Exit

### Icon Colors

- **Blue**: Actively transcribing (speaking detected)
- **Grey**: Listening / Auto-paused (silence detected)
- **Red**: Manually paused or microphone conflict

## How It Works

1. **Audio Capture**: Captures microphone input at 16kHz mono (optimal for Deepgram)
2. **Voice Activity Detection**: Detects when you're speaking using energy-based VAD
3. **Audio Ducking**: Automatically lowers other application volumes when you speak
4. **Deepgram Streaming**: Sends audio to Deepgram API for real-time transcription
5. **Keyboard Simulation**: Types finalized transcripts into the active window using Win32 SendInput
6. **Auto-pause**: Closes Deepgram connection after 3 seconds of silence to save credits

## Performance

- Binary size: 3-5MB (2-3MB with UPX)
- RAM usage: 5-10MB idle, 8-12MB during speech
- CPU usage: <1% idle, 2-5% during transcription
- Network: ~32KB/s during speech

## Troubleshooting

### No microphone detected
- Check that your microphone is set as the default recording device in Windows Sound Settings

### Microphone conflict (red icon)
- Another application (Zoom, Google Meet, etc.) may have exclusive access to the microphone
- Close the other application or disable its microphone to use dictation

### Transcription not appearing
- Ensure your Deepgram API key is valid
- Check that you have internet connectivity
- Verify the energy_threshold in config.toml (lower = more sensitive)

### Settings not opening
- Right-click tray icon → Settings
- Or manually navigate to `%APPDATA%\dictation\config.toml`

## Development

### Project Structure

```
dictation-rs/
├── src/
│   ├── main.rs           # Entry point and coordination
│   ├── config.rs         # Configuration management
│   ├── state.rs          # State machine
│   ├── audio/
│   │   ├── capture.rs    # Microphone capture (cpal)
│   │   ├── vad.rs        # Voice activity detection
│   │   └── ducking.rs    # Windows audio ducking
│   ├── deepgram/
│   │   ├── client.rs     # WebSocket streaming client
│   │   └── types.rs      # API types
│   ├── keyboard/
│   │   └── simulator.rs  # Win32 keyboard simulation
│   └── tray/
│       └── icon.rs       # System tray management
└── resources/
    ├── mic-blue.ico      # Active dictation icon
    ├── mic-grey.ico      # Listening/silence icon
    └── mic-red.ico       # Paused/conflict icon
```

## License

[Add your license here]

## Credits

- Powered by [Deepgram](https://deepgram.com/) API
- Icons from [Microsoft Fluent UI System Icons](https://github.com/microsoft/fluentui-system-icons)
