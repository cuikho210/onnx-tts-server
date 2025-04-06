use crate::config::AppConfig;
use eyre::Result;
use sherpa_rs::tts::{TtsAudio, VitsTts, VitsTtsConfig};

pub struct SherpaOnnxTts {
    tts: VitsTts,
}
impl SherpaOnnxTts {
    pub fn new(config: VitsTtsConfig) -> Self {
        Self {
            tts: VitsTts::new(config),
        }
    }

    pub fn from_app_config(app_config: &AppConfig) -> Self {
        let mut vits_tts_config = VitsTtsConfig {
            model: app_config.tts.model.to_owned(),
            tokens: app_config.tts.tokens.to_owned(),
            ..Default::default()
        };

        if let Some(path) = app_config.tts.lexicon.as_ref() {
            vits_tts_config.lexicon = path.to_owned();
        }
        if let Some(path) = app_config.tts.espeak_ng_data.as_ref() {
            vits_tts_config.data_dir = path.to_owned();
        }

        Self {
            tts: VitsTts::new(vits_tts_config),
        }
    }

    pub fn create(&mut self, text: &str, sid: i32, speed: f32) -> Result<TtsAudio> {
        self.tts.create(text, sid, speed)
    }
}
