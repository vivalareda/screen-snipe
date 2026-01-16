# Screen Snipe

A lightweight, customizable screenshot and OCR tool for macOS with global hotkey support.

## Features

- **Capture Region**: Select and capture a specific area of your screen
- **Capture Fullscreen**: Capture the entire screen instantly
- **OCR**: Select a screen region and extract text to clipboard using Vision framework
- **Custom Hotkeys**: Configure your own keyboard shortcuts
- **Configurable Save Directory**: Choose where screenshots are saved

## Requirements

- macOS (uses `screencapture` and Vision framework)
- Rust toolchain
- Xcode Command Line Tools (for Swift compilation)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/screen-snipe.git
cd screen-snipe
```

2. Compile the OCR helper:
```bash
swiftc -O ocr_helper.swift -o ocr_helper
```

3. Build the project:
```bash
cargo build --release
```

4. Grant Accessibility permissions:
   - Go to **System Preferences → Security & Privacy → Privacy → Accessibility**
   - Add the compiled binary to the list and enable it

## Configuration

Create a config file at `~/.config/screensnipe/screensnipe.conf`:

```bash
mkdir -p ~/.config/screensnipe
nano ~/.config/screensnipe/screensnipe.conf
```

### Config Format

```conf
save_dir=/Users/yourusername/Pictures/Screenshots
cmd+ctrl+9=capture_region
cmd+ctrl+8=ocr
cmd+ctrl+0=capture_fullscreen
```

### Available Actions

- `capture_region` - Interactive region selection screenshot
- `capture_fullscreen` - Full screen capture
- `ocr` - OCR text extraction to clipboard

### Key Modifiers

- `cmd` - Command key (⌘)
- `ctrl` - Control key
- `shift` - Shift key
- `alt` - Alt/Option key
- `0-9` - Number keys

### Default Configuration

If no config file is found, the following defaults are used:

- **Save Directory**: `~/Pictures`
- **Hotkeys**:
  - `cmd+ctrl+9` → Capture Region
  - `cmd+ctrl+8` → OCR
  - `cmd+ctrl+0` → Capture Fullscreen

## Usage

Run the application:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/screen-snipe
```

### Running as a Background Service

To run Screen Snipe on startup, create a LaunchAgent:

1. Create the plist file:
```bash
nano ~/Library/LaunchAgents/com.screensnipe.plist
```

2. Add the following content (adjust paths as needed):
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.screensnipe</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/screen-snipe/target/release/screen-snipe</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

3. Load the agent:
```bash
launchctl load ~/Library/LaunchAgents/com.screensnipe.plist
```

## How It Works

1. **Global Keyboard Listener**: Uses `rdev` to capture keyboard events system-wide
2. **Hotkey Matching**: Tracks pressed keys and matches combinations against your config
3. **Event Blocking**: Consumed hotkeys don't pass through to other applications
4. **Screenshot Capture**: Leverages macOS `screencapture` utility
5. **OCR Processing**: Uses Apple's Vision framework for text recognition

## Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Troubleshooting

### Hotkeys not working
- Ensure the app has Accessibility permissions in System Preferences
- Check that your hotkey combinations don't conflict with system shortcuts

### OCR not working
- Verify the `ocr_helper` binary is compiled and in the project directory
- Make sure Xcode Command Line Tools are installed: `xcode-select --install`

### Screenshots not saving
- Check that the save directory exists and is writable
- Verify the path in your config file is correct

## Project Structure

```
screen-snipe/
├── src/
│   ├── main.rs       # Main application logic
│   ├── config.rs     # Configuration handling
│   └── action.rs     # Action definitions
├── ocr_helper.swift  # Swift OCR utility
├── ocr_helper        # Compiled OCR binary
└── Cargo.toml        # Rust dependencies
```

## Dependencies

- **rdev**: Global keyboard event capture
- **anyhow**: Error handling
- **arboard**: Clipboard management
- **dirs**: System directory paths

## License

MIT License - feel free to use and modify as needed.

## Contributing

Contributions welcome! Feel free to open issues or submit pull requests.
