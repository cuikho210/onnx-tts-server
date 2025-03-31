#!/usr/bin/bash

mkdir -p ~/.config/systemd/user

BINARY_PATH=${BINARY_PATH:-$HOME/.cargo/bin/onnx-tts-server}
echo "Using binary path: ${BINARY_PATH}"

echo "Create service file"
cat > ~/.config/systemd/user/onnx-tts-server.service << EOF
[Unit]
Description=ONNX TTS Server
After=network.target

[Service]
Type=simple
ExecStart=${BINARY_PATH}

[Install]
WantedBy=default.target
EOF

# Reload, enable and start service
systemctl --user daemon-reload
systemctl --user enable --now onnx-tts-server.service

echo "The onnx-tts-server service has been installed at the user level and started."
echo "Check status: systemctl --user status onnx-tts-server"
echo "View logs: journalctl --user -u onnx-tts-server -f"
echo "Configure via: ~/.config/onnx-tts-server/config.toml"
