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
git clone https://github.com/nvbn/thefuck-upgrade.git
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
use anyhow::Result;

pub fn my_rule(command: &Command) -> Result<Option<MatchResult>> {
    // Check if command matches this rule
    if command.parts.len() < 2 || command.parts[0] != "mycommand" {
        return Ok(None);
    }

    // Check for specific error patterns
    if command.raw.contains("error") {
        Ok(Some(MatchResult {
            rule: "my_rule".to_string(),
            corrected_command: format!("corrected {}", &command.parts[1..].join(" ")),
            similarity: 0.9,
        }))
    } else {
        Ok(None)
    }
}
```

### Registering Rules

Add your rule to the rule registry:

```rust
// src/rules/mod.rs
pub mod my_rule;

// In RuleRegistry::new()
registry.register(my_rule::my_rule);
```

### Rule Guidelines

1. **Specificity**: Rules should match specific error patterns
2. **Accuracy**: Corrections should be correct and complete
3. **Performance**: Rules should be fast to execute
4. **Testing**: Include comprehensive tests

### Common Rule Patterns

```rust
// Git rules
pub fn git_rule(command: &Command) -> Result<Option<MatchResult>> {
    if command.parts[0] == "git" && command.parts[1] == "push" {
        // Handle push errors
        if let Some(output) = get_command_output(command) {
            if output.contains("non-fast-forward") {
                return Ok(Some(MatchResult {
                    rule: "git_force_push".to_string(),
                    corrected_command: "git push --force".to_string(),
                    similarity: 0.95,
                }));
            }
        }
    }
    Ok(None)
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
        
        let result = my_rule(&cmd).unwrap();
        assert!(result.is_none());

        let cmd = Command::new("mycommand --error-flag".to_string());
        let result = my_rule(&cmd).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().rule, "my_rule");
    }
}
```

### Integration Tests

Test the entire correction flow:

```rust
#[tokio::test]
async fn test_integration() {
    let cmd = Command::new("gti status".to_string());
    let mut corrector = Corrector::new(cmd).await.unwrap();
    
    let corrections = corrector.find_corrections().await.unwrap();
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
│   ├── system.rs            # System integration
│   └── shells.rs            # Shell detection
├── tests/                   # Test files
├── Cargo.toml              # Project configuration
├── ruff.toml               # Linting configuration
├── .pre-commit-config.yaml # Pre-commit hooks
└── README.md               # Project documentation
```

## Debugging

### Debug Mode

Enable debug logging:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Rest of your code
}
```

### Debug Commands

```bash
# Run with debug output
RUST_LOG=debug cargo run -- gti status

# Build debug version
cargo build
```

## Common Issues

### Missing Dependencies

If you get "could not find crate" errors:

```bash
cargo update
```

### Type Errors

For mypy errors:

```bash
cargo clippy -- -- -D warnings
```

### Build Failures

Clean build cache:

```bash
cargo clean
cargo build
```

## Community

- Join our Discord server for discussions
- Follow us on Twitter for updates
- Report bugs on GitHub Issues
- Suggest new features on GitHub Discussions

Happy coding! 🚀