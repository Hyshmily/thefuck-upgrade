# The Fuck [![Version](https://img.shields.io/badge/version-3.33.0-blue.svg)](https://github.com/HyShmily/thefuck-upgrade)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/HyShmily/thefuck-upgrade/blob/main/LICENSE)

[![中文](https://img.shields.io/badge/中文-简体-green.svg)](README_CN.md) [![English](https://img.shields.io/badge/English-blue.svg)](README_EN.md)

*The Fuck* is a magnificent command-line tool that corrects errors in previous console commands, inspired by a [@liamosaur](https://twitter.com/liamosaur/) [tweet](https://twitter.com/liamosaur/status/506975850596536320).

> [!NOTE]
> This is the Rust rewrite of the classic command-line error correction tool, led by **HyShmily**, bringing significant performance improvements and modern features.

## 🚀 Introduction

The Fuck is a **Rust rewrite** of the classic command-line error correction tool, bringing significant performance improvements and modern features:

- ⚡ **High Performance**: Built with Rust, zero-cost abstractions, millisecond response
- 🔧 **Modern Toolchain**: Uses Cargo, Rust 2021 edition, async programming
- 🛡️ **Memory Safety**: Rust's ownership model guarantees memory safety
- 🌐 **Cross-Platform**: Perfect support for Windows/macOS/Linux
- 🚀 **Async Processing**: Supports concurrent command execution for better response
- 🎯 **Smart Matching**: Optimized Levenshtein algorithm for more accurate corrections
- 🔌 **Highly Extensible**: Clear modular design, easy to add new rules
- 💪 **Modern Tool Support**: Supports `uv`, `pnpm`, `docker compose` and more

## 🆕 What's New in 3.33.0

> [!IMPORTANT]
> 3.33.0 focuses on modern command migrations and safer correction defaults.

- Added `git checkout` migration suggestions to `git switch` / `git switch -c`
- Added `pip` modernization suggestions to `uv pip` while preserving `python -m pip` fallback
- Added Docker legacy command migration suggestions:
	- `docker-compose ...` -> `docker compose ...`
	- `docker images ...` -> `docker image ls ...`
	- `docker ps ...` -> `docker container ls ...`
- Expanded integration tests to cover modernization rules and compatibility behavior

## 📖 Documentation

Select your language for detailed documentation:

- 🇨🇳 **[中文文档](README_CN.md)** - 完整的中文版使用指南
- 🇺🇸 **[English Documentation](README_EN.md)** - Complete English documentation

## 🚀 Quick Start

### Quick Installation

#### **Windows (PowerShell)**
```powershell
irm https://github.com/HyShmily/thefuck-upgrade/raw/main/install.ps1 | iex
```

#### **macOS (Homebrew)**
```bash
brew install thefuck
```

#### **Linux**
```bash
# Ubuntu/Debian
sudo apt install thefuck

# Arch Linux
sudo pacman -S thefuck

# Install from source (recommended)
git clone https://github.com/HyShmily/thefuck-upgrade.git
cd thefuck-upgrade
cargo install --path thefuck
```

### Basic Usage

```bash
➜ gti status
➜ fuck
git status [enter/↑/↓/ctrl+c]
* main
```

## 📋 Project Info

- **Maintainer**: [HyShmily](https://github.com/HyShmily)
- **Original Project Author**: [Nikita Sivakov](https://github.com/nvbn)
- **License**: MIT
- **Tech Stack**: Rust, Cargo, Async Programming
- **Supported Platforms**: Windows, macOS, Linux

## 🔗 Related Links

- [Project Structure Documentation](PROJECT_STRUCTURE.md)
- [Upgrade Guide](UPGRADE_SUMMARY.md)
- [Contributing Guide](CONTRIBUTING.md)
- [License](LICENSE)

## 🙏 Acknowledgments

- Original inspiration from [@liamosaur](https://twitter.com/liamosaur/)'s idea
- Thanks to thefuck original project author [Nikita Sivakov](https://github.com/nvbn) for creating this excellent tool
- Thanks to all contributors for making this project better
- Thanks to the Rust community for providing excellent tooling

---

**Tip**: For detailed usage instructions, installation configuration, and development guide, please select the appropriate language to view the complete documentation.