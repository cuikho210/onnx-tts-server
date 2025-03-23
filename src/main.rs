use eyre::Result;
use onnx_tts_server::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    server::serve(
        "0.0.0.0",
        3001,
        "/home/cuikho210/Documents/assets/tts-models/vits-piper-en_US-libritts_r-medium",
    )
    .await?;
    Ok(())
}
