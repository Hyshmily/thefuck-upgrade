# The Fuck - Project Structure Documentation [![中文](https://img.shields.io/badge/中文-简体-green.svg)](README_CN.md) [![English](https://img.shields.io/badge/English-blue.svg)](README_EN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> [!NOTE]
> This document provides a detailed introduction to the project structure and module design of The Fuck Rust rewrite version.

本文档详细介绍了 The Fuck Rust 重写版本的项目结构和模块设计。

## 📁 Project Overview

> [!IMPORTANT]
> The following directory structure shows the complete layout of the project:

```
thefuck-upgrade/
├── thefuck/                    # Rust core package
│   ├── Cargo.toml              # Rust project configuration
│   ├── src/
│   │   ├── lib.rs              # Library entry point (shared by tests and binary)
│   │   ├── main.rs             # Thefuck main binary entry
│   │   ├── firstuse.rs         # Thefuck_firstuse binary entry
│   │   ├── argument_parser.rs  # CLI argument parsing
│   │   ├── types.rs            # Core type definitions
│   │   ├── conf.rs             # Configuration management
│   │   ├── corrector.rs        # Rule matching and sorting
│   │   ├── history.rs          # History persistence
│   │   ├── io.rs               # Interaction and output
│   │   ├── system.rs           # Terminal system initialization
│   │   ├── entrypoints.rs      # Entry module exports
│   │   ├── entrypoints/
│   │   │   ├── alias.rs
│   │   │   ├── fix_command.rs
│   │   │   └── firstuse.rs
│   │   ├── util.rs             # Shared utilities (levenshtein, etc.)
│   │   └── rules/
│   │       ├── mod.rs          # Rule registry (68 registered rules)
│   │       ├── helpers.rs      # Shared rule helpers (replace_first, prepend, etc.)
│   │       ├── cd.rs
│   │       ├── git.rs
│   │       ├── python.rs
│   │       └── ... (26 more rule modules)
│   └── tests/
│       └── main.rs             # Integration tests (113 tests)
├── .github/                    # GitHub configuration
│   └── workflows/              # CI/CD workflows
│       └── ci.yml             # Continuous integration config
├── install.sh                 # Unix installation script
├── install.ps1                # Windows installation script
├── shell.sh                   # Shell integration script
├── verify.sh                  # Unix verification script
├── verify.ps1                 # Windows verification script
├── .gitignore                 # Git ignore rules
├── .pre-commit-config.yaml    # Pre-commit hook config
├── ruff.toml                  # Code linting and formatting config
├── pyproject.toml             # Python compatibility metadata
├── Makefile                   # Make-based build system
├── README.md                  # Main doc (points to CN/EN versions)
├── README_CN.md               # Chinese documentation
├── README_EN.md               # English documentation
├── LICENSE                    # MIT License
├── CHANGELOG.md               # Changelog (recommended to add)
├── CODE_OF_CONDUCT.md         # Code of conduct (recommended to add)
└── CONTRIBUTING.md            # Contributing guide
```

## 🔧 Core Modules in Detail

### 1. Core Modules (`thefuck/src/`)

> [!IMPORTANT]
> These modules form the foundation of the application:

#### `main.rs` - Application Entry Point
- CLI argument parsing
- Application main logic
- Error handling and exit code management
- Command-line interface definition

#### `types.rs` - Type Definitions
- `Command`: Input command structure
- `MatchResult`: Match result type
- `Settings`: Configuration settings structure
- Error type definitions

#### `conf.rs` - Configuration Management
- Configuration file loading and parsing
- Default value settings
- Configuration validation
- Runtime configuration updates

#### `corrector.rs` - Core Correction
- Command correction algorithm implementation
- Rule matching logic
- Similarity calculation
- Command suggestion generation

#### `system.rs` - System Integration
- Terminal processing
- Asynchronous I/O operations
- Platform-specific features
- Process management

#### `history.rs` - Command History
- Command history management
- Pattern matching
- History persistence
- Smart suggestions

#### `io.rs` - Input/Output
- Asynchronous input processing
- Output formatting
- User interaction
- Error message display

### 2. Entry Modules (`entrypoints/`)

> [!NOTE]
> These modules provide the main entry points for different functionalities:

#### `fix_command.rs` - Command Correction Main Logic
- Main correction flow
- Rule application
- User interaction
- Command execution

