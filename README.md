# ONNX TTS Server

A simple TTS server that uses ONNX-based TTS models via Sherpa-RS binding.

## Installation

```bash
# Clone the repository
git clone https://github.com/cuikho210/onnx-tts-server.git --single-branch --depth 1
cd onnx-tts-server

# Build the server
cargo install --path .

# Generate default config
onnx-tts-server --gen-config
```

## Usage

### Running the server

```bash
# Start with default settings
onnx-tts-server

# Custom configuration
onnx-tts-server --model-path /path/to/model --port 8080
```

### Installing as a systemd service

```bash
# Make the script executable
chmod +x install-as-systemd-service.sh

# Install the service
./install-as-systemd-service.sh
```

### API Endpoints

#### POST /speak

Synthesize and play speech from text.

Request:

```json
{
  "content": "Hello, this is a test.",
  "sid": 1, // Optional: Speaker ID (default: 1)
  "speed": 1.0 // Optional: Speech speed (default: 1.0)
}
```

## Configuration

The server can be configured through:

1. Command-line arguments
2. Configuration file at `~/.config/onnx-tts-server/config.toml` (Linux/macOS) or `%APPDATA%\onnx-tts-server\config.toml` (Windows)

Command-line arguments take precedence over configuration file settings. If the same option is specified in both places, the command-line value will be used.

Example config.toml:

```toml
[server]
host = "0.0.0.0"
port = 3001

[tts]
model_path = "./tts-models/default"
engine = "piper"  # or "melo"
```

## Model Support

The server supports two types of TTS engines:

1. **Piper**: Requires model.onnx, tokens.txt, and espeak-ng-data directory
2. **Melo**: Requires model.onnx, tokens.txt, and lexicon.txt
