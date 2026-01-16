use dirs::picture_dir;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::action::Action;

pub struct Config {
    pub save_dir: PathBuf,
    pub keymaps: Arc<Mutex<HashMap<String, Action>>>,
}

impl Config {
    pub fn new(save_dir: PathBuf, keymaps: Arc<Mutex<HashMap<String, Action>>>) -> Self {
        Self { save_dir, keymaps }
    }

    pub fn default() -> Self {
        let keymaps: Arc<Mutex<HashMap<String, Action>>> = Arc::new(Mutex::new(HashMap::new()));

        {
            let mut map = keymaps.lock().unwrap();
            map.insert("cmd+ctrl+9".to_string(), Action::CaptureRegion);
            map.insert("cmd+ctrl+8".to_string(), Action::Ocr);
            map.insert("cmd+ctrl+0".to_string(), Action::CaptureFullscreen);
        }

        Self {
            save_dir: picture_dir().unwrap(),
            keymaps: keymaps,
        }
    }
}