#### `alias.rs` - Shell Alias Generation
- Alias generation logic
- Shell-specific syntax
- Environment variable handling
- Installation script generation

#### `firstuse.rs` - First-time Use Setup
- First-run welcome
- Configuration initialization
- Dependency checking
- User guidance

### 3. Shared Utilities (`util.rs`)

> [!TIP]
> Utility functions shared across the codebase:

- `levenshtein()` / `levenshtein_ratio()` — string distance calculation for correction matching
- `SIMILARITY_*` constants — named thresholds (SIMILARITY_TYPO, SIMILARITY_SUBCOMMAND_TYPO, etc.)
- Helper constants used by corrector and rule modules

### 4. Rule System (`rules/`)

> [!WARNING]
> The rule system is responsible for correcting commands. Currently contains **31 rule modules** with **68 registered rules** across 20+ tool categories.

#### `mod.rs` - Rule Registration
- RuleRegistry with `new()` function
- Static function pointer registration for all 68 rules
- Loop-based rule iteration with name metadata

#### `helpers.rs` - Shared Rule Helpers
- `replace_first(&str, &str, &str) -> String` — replace first occurrence in a string
- `replace_part(&[&str], &str, &[&str]) -> String` — replace one part with multiple parts
- `prepend(&str, &str) -> String` — prepend a prefix to a string
- All utilities return `String` directly, avoiding intermediate `Vec<String>` allocations

#### `git.rs` - Git Rules
- Git command typo correction (e.g., `gti status` → `git status`)
- Git subcommand typo correction (e.g., `git stauts` → `git status`)
- `git checkout` → `git switch` / `git switch -c` migration suggestions
- `git push --force` → `git push --force-with-lease` safety migration

#### `python.rs` - Python Rules
- Python command typo correction (e.g., `pyrhon` → `python`)
- `pip` → `uv pip` modernization suggestion
- `pip` → `python -m pip` compatibility fallback

#### `cd.rs` - CD Command Correction
- Path spelling correction using Levenshtein distance
- Directory jump optimization
- Historical directory navigation

## 🏗️ Build System

### Cargo.toml - Rust Project Configuration
- Project metadata
- Dependency declarations
- Build configuration
- Feature flags

### Cargo.toml - Rust Project Configuration
- Project metadata, dependencies, feature flags
- Binary definitions (`thefuck`, `thefuck_firstuse`)
- Library target shared by tests and binaries

### Makefile - Build Automation
- Development tasks
- Build targets
- Test commands
- Release scripts

## 📊 Quality Assurance

> [!TIP]
> Quality is maintained through automated checks and testing at every stage of development.

### .pre-commit-config.yaml - Pre-commit Hooks
- Code formatting
- Static analysis
- Test running
- Security checks

### ruff.toml - Code Quality
- Code style rules
- Formatting configuration
- Checker settings
- Exclusion rules

### tests/ - Test Suite
- Unit tests
- Integration tests
- Performance tests
- Documentation tests

## 📦 Installation and Distribution

### Installation Scripts
- `install.sh` - Unix/macOS installation
- `install.ps1` - Windows PowerShell installation
- Environment auto-detection
- Shell alias configuration

### Verification Scripts
- `verify.sh` - Unix environment verification
- `verify.ps1` - Windows environment verification
- Functional testing
- Dependency checking

## 🚀 Extension Development

### Adding New Rules
1. Create new file in `thefuck/src/rules/`
2. Implement rule trait
3. Register in `rules/mod.rs`
4. Add test cases

### Custom Build
1. Modify `build.rs`
2. Add new build targets
3. Update Makefile
4. Configure release process

## 🔄 Data Flow

1. **Input**: User enters incorrect command
2. **Parse**: `argument_parser.rs` processes arguments
3. **Configure**: `conf.rs` loads settings
4. **Correct**: `corrector.rs` applies rule matching
5. **Display**: `io.rs` shows correction options
6. **Execute**: Selected command executes via shell
7. **History**: `history.rs` updates command history

## 🛠️ Development Best Practices

### Code Organization
- Modular design
- Clear separation of concerns
- Consistent naming conventions
- Comprehensive error handling

### Testing Strategy
- Unit tests for core logic
- Integration tests for overall functionality
- Performance tests for optimization
- Documentation tests for example code

### Documentation Maintenance
- Keep README updated
- Module documentation comments
- API documentation generation
- Complete example code

---

