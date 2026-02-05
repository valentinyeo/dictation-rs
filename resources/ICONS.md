# Icons

This directory should contain three microphone icon files:

- `mic-blue.ico` - Active dictation (blue #0078D4)
- `mic-grey.ico` - Listening/silence (grey #8A8A8A)
- `mic-red.ico` - Paused/conflict (red #D13438)

## How to Create Icons

1. Download Fluent UI microphone icon from:
   https://github.com/microsoft/fluentui-system-icons

2. Search for "mic" (microphone icon)

3. Export as .ico format in 16x16, 32x32, 48x48 sizes

4. Create three color variations (blue, grey, red) as listed above

5. Save the files in this directory

## Temporary Solution

Currently, the app generates simple circular icons programmatically in `src/tray/icon.rs`.
For production use, replace with proper Fluent UI icons.
