use clap::Parser;
use eyre::Result;
use onnx_tts_server::server;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to TTS model directory
    #[clap(short, long, default_value = "./tts-models/default")]
    model_path: String,

    /// Host address to bind
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[clap(long, default_value = "3001")]
    port: i16,

    /// TTS engine type: "melo" or "piper"
    #[clap(long, default_value = "piper")]
    engine: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    tracing::info!("Loading TTS model from: {}", args.model_path);

    server::serve(&args.host, args.port, &args.model_path, &args.engine).await?;
    Ok(())
}
