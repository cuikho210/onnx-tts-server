use eyre::Result;
use onnx_tts_server::server;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve(
        "0.0.0.0",
        3001,
        "/home/cuikho210/Documents/assets/tts-models/vits-piper-en_US-libritts_r-medium",
    )
    .await?;
    Ok(())
}
