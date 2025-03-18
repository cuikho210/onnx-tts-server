use eyre::Result;
use sherpa_rs::tts::{TtsAudio, VitsTts, VitsTtsConfig};

pub struct SherpaOnnxPiperTts {
    tts: VitsTts,
}
impl SherpaOnnxPiperTts {
    pub fn new(config: VitsTtsConfig) -> Self {
        Self {
            tts: VitsTts::new(config),
        }
    }

    pub fn from_path(path: &str) -> Self {
        let config = VitsTtsConfig {
            model: format!("{}/model.onnx", path),
            tokens: format!("{}/tokens.txt", path),
            data_dir: format!("{}/espeak-ng-data", path),
            ..Default::default()
        };

        Self {
            tts: VitsTts::new(config),
        }
    }

    pub fn create(&mut self, text: &str, sid: i32, speed: f32) -> Result<TtsAudio> {
        self.tts.create(text, sid, speed)
    }
}
