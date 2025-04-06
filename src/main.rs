use clap::Parser;
use eyre::Result;
use onnx_tts_server::{config::AppConfig, server};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Host address to bind
    #[clap(long)]
    host: Option<String>,

    /// Port to listen on
    #[clap(long)]
    port: Option<u16>,

    /// Generate default config file
    #[clap(long)]
    gen_config: bool,

    /// Path to TTS model directory
    #[clap(short, long)]
    model: Option<String>,

    /// Path to TTS model directory
    #[clap(short, long)]
    tokens: Option<String>,

    /// Path to TTS model directory
    #[clap(short, long)]
    lexicon: Option<String>,

    /// Path to TTS model directory
    #[clap(short, long)]
    espeak_ng_data: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    if args.gen_config {
        if let Some(config_path) = AppConfig::default_path() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let default_config = AppConfig::default();
            default_config.save_to_file(&config_path)?;

            tracing::info!("Default config generated at: {}", config_path.display());
            return Ok(());
        } else {
            tracing::error!("Couldn't determine default config path");
            return Err(eyre::eyre!("Couldn't determine default config path"));
        }
    }

    let config = {
        let mut c = AppConfig::load_config();

        if let Some(val) = args.model {
            c.tts.model = val;
        }
        if let Some(val) = args.tokens {
            c.tts.tokens = val;
        }
        if let Some(val) = args.lexicon {
            c.tts.lexicon = Some(val);
        }
        if let Some(val) = args.espeak_ng_data {
            c.tts.espeak_ng_data = Some(val);
        }

        if let Some(val) = args.host {
            c.server.host = val;
        }
        if let Some(val) = args.port {
            c.server.port = val;
        }

        c
    };

    tracing::info!("Loading TTS model from: {}", &config.tts.model);

    server::serve(&config).await?;
    Ok(())
}
