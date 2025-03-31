use crate::{
    tts::{SherpaOnnxMeloTts, SherpaOnnxPiperTts, TtsEngine, TtsEngineType},
    utils::split_sentences,
    Speaker,
};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use eyre::Result;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub tts: Arc<Mutex<Box<dyn TtsEngine + Send>>>,
    pub speaker: Arc<Speaker>,
}
impl AppState {
    pub fn from_path(path: &str, engine_type: TtsEngineType) -> Self {
        let tts: Box<dyn TtsEngine + Send> = match engine_type {
            TtsEngineType::Melo => Box::new(SherpaOnnxMeloTts::from_path(path)),
            TtsEngineType::Piper => Box::new(SherpaOnnxPiperTts::from_path(path)),
        };

        Self {
            tts: Arc::new(Mutex::new(tts)),
            speaker: Arc::new(Speaker::new()),
        }
    }
}

pub async fn serve(host: &str, port: u16, model_path: &str, engine: &str) -> Result<()> {
    let engine_type = TtsEngineType::from_str_arg(engine);
    let state = AppState::from_path(model_path, engine_type);
    let app = Router::new().route("/speak", post(speak)).with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    tracing::info!("Start listening on http://{}:{}", host, port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeakRequest {
    pub content: String,
    pub sid: Option<i32>,
    pub speed: Option<f32>,
}

async fn speak(
    State(state): State<AppState>,
    Json(payload): Json<SpeakRequest>,
) -> Result<(), StatusCode> {
    state.speaker.clear();
    state.speaker.play();

    let sid = payload.sid.unwrap_or(1);
    let speed = payload.speed.unwrap_or(1.0);
    let sentences = split_sentences(&payload.content);

    for sentence in sentences {
        let audio = {
            let mut tts = state.tts.lock().await;
            tts.create(&sentence, sid, speed).map_err(|e| {
                tracing::error!("Error when creating audio: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?
        };

        state
            .speaker
            .append_samples(audio.samples, audio.sample_rate);
    }

    state.speaker.sleep_until_end();
    Ok(())
}
