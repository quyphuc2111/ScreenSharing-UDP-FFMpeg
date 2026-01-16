#!/bin/bash

# Script tự động cài đặt FFmpeg cho macOS

echo "========================================"
echo "  FFmpeg Auto-Install Script (macOS)"
echo "========================================"
echo ""

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found!"
    echo ""
    echo "Please install Homebrew first:"
    echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    echo ""
    exit 1
fi

echo "✓ Homebrew found"
echo ""

# Check if FFmpeg is already installed
if command -v ffmpeg &> /dev/null; then
    echo "✓ FFmpeg already installed!"
    echo ""
    ffmpeg -version | head -n 1
    echo ""
    echo "FFplay available: $(command -v ffplay)"
    echo ""
    read -p "Do you want to reinstall/upgrade? (y/N): " response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo "Skipping installation."
        exit 0
    fi
fi

# Install FFmpeg
echo "[1/2] Installing FFmpeg via Homebrew..."
echo "      This may take a few minutes..."
echo ""

brew install ffmpeg

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ FFmpeg installed successfully!"
else
    echo ""
    echo "❌ Installation failed!"
    exit 1
fi

# Verify installation
echo ""
echo "[2/2] Verifying installation..."
echo ""

if command -v ffmpeg &> /dev/null && command -v ffplay &> /dev/null; then
    echo "✓ FFmpeg: $(which ffmpeg)"
    echo "✓ FFplay: $(which ffplay)"
    echo ""
    ffmpeg -version | head -n 1
    echo ""
    echo "========================================"
    echo "  Installation Complete!"
    echo "========================================"
    echo ""
    echo "Next steps:"
    echo "  1. Run: npm run tauri dev"
    echo "  2. Or build: npm run tauri build"
    echo ""
else
    echo "❌ Verification failed!"
    echo "FFmpeg or FFplay not found in PATH"
    exit 1
fi
