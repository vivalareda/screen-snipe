mod action;
pub mod config;

use crate::action::Action;
use anyhow::{anyhow, Result};
use arboard::Clipboard;
use dirs::picture_dir;
use rdev::{grab, Event, EventType, Key};
use std::collections::HashSet;
use std::str::FromStr;
use std::{
    collections::HashMap,
    env::home_dir,
    fs::read_to_string,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::config::Config;

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn capture_screen_region(save_dir: &str) -> Result<()> {
    let filepath = format!("{}/screenshot_test_{}.png", save_dir, get_timestamp());
    let output = Command::new("screencapture")
        .args(["-i", &filepath])
        .output()?;

    if output.status.success() {
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

fn capture_fullscreen(save_dir: &str) -> Result<()> {
    let filepath = format!("{}/screenshot_test_{}.png", save_dir, get_timestamp());
    let output = Command::new("screencapture")
        .args(["-W", &filepath])
        .output()?;

    if output.status.success() {
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

fn ocr_text() -> Result<String> {
    let temp_path = format!("/tmp/screenshot_ocr_{}.png", get_timestamp());
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();
    let ocr_helper = exe_dir.join("ocr_helper");

    let mut clipboard = Clipboard::new()?;
    Command::new("screencapture")
        .args(["-i", "-x", &temp_path])
        .output()?;

    if !std::path::Path::new(&temp_path).exists() {
        return Err(anyhow!("Screenshot cancelled"));
    }

    let ocr = Command::new(&ocr_helper).arg(&temp_path).output()?;

    let text = String::from_utf8_lossy(&ocr.stdout).trim().to_string();

    if let Err(e) = std::fs::remove_file(&temp_path) {
        eprint!("Could not delete file {}", e);
    };

    if let Err(e) = clipboard.set_text(&text) {
        eprint!("Could not paste to clipboard {}", e);
    }

    Ok(text)
}

fn load_config() -> Result<Config> {
    let keymaps: Arc<Mutex<HashMap<String, Action>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut save_dir = picture_dir().unwrap();
    let home = home_dir().ok_or_else(|| anyhow!("Could not found home directory"))?;
    let config_path = home
        .join(".config")
        .join("screensnipe")
        .join("screensnipe.conf");

    if !config_path.exists() {
        eprintln!("No config file found at {:?}, using default", config_path);
        return Ok(Config::default());
    }

    let file = read_to_string(config_path).unwrap();
    let parts = file.split("\n");

    for line in parts {
        if line.trim().is_empty() {
            continue;
        }

        let mut split = line.split("=");
        let key = split
            .next()
            .ok_or_else(|| anyhow!("Invalid line: {}", line))?
            .trim();
        let value = split
            .next()
            .ok_or_else(|| anyhow!("Invalid line: {}", line))?
            .trim();

        if key == "save_dir" {
            save_dir = PathBuf::from(value);
        } else {
            let action = Action::from_str(value)?;
            keymaps.lock().unwrap().insert(key.to_string(), action);
        }
    }

    let config = Config::new(save_dir, keymaps);

    Ok(config)
}

fn key_combo_to_string(keys: &HashSet<Key>) -> String {
    let mut combo = Vec::new();

    if keys.contains(&Key::MetaLeft) || keys.contains(&Key::MetaRight) {
        combo.push("cmd");
    };

    if keys.contains(&Key::ControlLeft) || keys.contains(&Key::ControlRight) {
        combo.push("ctrl");
    };

    if keys.contains(&Key::ShiftLeft) || keys.contains(&Key::ShiftRight) {
        combo.push("shift");
    };

    if keys.contains(&Key::Alt) || keys.contains(&Key::AltGr) {
        combo.push("alt");
    };

    for (key, name) in [
        (Key::Num0, "0"),
        (Key::Num1, "1"),
        (Key::Num2, "2"),
        (Key::Num3, "3"),
        (Key::Num4, "4"),
        (Key::Num5, "5"),
        (Key::Num6, "6"),
        (Key::Num7, "7"),
        (Key::Num8, "8"),
        (Key::Num9, "9"),
    ] {
        if keys.contains(&key) {
            combo.push(name);
        }
    }

    combo.join("+")
}

fn execute_action(action: &Action, save_dir: &str) {
    match action {
        Action::CaptureRegion => {
            if let Err(e) = capture_screen_region(save_dir) {
                eprintln!("Failed to capture region: {}", e);
            }
        }
        Action::CaptureFullscreen => {
            if let Err(e) = capture_fullscreen(save_dir) {
                eprintln!("Failed to capture fullscreen: {}", e);
            }
        }
        Action::Ocr => match ocr_text() {
            Ok(text) => println!("OCR text copied to clipboard: {}", text),
            Err(e) => eprintln!("OCR failed: {}", e),
        },
    }
}

fn main() {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            Config::default()
        }
    };

    let save_dir_str = config.save_dir.to_string_lossy().to_string();
    let keys: Arc<Mutex<HashSet<Key>>> = Arc::new(Mutex::new(HashSet::new()));
    let keys_clone = Arc::clone(&keys);
    let keymaps = Arc::clone(&config.keymaps);

    let callback = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("Key pressed: {:?}", key);
                let mut curr_keys = keys_clone.lock().unwrap();
                curr_keys.insert(key);

                let combo = key_combo_to_string(&curr_keys);
                if !combo.is_empty() {
                    println!("Current combo: {}", combo);
                    if let Some(action) = keymaps.lock().unwrap().get(&combo) {
                        execute_action(action, &save_dir_str);
                        return None;
                    }
                }

                Some(event)
            }
            EventType::KeyRelease(key) => {
                println!("Removing key: {:?}", key);
                let mut curr_keys = keys_clone.lock().unwrap();
                curr_keys.remove(&key);
                Some(event)
            }
            _ => Some(event),
        }
    };

    if let Err(e) = grab(callback) {
        eprintln!("Error: {:?}", e);
    }
}
