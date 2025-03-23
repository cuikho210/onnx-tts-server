use eyre::Result;
use sherpa_rs::tts::TtsAudio;

pub trait TtsEngine {
    fn create(&mut self, text: &str, sid: i32, speed: f32) -> Result<TtsAudio>;
}
