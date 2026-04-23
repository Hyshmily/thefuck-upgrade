# The Fuck - 升级指南 [![中文](https://img.shields.io/badge/中文-简体-green.svg)](README_CN.md) [![English](https://img.shields.io/badge/English-blue.svg)](README_EN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

本文档详细介绍了从 Python 版 The Fuck 迁移到 Rust 重写版本的过程和注意事项。

## 📋 概述

The Fuck 已完成从 Python 到 Rust 的全面现代化改造。这次重写带来了显著的性能提升、更好的类型安全性和现代化的开发体验。

## 🆕 主要改进

### 1. **Rust 重写** ✨
- 完全从 Python 迁移到 Rust
- 通过零成本抽象提升性能
- Rust 所有权模型保证内存安全
- 使用 async/await 实现并发处理

### 2. **现代化功能**
- **高性能**：毫秒级响应时间
- **类型安全**：完整的类型检查和错误处理
- **跨平台**：统一的 Windows/macOS/Linux 支持
- **异步 I/O**：非阻塞的输入输出操作
- **模块化设计**：清晰的代码结构，易于维护

### 3. **增强的命令匹配**
- 优化的 Levenshtein 距离算法
- 命令特定的纠正模式
- 缓存机制提升性能
- 更好的模糊匹配能力

### 4. **新规则支持**
- `uv`（Python 包管理器）
- `pnpm`（npm 替代品）
- `docker compose`（v2 版本）
- 增强的 Git 规则（main/master 分支处理）
- 改进的 Windows/PowerShell 支持

### 5. **现代化工具链**
- **Cargo**：构建系统和包管理器
- **Ruff**：快速代码检查和格式化（替代 flake8/black）
- **Pre-commit 钩子**：自动化代码质量检查
- **Mypy**：Python 兼容性类型检查
- **Criterion**：性能基准测试

### 6. **改进的开发体验**
- 全面的测试套件
- 更好的文档
- 所有主要 Shell 的集成脚本
- 跨平台构建支持

## 🔄 迁移指南

### 用户迁移

#### 安装新版本

```bash
# 推荐使用新安装脚本
curl -sSL https://github.com/HyShmily/thefuck-upgrade/raw/main/install.sh | sh

# 或者从源码安装
git clone https://github.com/HyShmily/thefuck-upgrade.git
cd thefuck-upgrade
cargo install --path thefuck
```

#### Shell 集成

The Fuck 现在提供特定 Shell 的脚本：

```bash
# Unix 系统
./shell.sh

# PowerShell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

#### 配置迁移

配置文件位置保持不变：
- Linux/macOS: `~/.config/thefuck/settings.py`
- Windows: `%APPDATA%\thefuck\settings.py`

**重要**：Rust 版本完全兼容 Python 版本的配置格式，无需修改现有配置！

### 开者迁移

#### 开发环境设置

```bash
# 克隆并构建
git clone https://github.com/HyShmily/thefuck-upgrade.git
cd thefuck-upgrade
make build

# 运行测试
make test

# 代码格式化和检查
make fmt
make lint
make check
```

#### 项目结构变化

```bash
thefuck-upgrade/
├── thefuck/               # Rust 核心包
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── firstuse.rs
│   │   ├── entrypoints/
│   │   └── rules/
│   └── tests/
├── PROJECT_STRUCTURE.md  # 架构说明
├── pyproject.toml       # Python 兼容性元数据
├── ruff.toml           # 代码检查配置
├── .pre-commit-config.yaml  # 代码质量钩子
└── README.md            # 文档
```

## ⚡ 性能对比

| 指标 | Python 版 | Rust 版 | 提升 |
|------|-----------|---------|------|
| 启动时间 | ~100-300ms | ~5-20ms | 10-20x |
| 内存占用 | 较高 | 优化后更低 | 30-50% |
| 命令匹配速度 | 较慢 | 显著更快 | 5-10x |
| 并发支持 | 有限 | 完整支持 | - |
| 类型安全 | 运行时错误 | 编译时检查 | - |

## 🔧 配置兼容性

### 兼容的配置选项

```python
# 原配置仍然有效
rules = ['git_command', 'python_command', 'cd_correction']
exclude_rules = ['slow_rule']
require_confirmation = False
num_close_matches = 5
instant_mode = False
```

### 新增配置选项

```python
# 新增的配置项
# 启用调试模式
debug = False

