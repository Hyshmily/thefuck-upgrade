# Contributing to The Fuck

Thank you for your interest in contributing to The Fuck! This document will guide you through the process of contributing to the project.

## Table of Contents

- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Writing Rules](#writing-rules)
- [Testing](#testing)
- [Submitting Pull Requests](#submitting-pull-requests)
- [Project Structure](#project-structure)

## Development Setup

### Prerequisites

- Rust 1.70+
- Cargo
- Git

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/HyShmily/thefuck-upgrade.git
cd thefuck-upgrade

# Build the project
cd thefuck
cargo build

# Run tests
cargo test

# Run the application
cargo run -- gti status
```

### Development Tools

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests with coverage
cargo test -- --nocapture

# Build documentation
cargo doc --open
```

## Code Style

### Rust Code Style

- Follow the Rust naming conventions
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants

### Comments

- Write clear, concise comments
- Document public APIs with doc comments
- Explain complex algorithms, not obvious code

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `anyhow::Result` for application-level errors
- Provide meaningful error messages

```rust
// Good
fn process_command(cmd: &str) -> anyhow::Result<String> {
    // ... implementation
}

// Bad
fn process_command(cmd: &str) -> Result<String, String> {
    // ... implementation
}
```

## Writing Rules

### Rule Structure

A rule in The Fuck is a function that takes a command and returns a correction.

```rust
use crate::types::{Command, MatchResult};

pub fn my_rule(command: &Command) -> Option<MatchResult> {
    // Check if command matches this rule
    if command.parts.len() < 2 || command.parts[0] != "mycommand" {
        return None;
    }

    // Check for specific error patterns
    if command.raw.contains("error") {
        Some(MatchResult {
            rule: "my_rule",
            corrected_command: format!("corrected {}", &command.parts[1..].join(" ")),
            similarity: 0.9,
        })
    } else {
        None
    }
}
```

### Registering Rules

Add your rule to the rule registry:

```rust
// In src/rules/mod.rs:
// 1. Add `pub mod my_rule;` at the top
// 2. Add a RuleDefinition entry in RuleRegistry::new():
RuleDefinition {
    name: "my_rule",
    apply: my_rule::my_rule,
},
```

### Rule Guidelines

1. **Specificity**: Rules should match specific error patterns
2. **Accuracy**: Corrections should be correct and complete
3. **Performance**: Rules should be fast to execute
4. **Testing**: Include comprehensive tests

### Common Rule Patterns

```rust
// Git rules
pub fn git_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "git" {
        return None;
    }

    match command.parts[1].as_str() {
        "psuh" | "pus" => Some(MatchResult {
            rule: "git_subcommand_typo",
            corrected_command: command.parts.iter().enumerate()
                .map(|(i, p)| if i == 1 { "push".to_string() } else { p.clone() })
                .collect::<Vec<_>>().join(" "),
            similarity: 0.96,
        }),
        _ => None,
    }
}
```

## Testing

### Unit Tests

Write tests for each rule:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Command;

    #[test]
    fn test_my_rule() {
        let cmd = Command::new("mycommand --flag".to_string());
        assert!(my_rule(&cmd).is_none());

        let cmd = Command::new("mycommand --error-flag".to_string());
        let result = my_rule(&cmd);
        assert!(result.is_some());
        assert_eq!(result.unwrap().rule, "my_rule");
    }
}
```

### Integration Tests

Test the entire correction flow:

```rust
use thefuck::types::{Command, Settings};
use thefuck::corrector::Corrector;

#[test]
fn test_integration() {
    let cmd = Command::new("gti status".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(cmd, settings);

    let corrections = corrector.find_corrections();
    assert!(!corrections.is_empty());
    assert_eq!(corrections[0].rule, "git_command");
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test my_rule

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

## Submitting Pull Requests

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Update documentation if needed
7. Submit a pull request

### PR Checklist

- [ ] Tests are included and passing
- [ ] Code follows the project style guide
- [ ] Documentation is updated
- [ ] PR title follows the format: "feat: add new rule" or "fix: correct git command"
- [ ] Description explains the changes

### PR Template

```markdown
## Summary
- Added new rule for [tool] command corrections
- Fixed issue with [specific problem]
- Improved performance by [optimization]

## Changes Made
- Added [rule_name] rule
- Updated [existing_rule] to handle [new case]
- Added [number] test cases

## Testing
- Manual testing: [describe what you tested]
- Unit tests: [describe what tests were added]
```

## Project Structure

```
thefuck/
├── src/
│   ├── main.rs              # Main entry point
│   ├── types.rs             # Type definitions
│   ├── conf.rs              # Configuration handling
│   ├── argument_parser.rs   # Command line parsing
│   ├── corrector.rs         # Core correction logic
│   ├── entrypoints/         # Entry points
│   │   ├── mod.rs
│   │   ├── fix_command.rs
│   │   └── alias.rs
│   ├── rules/               # Rule implementations
│   │   ├── mod.rs
│   │   ├── git.rs
│   │   └── ...
│   ├── io.rs                # Input/output handling
│   ├── history.rs           # Command history
│   └── util.rs              # Shared utilities (Levenshtein, constants)
├── tests/                   # Test files
├── Cargo.toml              # Project configuration
└── README.md               # Project documentation
```

## Debugging

### Debug Commands

```bash
# Build and run in debug mode
cargo run -- gti status

# Build debug version
cargo build

# Run tests with output
cargo test -- --nocapture
```

## Common Issues

### Missing Dependencies

If you get "could not find crate" errors:

```bash
cargo update
```

### Type Errors

Run clippy to catch type and lint issues:

```bash
cargo clippy -- -D warnings
```

### Build Failures

Clean build cache:

```bash
cargo clean
cargo build
```

## Community

- Report bugs on GitHub Issues
- Suggest new features on GitHub Discussions

Happy coding! 🚀
