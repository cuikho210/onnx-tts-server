use clap::Parser;
use eyre::Result;
use onnx_tts_server::{config::Config, server};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to TTS model directory
    #[clap(short, long)]
    model_path: Option<String>,

    /// Host address to bind
    #[clap(long)]
    host: Option<String>,

    /// Port to listen on
    #[clap(long)]
    port: Option<u16>,

    /// TTS engine type: "melo" or "piper"
    #[clap(long)]
    engine: Option<String>,

    /// Generate default config file
    #[clap(long)]
    gen_config: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    if args.gen_config {
        if let Some(config_path) = Config::default_path() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let default_config = Config::default();
            default_config.save_to_file(&config_path)?;

            tracing::info!("Default config generated at: {}", config_path.display());
            return Ok(());
        } else {
            tracing::error!("Couldn't determine default config path");
            return Err(eyre::eyre!("Couldn't determine default config path"));
        }
    }

    let config = Config::load_config();

    let host = args.host.unwrap_or(config.server.host);
    let port = args.port.unwrap_or(config.server.port);
    let model_path = args.model_path.unwrap_or(config.tts.model_path);
    let engine = args.engine.unwrap_or(config.tts.engine);

    tracing::info!("Loading TTS model from: {}", &model_path);

    server::serve(&host, port, &model_path, &engine).await?;
    Ok(())
}
