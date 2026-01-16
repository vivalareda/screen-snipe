use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Action {
    CaptureRegion,
    CaptureFullscreen,
    Ocr,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "capture_region" => Ok(Action::CaptureRegion),
            "capture_fullscreen" => Ok(Action::CaptureFullscreen),
            "ocr" => Ok(Action::Ocr),
            _ => Err(anyhow!("Unknown action {}", s)),
        }
    }
}
