# The Fuck

[![中文](https://img.shields.io/badge/中文-简体-green.svg)](README_CN.md) [![English](https://img.shields.io/badge/English-blue.svg)](README_EN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE) [![Version](https://img.shields.io/badge/version-3.33.0-blue.svg)](https://github.com/HyShmily/thefuck-upgrade)

> [!NOTE]
> This project is the Rust rewrite of the classic command-line error correction tool. If you are new here, start with the language-specific README.

## Quick Start

## What's New in 3.33.0

> [!NOTE]
> This release expands command modernization rules and deprecates older command styles through suggestions.

- `git checkout` suggestions now prefer `git switch`
- `pip` workflows can be upgraded to `uv pip`
- Docker legacy command styles are migrated toward modern subcommand forms

### Install

```bash
# Windows (PowerShell)
irm https://github.com/HyShmily/thefuck-upgrade/raw/main/install.ps1 | iex

# macOS (Homebrew)
brew install thefuck

# Linux
sudo apt install thefuck  # Ubuntu/Debian
sudo pacman -S thefuck    # Arch
```

### Use

```bash
➜ gti status
➜ fuck
git status [enter/↑/↓/ctrl+c]
* main
```

> [!TIP]
> If you only need the core concept and install command, this page is enough. For full usage, open the Chinese or English README.

## Documentation

- [中文文档](README_CN.md) - Full usage guide in Chinese
- [English Documentation](README_EN.md) - Full usage guide in English
- [Project Structure](PROJECT_STRUCTURE.md) - Current code layout
- [Upgrade Guide](UPGRADE_SUMMARY.md) - Migration notes and roadmap
- [Contributing Guide](CONTRIBUTING.md) - Development workflow

## Project Info

- Maintainer: [HyShmily](https://github.com/HyShmily)
- Original project: [thefuck](https://github.com/nvbn/thefuck)
- Stack: Rust, Cargo, async programming
- License: MIT

> [!WARNING]
> This repository contains a Rust implementation in progress. Some advanced behaviors may still be evolving, so check the tests and release notes before relying on edge-case corrections.
