#!/usr/bin/bash

mkdir -p ~/.config/systemd/user

echo "Request information"
read -p "Enter the path to the binary file (default: \$HOME/.cargo/bin/onnx-tts-server): " BINARY_PATH
BINARY_PATH=${BINARY_PATH:-$HOME/.cargo/bin/onnx-tts-server}

read -p "Enter the path to the models directory (e.g., ~/onnx-tts/models): " MODEL_PATH

read -p "Enter engine (melo or piper) [default: piper]: " ENGINE
ENGINE=${ENGINE:-piper}

read -p "Enter host to bind [default: 0.0.0.0]: " HOST
HOST=${HOST:-0.0.0.0}
read -p "Enter port [default: 3001]: " PORT
PORT=${PORT:-3001}

echo "Create service file"
cat > ~/.config/systemd/user/onnx-tts.service << EOF
[Unit]
Description=ONNX TTS Server
After=network.target

[Service]
Type=simple
ExecStart=${BINARY_PATH} --model-path ${MODEL_PATH} --engine ${ENGINE} --host ${HOST} --port ${PORT}

[Install]
WantedBy=default.target
EOF

# Reload, enable and start service
systemctl --user daemon-reload
systemctl --user enable --now onnx-tts.service

echo "The onnx-tts service has been installed at the user level and started."
echo "Check status: systemctl --user status onnx-tts"
echo "View logs: journalctl --user -u onnx-tts -f"