**Maintainer**: [HyShmily](https://github.com/HyShmily)  
**Original Project Author**: [Nikita Sivakov](https://github.com/nvbn)  
**License**: MIT

---

## 项目结构中文详解 / Chinese Project Structure Details

> [!NOTE]
> 以下是完整的项目结构说明，包含各模块的详细功能介绍。

### 📁 目录总览

> [!IMPORTANT]
> 项目根目录结构如下：

```
thefuck-upgrade/
├── thefuck/                    # Rust 核心包
│   ├── Cargo.toml              # Rust 项目配置
│   ├── src/
│   │   ├── lib.rs              # 库入口（供 tests 和二进制复用）
│   │   ├── main.rs             # thefuck 主二进制入口
│   │   ├── firstuse.rs         # thefuck_firstuse 二进制入口
│   │   ├── argument_parser.rs  # CLI 参数解析
│   │   ├── types.rs            # 核心类型定义
│   │   ├── conf.rs             # 配置管理
│   │   ├── corrector.rs        # 规则匹配与排序
│   │   ├── history.rs          # 历史记录持久化
│   │   ├── io.rs               # 交互与输出
│   │   ├── system.rs           # 终端系统初始化
│   │   ├── entrypoints.rs      # 入口模块导出
│   │   ├── entrypoints/
│   │   │   ├── alias.rs
│   │   │   ├── fix_command.rs
│   │   │   └── firstuse.rs
│   │   ├── util.rs             # 共享工具（levenshtein 等）
│   │   └── rules/
│   │       ├── mod.rs          # 规则注册表（68 条注册规则）
│   │       ├── helpers.rs      # 共享规则辅助函数
│   │       ├── cd.rs
│   │       ├── git.rs
│   │       ├── python.rs
│   │       └── ...（26 个更多规则模块）
│   └── tests/
│       └── main.rs             # 集成测试（113 个测试）
├── .github/                    # GitHub 配置
│   └── workflows/              # CI/CD 工作流
│       └── ci.yml             # 持续集成配置
├── install.sh                 # Unix 安装脚本
├── install.ps1                # Windows 安装脚本
├── shell.sh                   # Shell 集成脚本
├── verify.sh                  # Unix 验证脚本
├── verify.ps1                 # Windows 验证脚本
├── .gitignore                 # Git 忽略规则
├── .pre-commit-config.yaml    # Pre-commit 钩子配置
├── ruff.toml                  # 代码检查和格式化配置
├── pyproject.toml             # Python 兼容性元数据
├── Makefile                   # 基于 Make 的构建系统
├── README.md                  # 主文档（指向中英文版本）
├── README_CN.md               # 中文文档
├── README_EN.md               # 英文文档
├── LICENSE                    # MIT 许可证
├── CHANGELOG.md               # 更新日志（建议添加）
├── CODE_OF_CONDUCT.md         # 行为准则（建议添加）
└── CONTRIBUTING.md            # 贡献指南
```

### 🔧 核心模块详解

#### 1. 核心模块 (`thefuck/src/`)

> [!IMPORTANT]
> 这些模块构成了应用程序的基础：

##### `main.rs` - 应用程序入口
- CLI 参数解析
- 应用程序主逻辑
- 错误处理和退出码管理
- 命令行界面定义

##### `types.rs` - 类型定义
- `Command`: 输入命令结构
- `MatchResult`: 匹配结果类型
- `Settings`: 配置设置结构
- 错误类型定义

##### `conf.rs` - 配置管理
- 配置文件加载和解析
- 默认值设置
- 配置验证
- 运行时配置更新

##### `corrector.rs` - 纠错核心
- 命令纠正算法实现
- 规则匹配逻辑
- 相似度计算
- 命令建议生成

##### `system.rs` - 系统集成
- 终端处理
- 异步 I/O 操作
- 平台特定功能
- 进程管理

##### `history.rs` - 命令历史
- 历史命令管理
- 模式匹配
- 历史记录持久化
- 智能建议

##### `io.rs` - 输入输出
- 异步输入处理
- 输出格式化
- 用户交互
- 错误信息显示

#### 2. 入口模块 (`entrypoints/`)

> [!NOTE]
> 这些模块提供不同功能的主要入口点：

##### `fix_command.rs` - 命令纠正主逻辑
- 主要纠正流程
- 规则应用
- 用户交互
- 命令执行

##### `alias.rs` - Shell 别名生成
- 别名生成逻辑
- Shell 特定语法
- 环境变量处理
- 安装脚本生成

##### `firstuse.rs` - 首次使用设置
- 首次运行欢迎
- 初始化配置
- 检查依赖
- 用户引导

#### 3. 共享工具 (`util.rs`)

> [!TIP]
> 跨代码库共享的工具函数：

- `levenshtein()` / `levenshtein_ratio()` — 字符串距离计算
- `SIMILARITY_*` 常量 — 命名阈值常量
- corrector 和规则模块使用的辅助常量

#### 4. 规则系统 (`rules/`)

> [!WARNING]
> 规则系统负责纠正命令。目前包含 **31 个规则模块**、**68 条注册规则**，覆盖 20+ 工具类别。

##### `mod.rs` - 规则注册
- `RuleRegistry` 和 `new()` 函数
- 68 条规则的静态函数指针注册
- 带名称元数据的循环迭代

##### `helpers.rs` - 共享规则辅助函数
- `replace_first(&str, &str, &str) -> String` — 替换字符串中第一个匹配
- `replace_part(&[&str], &str, &[&str]) -> String` — 替换一部分为多个部分
- `prepend(&str, &str) -> String` — 在字符串前添加前缀
- 所有函数直接返回 `String`，避免中间 `Vec<String>` 分配

##### `git.rs` - Git 规则
- Git 命令拼写纠错（如 `gti status` → `git status`）
- Git 子命令拼写纠错（如 `git stauts` → `git status`）
- `git checkout` → `git switch` / `git switch -c` 迁移建议
- `git push --force` → `git push --force-with-lease` 安全迁移

##### `python.rs` - Python 规则
- Python 命令拼写纠错（如 `pyrhon` → `python`）
- `pip` → `uv pip` 现代化建议
- `pip` → `python -m pip` 兼容性兜底

##### `cd.rs` - CD 命令纠正
- 路径拼写修正（基于 Levenshtein 距离）
- 目录跳转优化
- 历史目录导航

### 🏗️ 构建系统

#### Makefile - 构建自动化
- 开发任务
- 构建目标
- 测试命令
- 发布脚本

### 📊 质量保证

> [!TIP]
> 通过在开发的每个阶段进行自动化检查和测试来保证质量。

#### .pre-commit-config.yaml - Pre-commit 钩子
- 代码格式化
- 静态分析
- 测试运行
- 安全检查

#### ruff.toml - 代码质量
- 代码风格规则
- 格式化配置
- 检查器设置
- 排除规则

#### tests/ - 测试套件
- 单元测试
- 集成测试
- 性能测试
- 文档测试

### 📦 安装和分发

#### 安装脚本
- `install.sh` - Unix/macOS 安装
- `install.ps1` - Windows PowerShell 安装
- 自动检测环境
- 配置 Shell 别名

#### 验证脚本
- `verify.sh` - Unix 环境验证
- `verify.ps1` - Windows 环境验证
- 功能测试
- 依赖检查

### 🚀 扩展开发

#### 添加新规则
1. 在 `thefuck/src/rules/` 创建新文件
2. 实现规则特征
3. 在 `rules/mod.rs` 注册
4. 添加测试用例

#### 自定义构建
1. 修改 `build.rs`
2. 添加新的构建目标
3. 更新 Makefile
4. 配置发布流程

### 🔄 数据流

1. **输入**: 用户输入错误命令
2. **解析**: `argument_parser.rs` 处理参数
3. **配置**: `conf.rs` 加载设置
4. **纠正**: `corrector.rs` 应用规则匹配
5. **显示**: `io.rs` 展示纠正选项
6. **执行**: 选择命令通过 shell 执行
7. **历史**: `history.rs` 更新命令历史

### 🛠️ 开发最佳实践

#### 代码组织
- 模块化设计
- 清晰的职责分离
- 统一的命名约定
- 完善的错误处理

#### 测试策略
- 单元测试覆盖核心逻辑
- 集成测试验证整体功能
- 性能测试确保优化
- 文档测试示例代码

#### 文档维护
- 保持 README 更新
- 模块文档注释
- API 文档生成
- 示例代码完整

---

**维护者**: [HyShmily](https://github.com/HyShmily)  
**原项目作者**: [Nikita Sivakov](https://github.com/nvbn)  
**许可证**: MIT