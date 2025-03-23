mod engine;
mod melo_tts;
mod piper_tts;

pub use engine::TtsEngine;
pub use melo_tts::SherpaOnnxMeloTts;
pub use piper_tts::SherpaOnnxPiperTts;

pub enum TtsEngineType {
    Melo,
    Piper,
}
impl TtsEngineType {
    pub fn from_str_arg(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "melo" => TtsEngineType::Melo,
            _ => TtsEngineType::Piper, // Default to Piper
        }
    }
}
