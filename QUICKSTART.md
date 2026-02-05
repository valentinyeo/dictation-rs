# Quick Start Guide

## Prerequisites

1. **Install Rust** (if not already installed):
   - Download from https://rustup.rs/
   - Run the installer and follow the prompts
   - Restart your terminal after installation

2. **Get Deepgram API Key**:
   - Sign up at https://console.deepgram.com/
   - Create a new API key
   - Copy the key (you'll need it in step 5)

## Building and Running

1. **Navigate to the project directory**:
```bash
cd dictation-rs
```

2. **Build the project**:
```bash
cargo build --release
```

This will take a few minutes on first build as it downloads and compiles dependencies.

3. **Run the application**:
```bash
target\release\dictation-rs.exe
```

4. **On first run**, the app will create a config file at:
```
%APPDATA%\dictation\config.toml
```

5. **Add your API key**:
   - Right-click the tray icon → Settings
   - Or open `%APPDATA%\dictation\config.toml` in a text editor
   - Replace `api_key = ""` with your Deepgram API key:
     ```toml
     api_key = "your_actual_api_key_here"
     ```
   - Save and close the file

6. **Restart the application** for the API key to take effect

## Using the App

1. **Look for the tray icon** in your system tray (bottom-right of Windows taskbar)
   - Grey icon = Listening (ready to transcribe)
   - Blue icon = Actively transcribing
   - Red icon = Paused

2. **Start dictating**:
   - Click anywhere you want to type (Word, browser, notepad, etc.)
   - Start speaking
   - Icon turns blue when speech is detected
   - Transcribed text appears in your active window

3. **Pause/Resume**:
   - Left-click the tray icon to pause
   - Left-click again to resume

4. **Switch language**:
   - Right-click → Language: English/German

5. **Auto-start on boot**:
   - Right-click → Check "Start on Boot"

## Troubleshooting

**Icon stays grey when I speak:**
- Increase speech volume
- Adjust `energy_threshold` in config.toml (lower = more sensitive)
- Check that your microphone is working

**No text appears:**
- Verify API key is correct in config.toml
- Check internet connection
- Look at console output for errors

**Red icon appears:**
- Another app (Zoom, Meet) may be using the microphone
- Close the other app or disable its microphone

## Next Steps

- Adjust sensitivity in config.toml
- Set up auto-start for convenience
- Test with different applications

Enjoy hands-free dictation!
