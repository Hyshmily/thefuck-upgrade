# The Fuck [![Version](https://img.shields.io/badge/version-3.33.0-blue.svg)](https://github.com/HyShmily/thefuck-upgrade)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/HyShmily/thefuck-upgrade/blob/main/LICENSE)

[![中文](https://img.shields.io/badge/中文-简体-green.svg)](README_CN.md) [![English](https://img.shields.io/badge/English-blue.svg)](README_EN.md)

*The Fuck* 是一个命令行纠错工具，可以自动修正常见的错误命令。

> [!NOTE]
> 这是 The Fuck 的 Rust 重写版本，当前实现重点放在更清晰的架构、更快的匹配和更容易维护的规则系统。

## 概览

- ⚡ 高性能：Rust 零成本抽象，启动快、匹配快
- 🛡️ 类型安全：编译期检查减少运行时错误
- 🌐 跨平台：支持 Windows、macOS、Linux
- 🔌 易扩展：规则系统模块化，方便添加新规则
- 📦 现代工具链：Cargo、async/await、pre-commit、clippy

## 3.33.0 更新摘要

> [!NOTE]
> 本版本重点是“新命令优先、旧命令平滑迁移”的纠错建议。

- 增加 `git checkout` -> `git switch` / `git switch -c` 建议
- 增加 `pip` -> `uv pip` 建议，并保留 `python -m pip` 兜底
- 增加 Docker 旧命令向新子命令风格迁移建议

> [!TIP]
> 如果你只想快速上手，先看“快速开始”；如果你要改代码，先看“项目结构”和“开发指南”。

## 快速开始

### 安装

```bash
# Windows (PowerShell)
irm https://github.com/HyShmily/thefuck-upgrade/raw/main/install.ps1 | iex

# macOS
brew install thefuck

# Linux
sudo apt install thefuck  # Ubuntu/Debian
sudo pacman -S thefuck     # Arch
```

### 使用

```bash
➜ gti status
➜ fuck
git status [enter/↑/↓/ctrl+c]
```

### Shell 集成

```bash
# Bash / Zsh
 eval "$(thefuck --alias)"

# PowerShell
Invoke-Expression (thefuck --alias | Out-String)
```

## 项目结构

> [!IMPORTANT]
> 下面的目录以当前仓库中的 `thefuck/` 子包为准。

```text
thefuck-upgrade/
├── thefuck/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── firstuse.rs
│   │   ├── argument_parser.rs
│   │   ├── corrector.rs
│   │   ├── history.rs
│   │   ├── io.rs
│   │   ├── system.rs
│   │   ├── entrypoints/
│   │   └── rules/
│   └── tests/
├── README.md
├── README_CN.md
├── README_EN.md
├── README_ROOT.md
├── PROJECT_STRUCTURE.md
├── UPGRADE_SUMMARY.md
└── CONTRIBUTING.md
```

## 核心模块

### `corrector.rs`
- 负责规则匹配、排序和相似度计算

### `rules/`
- 放置 Git、Python、cd、docker-compose、sudo 等规则
- 新规则应先补测试，再注册到 `rules/mod.rs`

### `entrypoints/`
- 负责 `fuck`、`--alias`、`firstuse` 等入口流程

### `io.rs`
- 负责候选输出、交互选择和确认提示

### `history.rs`
- 负责历史记录持久化与读取

> [!WARNING]
> 规则系统会直接影响纠错结果。新增或修改规则时，请先写测试，再改注册表。

## 开发指南

```bash
cd thefuck
cargo build
cargo test
cargo fmt
cargo clippy
```

> [!TIP]
> 开发时优先执行 `cargo fmt` 和 `cargo test`，再跑 `cargo clippy`，这样反馈最快。

## 添加新规则

1. 在 `thefuck/src/rules/` 下创建新文件。
2. 实现规则函数，返回 `Option<MatchResult>`。
3. 在 `thefuck/src/rules/mod.rs` 注册。
4. 在 `thefuck/tests/main.rs` 增加测试。

> [!NOTE]
> 如果规则依赖命令输出或环境状态，尽量把输入抽成最小可测单元，避免测试脆弱。

## 相关文档

- [项目结构](PROJECT_STRUCTURE.md)
- [升级指南](UPGRADE_SUMMARY.md)
- [贡献指南](CONTRIBUTING.md)
- [英文文档](README_EN.md)

## 维护信息

- 维护者: [HyShmily](https://github.com/HyShmily)
- 原项目作者: [Nikita Sivakov](https://github.com/nvbn)
- 许可证: MIT

> [!TIP]
> 当前仓库仍处于持续优化中，建议优先参考最新的测试与项目结构文档。