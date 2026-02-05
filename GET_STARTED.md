# Getting Started - No Rust Installation Required!

This guide shows you how to get the dictation app running **without installing Rust**. GitHub Actions builds the app for you automatically!

## Step 1: Push Code to GitHub

```bash
cd C:\Users\valen\Documents\GitHub\dictation-rs

# Initialize git repository
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Super lean dictation app"

# Create repository on GitHub (via web interface or gh CLI)
# Then link it:
git remote add origin https://github.com/YOUR_USERNAME/dictation-rs.git

# Push to GitHub
git push -u origin main
```

## Step 2: Wait for GitHub Actions to Build

1. Go to your repository on GitHub
2. Click the **"Actions"** tab
3. You'll see a workflow running called "Build Dictation App"
4. Wait for it to complete (green checkmark) - takes about 5-10 minutes

## Step 3: Download the Executable

1. Click on the completed workflow run
2. Scroll down to **"Artifacts"** section
3. Click **"dictation-rs-windows"** to download
4. Extract the ZIP file to a folder (e.g., `C:\Apps\Dictation\`)

## Step 4: Set Up Configuration

**Option A: Manual Setup**
1. Press `Win + R`
2. Type: `%APPDATA%\dictation`
3. Create this folder if it doesn't exist
4. Copy `config.sample.toml` to this folder
5. Rename it to `config.toml`

**Option B: Automatic Setup (Easier!)**
1. Just run `dictation-rs.exe` once
2. It will create the config folder automatically
3. Right-click the tray icon → Settings
4. The config file opens in Notepad
5. Your API key is already filled in!

## Step 5: Run the App

1. Double-click `dictation-rs.exe`
2. Look for the microphone icon in your system tray (bottom-right)
3. The icon should be **grey** (ready to listen)

## Step 6: Test It Out

1. Open any text editor (Notepad, Word, browser, etc.)
2. Click in the text area
3. Start speaking clearly
4. Watch the icon turn **blue** when you speak
5. Transcribed text appears automatically!

## Icon Colors

- 🔵 **Blue** = Actively transcribing (you're speaking)
- ⚪ **Grey** = Listening / Auto-paused (silence)
- 🔴 **Red** = Manually paused or mic conflict

## Controls

- **Left-click icon**: Pause/Resume
- **Right-click icon**: Menu
  - Switch language (English ↔ German)
  - Enable "Start on Boot"
  - Open Settings
  - Exit

## Troubleshooting

**Icon stays grey when speaking:**
- Speak louder or adjust `energy_threshold` in config.toml
- Lower value = more sensitive

**No text appears:**
- Check your internet connection
- Verify API key in config.toml
- Make sure you clicked in a text area

**Red icon appears:**
- Another app (Zoom, Meet) is using your microphone
- Close that app or wait for it to release the mic

## Updating the App

To get a new version:
1. Make changes to the code
2. Push to GitHub: `git push`
3. Wait for GitHub Actions to build
4. Download the new artifact
5. Replace the old `dictation-rs.exe`

## Creating Releases (Optional)

To create a permanent download link:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will create a Release with the executable attached.
Go to your repository → Releases to download.

---

**That's it!** You now have a working dictation app without installing Rust locally.

Your API key is already configured: `e2406a887911086cca154ab1109fa13e85055ab5`
