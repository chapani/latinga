# Installation Guide

You can use **Latinga** by downloading pre-compiled binaries or building from source.

## 1. Download Pre-compiled Binaries
Go to the [Releases](https://github.com/yourusername/latinga/releases) page and download the version for your OS:

### 🐧 Linux
1. Download `latinga-linux-amd64`.
2. Make it executable: `chmod +x latinga-linux-amd64`.
3. Move it to your path: `sudo mv latinga-linux-amd64 /usr/local/bin/latinga`.

### 🍎 macOS (Apple Silicon M1/M2/M3)
Due to macOS security policies, follow these steps:
1. Download `latinga-macos-arm64`.
2. Open your terminal and run: `chmod +x latinga-macos-arm64`.
3. **Right-click** the file in Finder and select **Open**.
4. When the warning appears, click **Open** (this only needs to be done once).
5. Move to your path: `sudo mv latinga-macos-arm64 /usr/local/bin/latinga`.

### 🪟 Windows
1. Download `latinga-windows-amd64.exe`.
2. (Optional) Rename it to `latinga.exe` and add it to your System PATH.

## 2. Install via Cargo (Recommended for Rust Users)
```bash
cargo install latinga --features cli
