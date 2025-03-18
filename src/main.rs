use eyre::Result;
use onnx_tts_server::server;

#[tokio::main]
async fn main() -> Result<()> {
    server::serve(
        "0.0.0.0",
        3001,
        "/home/cuikho210/Projects/train-tts/qiqi/onnx-model",
    )
    .await?;
    Ok(())
}