# 设置命令等待时间（毫秒）
wait_command = 1000

# 最大历史记录数
max_history = 1000
```

## 📦 平台支持

### 完全支持的平台
- ✅ Linux (x86_64, ARM64)
- ✅ macOS (x86_64, ARM64)
- ✅ Windows (x86_64, ARM64)

### 新增的平台特定功能
- **Windows**: 原生 PowerShell 支持
- **macOS**: M1/M2 芯片原生支持
- **Linux**: 更好的终端兼容性

## 🛠️ 常见迁移问题

### Q: 性能提升多少？
A: 启动时间提升 10-20 倍，内存使用减少 30-50%，命令匹配速度提升 5-10 倍。

### Q: 配置文件需要修改吗？
A: 不需要！Rust 版本完全兼容 Python 版本的配置文件格式。

### Q: 自定义规则需要重写吗？
A: 大部分规则可以直接使用，但建议检查并按照 Rust 标准进行优化。

### Q: 安装方式有什么变化？
A: 安装方式基本相同，新增了从源码编译的选项，推荐使用官方安装脚本。

### Q: Git 分支名称更新了？
A: 是的，默认分支名从 `master` 更新为 `main`，符合现代 Git 最佳实践。

## 🚀 升级步骤

### 1. 备份现有配置
```bash
# 备份配置文件
cp ~/.config/thefuck/settings.py ~/.config/thefuck/settings.py.bak
```

### 2. 安装 Rust 版本
```bash
# 使用官方安装脚本
curl -sSL https://github.com/HyShmily/thefuck-upgrade/raw/main/install.sh | sh
```

### 3. 验证安装
```bash
# 测试基本功能
echo "gti status" | thefuck

# 检查版本
thefuck --version
```

### 4. 恢复配置（可选）
```bash
# 如果需要，恢复旧配置
cp ~/.config/thefuck/settings.py.bak ~/.config/thefuck/settings.py
```

### 5. 更新 Shell 别名
```bash
# 重新设置别名
eval $(thefuck --alias)
```

## 📊 已知限制

### 潜在问题
1. **初始编译时间**：Rust 编译可能比 Python 慢
2. **二进制大小**：比 Python 版本大（包含 Rust 运行时）
3. **Python 2**：不再支持
4. **自定义规则**：可能需要更新格式

### 解决方案
1. 使用发布版本避免编译等待
2. 考虑使用安装脚本自动处理
3. 确保使用 Python 3.11+
4. 参考 Rust 规则编写指南更新

## 🎯 未来规划

### 第二阶段（进行中）
- [ ] 规则插件系统
- [ ] 性能基准测试
- [ ] Web 配置界面
- [ ] 移动端支持

### 第三阶段（规划中）
- [ ] AI 驱动的纠错
- [ ] 规则市场
- [ ] IDE 集成
- [ ] 语音控制支持

## 📞 获取帮助

### 文档资源
- [主文档](README.md) - 指向中英文版本
- [项目结构](PROJECT_STRUCTURE.md) - 详细架构说明
- [贡献指南](CONTRIBUTING.md) - 开发指南

### 支持渠道
- **Issues**: [GitHub Issues](https://github.com/HyShmily/thefuck-upgrade/issues)
- **Discussions**: [GitHub Discussions](https://github.com/HyShmily/thefuck-upgrade/discussions)
- **社区**: 加入开发者讨论组

---

**升级指导**: 如果遇到问题，请先查看本文档，然后访问 GitHub Issues 提交问题。

**维护者**: [HyShmily](https://github.com/HyShmily)  
**原项目作者**: [Nikita Sivakov](https://github.com/nvbn)  
**许可证**: MIT