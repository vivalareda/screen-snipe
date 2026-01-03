use anyhow::Result;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const SAVE_DIR: &str = "/Users/lilflare/github/screen-snipe/saved-test";

fn capture_screen_region() -> Result<()> {
    // Generate filename with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let filepath = format!("{}/screenshot_{}.png", SAVE_DIR, timestamp);

    // Use macOS native screencapture with interactive mode (-i)
    let output = Command::new("screencapture")
        .args(["-i", &filepath])
        .output()?;

    if output.status.success() {
        // Check if file was actually created (user might have cancelled)
        if std::path::Path::new(&filepath).exists() {
            println!("Screenshot saved to: {}", filepath);
        } else {
            println!("Screenshot cancelled");
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Screenshot failed: {}", stderr);
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Screen Snipe is running...");
    println!("Press Ctrl + Cmd + 9 to capture a screen region");
    println!("Press Ctrl + C to exit");

    let pressed_keys: Arc<Mutex<HashSet<Key>>> = Arc::new(Mutex::new(HashSet::new()));

    let callback = move |event: Event| {
        let mut keys = pressed_keys.lock().unwrap();

        match event.event_type {
            EventType::KeyPress(key) => {
                keys.insert(key);

                // Check for Ctrl + Cmd + 9
                let has_ctrl = keys.contains(&Key::ControlLeft) || keys.contains(&Key::ControlRight);
                let has_cmd = keys.contains(&Key::MetaLeft) || keys.contains(&Key::MetaRight);
                let has_9 = keys.contains(&Key::Num9);

                if has_ctrl && has_cmd && has_9 {
                    println!("Capturing screenshot...");
                    keys.clear();
                    drop(keys);

                    if let Err(e) = capture_screen_region() {
                        eprintln!("Error capturing screenshot: {}", e);
                    }
                }
            }
            EventType::KeyRelease(key) => {
                keys.remove(&key);
            }
            _ => {}
        }
    };

    // Start listening for global keyboard events
    if let Err(error) = listen(callback) {
        eprintln!("Error listening to events: {:?}", error);
    }

    Ok(())
}
