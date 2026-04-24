#!/bin/bash

# The Fuck Installer for Linux/macOS

set -e

echo "Installing The Fuck..."

# Check if rust is installed
if ! command -v rustup &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Clone the repository
REPO_DIR="$HOME/thefuck-upgrade"
if [ -d "$REPO_DIR" ]; then
    echo "The Fuck is already installed. Removing the old installation..."
    rm -rf "$REPO_DIR"
fi

echo "Cloning repository..."
git clone https://github.com/nvbn/thefuck-upgrade.git "$REPO_DIR"
cd "$REPO_DIR/thefuck"

# Build The Fuck
echo "Building The Fuck..."
cargo build --release

# Copy to local bin
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"
cp target/release/thefuck "$BIN_DIR/thefuck"
cp target/release/thefuck_firstuse "$BIN_DIR/thefuck_firstuse"

# Add to PATH if not already
if ! grep -q "$BIN_DIR" ~/.bashrc 2>/dev/null; then
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> ~/.bashrc
fi

# Add alias
echo "
# The Fuck alias
alias fuck='sudo TF_HISTORY=\$(fc -l -1 | sed \"s/^[ ]*[0-9]*[ ]*//\") THEFUCK_COMMAND_HISTORY=\$TF_HISTORY thefuck'" >> ~/.bashrc

echo "The Fuck installed successfully!"
echo ""
echo "Restart your shell or run:"
echo "  source ~/.bashrc"
echo ""
echo "Then try:"
echo "  gti status"
echo "  fuck"