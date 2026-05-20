# The Fuck - Project Structure Documentation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> [!NOTE]
> This document provides a detailed introduction to the project structure and module design of The Fuck Rust rewrite version.

## Project Overview

## What's New in 3.34.0

> [!NOTE]
> Version 3.34.0 focuses on performance optimization, code quality improvements, and comprehensive test coverage.

- ⚡ **Performance**: Created `rules/helpers.rs` with `replace_first`, `replace_part`, `prepend` utilities. All 31 rule files now use lazy parts cloning, eliminating intermediate `Vec<String>` allocations.
- 🧹 **Code cleanup**: Removed dead code duplication in `pip.rs` (unused `PIP_SUBCOMMAND_TYPOS` and `pip_subcommand_typo_rule`), fixed pacman self-referencing typo, made `history::add_command` synchronous.
- ✅ **Test coverage**: Added 26 new tests covering all 13 previously untested rule files — now **113 tests total**.
- 📦 **Rule system**: 31 module files, **68 registered rules** with shared helper utilities.

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
│   │   ├── util.rs             # Shared utilities (levenshtein, SIMILARITY constants)
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

## Core Modules in Detail

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

#### `shells.rs` - Shell Support
- Shell type detection (Bash/Zsh/Fish/PowerShell)
- Shell-specific features
- Platform adaptation
- Path handling

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

#### `output_readers.rs` - Output Reading
- Command output parsing
- Error message extraction
- Pattern matching
- Data extraction utilities

#### `exit_codes.rs` - Exit Code Definitions
- Standard exit codes
- Error type codes
- Success status codes
- Custom exit codes

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

#### `firstuse.rs` - First-time Use Setup
- First-run welcome
- Configuration initialization
- User guidance

#### `update.rs` - Update Instructions
- Platform-specific update guide
- Cargo, Homebrew, APT, Pacman instructions

#### `delete.rs` - Uninstall / Cleanup
- History and data directory removal
- Shell alias removal instructions
- Platform-specific uninstall guide

### 3. Rule System (`rules/`)

> [!WARNING]
> The rule system is responsible for correcting commands. Adding new rules requires implementing the rule trait.

#### `mod.rs` - Rule Registration
- Rule trait definitions
- Rule registry
- Rule loading
- 20+ tool categories, 68 registered rules across 31 modules

#### `common.rs` - Common Command Typos
- `sl→ls`, `gerp→grep`, `mkae→make`, `ehco→echo`, `chomd→chmod`, `vom→vim`, and more

#### `git.rs` - Git Rules
- Git command correction
- Branch operation correction
- Subcommand typo correction
- Push/pull workflow fixes

#### `python.rs` - Python Rules
- Python command correction
- pip to uv migration
- pip to python -m pip fallback
- pip subcommand typo correction

#### `docker.rs` - Docker Rules
- Docker command typo correction
- docker-compose v2 migration
- Legacy management command migration
- Docker subcommand typo correction

#### `npm.rs` - Node.js Rules
- npm/yarn/pnpm command typo correction
- npm/yarn subcommand typo correction

#### `cargo.rs` - Cargo Rules
- Cargo command typo correction
- Cargo subcommand typo correction

#### `go.rs` - Go Rules
- Go command typo correction
- Go subcommand typo correction

#### `brew.rs` - Homebrew Rules
- brew command typo correction
- brew subcommand typo correction

#### `apt.rs` - APT Rules
- apt/apt-get command typo correction
- apt/apt-get subcommand typo correction
- `apt-get→apt` migration suggestion

#### `systemctl.rs` - systemd Rules
- systemctl command typo correction
- systemctl subcommand typo correction

#### `kubectl.rs` - Kubernetes Rules
- kubectl command typo correction
- kubectl subcommand typo correction

#### `terraform.rs` - Terraform Rules
- terraform command typo correction
- terraform subcommand typo correction

#### `conda.rs` - Conda Rules
- Conda command typo correction
- Conda subcommand typo correction

#### `maven.rs` - Maven Rules
- Maven command typo correction
- Maven subcommand typo correction

#### `pip.rs` - Pip Rules
- pip command typo correction

#### `cd.rs` - CD Command Correction
- Path spelling correction

#### `sudo.rs` - Sudo Rules
- Missing sudo detection for package managers
- Extended detection for `make install`, `npm install -g`

#### `missing_space.rs` - Missing Space / Wrong Hyphen
- `cd..→cd ..`
- `git-log→git log`, `<cmd>-<subcmd>→<cmd> <subcmd>`

## Build System

### Cargo.toml - Rust Project Configuration
- Project metadata
- Dependency declarations
- Build configuration
- Feature flags

### build.rs - Build Script
- Rule compilation
- Resource processing
- Build-time checks
- Custom build steps

### Makefile - Build Automation
- Development tasks
- Build targets
- Test commands
- Release scripts

## Quality Assurance

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

## Installation and Distribution

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

## Extension Development

### Adding New Rules

> [!IMPORTANT]
> Write tests before registering a new rule. That keeps matching behavior easier to verify and roll back.

1. Create new file in `thefuck/src/rules/`
2. Implement rule trait
3. Register in `rules/mod.rs`
4. Add test cases

### Custom Build
1. Build with `cargo build` (debug) or `cargo build --release`
2. Add new build targets
3. Update Makefile
4. Configure release process

## Data Flow

1. **Input**: User enters incorrect command
2. **Parse**: `argument_parser.rs` processes arguments
3. **Configure**: `conf.rs` loads settings
4. **Correct**: `corrector.rs` applies rule matching
5. **Display**: `io.rs` shows correction options
6. **Execute**: Selected command executes via shell
7. **History**: `history.rs` updates command history

## Development Best Practices

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