# Deployment Guide - GitHub Actions Build

This project uses GitHub Actions to automatically build the Windows executable. You don't need to install Rust locally!

## Setup

### 1. Push to GitHub

```bash
cd dictation-rs
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR_USERNAME/dictation-rs.git
git push -u origin main
```

### 2. GitHub Actions Will Build Automatically

Once you push, GitHub Actions will:
- Install Rust on a Windows runner
- Build the release version
- Strip the binary for smaller size
- Create downloadable artifacts

### 3. Download the Built Executable

**Option A: From Actions Tab**
1. Go to your repository on GitHub
2. Click "Actions" tab
3. Click on the latest successful workflow run
4. Scroll to "Artifacts" section
5. Download "dictation-rs-windows"
6. Extract the ZIP file
7. Run `dictation-rs.exe`

**Option B: Create a Release**
1. Create a git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
2. GitHub Actions will create a release automatically
3. Go to "Releases" on GitHub
4. Download `dictation-rs.exe` from the latest release

## First Run Setup

1. **Run the executable**:
   ```
   dictation-rs.exe
   ```

2. **Configure your API key**:
   - The app creates a config file at `%APPDATA%\dictation\config.toml`
   - Right-click tray icon → Settings
   - Add your Deepgram API key:
     ```toml
     [deepgram]
     api_key = "e2406a887911086cca154ab1109fa13e85055ab5"
     language = "en"
     model = "nova-2"
     ```

3. **Restart the app** to load the new configuration

## Updating the App

1. Make changes to the code
2. Commit and push:
   ```bash
   git add .
   git commit -m "Description of changes"
   git push
   ```
3. Download the new build from GitHub Actions

## Build Times

- First build: ~5-10 minutes (downloads and compiles all dependencies)
- Subsequent builds: ~2-3 minutes (uses cached dependencies)

## Troubleshooting

**GitHub Actions fails:**
- Check the Actions tab for error logs
- Ensure the repository has Actions enabled (Settings → Actions)

**Binary doesn't run:**
- Windows may block downloaded executables
- Right-click → Properties → Unblock → OK

**Build is slow:**
- This is normal for Rust compilation
- Subsequent builds will be faster due to caching
