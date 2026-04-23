# The Fuck - 项目结构文档

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> [!NOTE]
> 本文档详细介绍了 The Fuck Rust 重写版本的项目结构和模块设计。

## 项目概述

> [!IMPORTANT]
> 以下目录结构展示了项目的完整布局：

```
thefuck-upgrade/
├── thefuck/                    # Rust 核心包
│   ├── Cargo.toml              # Rust 项目配置
│   ├── build.rs                # 构建脚本（规则文件扫描）
│   ├── src/
│   │   ├── lib.rs              # 库入口（供 tests 和二进制复用）
│   │   ├── main.rs             # Thefuck 主二进制入口
│   │   ├── firstuse.rs         # Thefuck_firstuse 二进制入口
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
│   │   └── rules/
│   │       ├── mod.rs
│   │       ├── git.rs
│   │       ├── python.rs
│   │       └── cd.rs
│   └── tests/
│       └── main.rs             # 集成测试
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

## 核心模块详解

### 1. 核心模块 (`thefuck/src/`)

> [!IMPORTANT]
> 这些模块构成了应用程序的基础：

#### `main.rs` - 应用程序入口
- CLI 参数解析
- 应用程序主逻辑
- 错误处理和退出码管理
- 命令行界面定义

#### `types.rs` - 类型定义
- `Command`: 输入命令结构
- `MatchResult`: 匹配结果类型
- `Settings`: 配置设置结构
- 错误类型定义

#### `conf.rs` - 配置管理
- 配置文件加载和解析
- 默认值设置
- 配置验证
- 运行时配置更新

#### `corrector.rs` - 纠错核心
- 命令纠正算法实现
- 规则匹配逻辑
- 相似度计算
- 命令建议生成

#### `system.rs` - 系统集成
- 终端处理
- 异步 I/O 操作
- 平台特定功能
- 进程管理

#### `shells.rs` - Shell 支持
- Shell 类型检测（Bash/Zsh/Fish/PowerShell）
- Shell 特定功能
- 平台适配
- 路径处理

#### `history.rs` - 命令历史
- 历史命令管理
- 模式匹配
- 历史记录持久化
- 智能建议

#### `io.rs` - 输入输出
- 异步输入处理
- 输出格式化
- 用户交互
- 错误信息显示

#### `output_readers.rs` - 输出读取
- 命令输出解析
- 错误信息提取
- 模式匹配
- 数据提取工具

#### `exit_codes.rs` - 退出码定义
- 标准退出码
- 错误类型码
- 成功状态码
- 自定义退出码

### 2. 入口模块 (`entrypoints/`)

> [!NOTE]
> 这些模块提供不同功能的主要入口点：

#### `fix_command.rs` - 命令纠正主逻辑
- 主要纠正流程
- 规则应用
- 用户交互
- 命令执行

#### `alias.rs` - Shell 别名生成
- 别名生成逻辑
- Shell 特定语法
- 环境变量处理
- 安装脚本生成

#### `firstuse.rs` - 首次使用设置
- 首次运行欢迎
- 初始化配置
- 检查依赖
- 用户引导

### 3. 规则系统 (`rules/`)

> [!WARNING]
> 规则系统负责纠正命令。添加新规则需要实现规则特征。

#### `mod.rs` - 规则注册
- 规则特征定义
- 规则注册表
- 规则加载
- 特征实现

#### `git.rs` - Git 规则
- Git 命令纠错
- 分支操作纠正
- 提交信息修正
- 远程操作处理

#### `python.rs` - Python 规则
- Python 命令纠正
- 包管理器支持
- 虚拟环境处理
- 模块导入修正

#### `cd.rs` - CD 命令纠正
- 路径拼写修正
- 目录跳转优化
- 自动补全
- 历史目录导航

## 构建系统

### Cargo.toml - Rust 项目配置
- 项目元数据
- 依赖声明
- 构建配置
- 特性标志

### build.rs - 构建脚本
- 规则编译
- 资源处理
- 构建时检查
- 自定义构建步骤

### Makefile - 构建自动化
- 开发任务
- 构建目标
- 测试命令
- 发布脚本

## 质量保证

> [!TIP]
> 通过在开发的每个阶段进行自动化检查和测试来保证质量。

### .pre-commit-config.yaml - Pre-commit 钩子
- 代码格式化
- 静态分析
- 测试运行
- 安全检查

### ruff.toml - 代码质量
- 代码风格规则
- 格式化配置
- 检查器设置
- 排除规则

### tests/ - 测试套件
- 单元测试
- 集成测试
- 性能测试
- 文档测试

## 安装和分发

### 安装脚本
- `install.sh` - Unix/macOS 安装
- `install.ps1` - Windows PowerShell 安装
- 自动检测环境
- 配置 Shell 别名

### 验证脚本
- `verify.sh` - Unix 环境验证
- `verify.ps1` - Windows 环境验证
- 功能测试
- 依赖检查

## 扩展开发

### 添加新规则
1. 在 `thefuck/src/rules/` 创建新文件
2. 实现规则特征
3. 在 `rules/mod.rs` 注册
4. 添加测试用例

### 自定义构建
1. 修改 `build.rs`
2. 添加新的构建目标
3. 更新 Makefile
4. 配置发布流程

## 数据流

1. **输入**: 用户输入错误命令
2. **解析**: `argument_parser.rs` 处理参数
3. **配置**: `conf.rs` 加载设置
4. **纠正**: `corrector.rs` 应用规则匹配
5. **显示**: `io.rs` 展示纠正选项
6. **执行**: 选择命令通过 shell 执行
7. **历史**: `history.rs` 更新命令历史

## 开发最佳实践

### 代码组织
- 模块化设计
- 清晰的职责分离
- 统一的命名约定
- 完善的错误处理

### 测试策略
- 单元测试覆盖核心逻辑
- 集成测试验证整体功能
- 性能测试确保优化
- 文档测试示例代码

### 文档维护
- 保持 README 更新
- 模块文档注释
- API 文档生成
- 示例代码完整

---

**维护者**: [HyShmily](https://github.com/HyShmily)  
**原项目作者**: [Nikita Sivakov](https://github.com/nvbn)  
**许可证**: MIT