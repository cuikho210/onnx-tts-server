use crate::{tts::SherpaOnnxPiperTts, Speaker};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use eyre::Result;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub tts: Arc<Mutex<SherpaOnnxPiperTts>>,
    pub speaker: Arc<Speaker>,
}
impl AppState {
    pub fn from_path(path: &str) -> Self {
        Self {
            tts: Arc::new(Mutex::new(SherpaOnnxPiperTts::from_path(path))),
            speaker: Arc::new(Speaker::new()),
        }
    }
}

pub async fn serve(host: &str, port: i16, model_path: &str) -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState::from_path(model_path);
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
    let sid = payload.sid.unwrap_or(1);
    let speed = payload.speed.unwrap_or(1.0);

    let audio = {
        let mut tts = state.tts.lock().await;
        tts.create(&payload.content, sid, speed).map_err(|e| {
            tracing::error!("Error when creating audio {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };

    state
        .speaker
        .append_samples(audio.samples, audio.sample_rate);

    Ok(())
}
