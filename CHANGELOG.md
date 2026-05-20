# Changelog

All notable changes to this project are documented in this file.

## [3.34.0] - 2026-05-20

### Performance
- **Lazy parts cloning**: Created `rules/helpers.rs` with `replace_first`, `replace_part`, `prepend` utilities, eliminating intermediate `Vec<String>` allocations across all 31 rule files. All rules now build corrected commands directly without cloning `command.parts`.
- **Sync history I/O**: `history::add_command` changed from `async` to synchronous — it performed purely synchronous file I/O with no `.await` calls.

### Bug Fixes
- Fixed pacman self-referencing typo: `-Syu` was listed as both a valid subcommand and a typo of itself in `PACMAN_SUBCOMMAND_TYPOS`.
- Fixed duplicate `-Syu` entry in `PACMAN_SUBCOMMANDS`.
- Removed dead code from `pip.rs`: `PIP_SUBCOMMAND_TYPOS` and `pip_subcommand_typo_rule` were verbatim duplicates of `python.rs` versions and never registered in `RuleRegistry`.

### Testing
- Expanded test coverage from 87 to 113 tests (+26 new tests).
- Added comprehensive tests for all 13 previously uncovered rule files: aws, az, bun, choco, dnf, gradle, grep, pacman, pip, pnpm, uv, winget.
- Each uncovered file now has typo correction, subcommand typo, and negative case tests.

## [3.33.0] - 2026-04-24

### Added
- Modern Git migration suggestions from `git checkout` to `git switch`.
- Support for branch creation migration from `git checkout -b` to `git switch -c`.
- Python package workflow modernization suggestions from `pip` / `pip3` to `uv pip` for common package-management commands.
- Compatibility fallback suggestions from `pip` / `pip3` to `python -m pip` for safer execution in older environments.
- Docker legacy command migration suggestions for `docker images` and `docker ps`.
- Expanded integration test coverage for the new modernization rules.

### Changed
- Updated the rule registry to include the new modernization rules for Git, Python, and Docker.
- Updated project version metadata to `3.33.0` in both the Rust crate and Python packaging metadata.
- Synchronized the README files with the 3.33.0 release highlights.
- Standardized terminal-facing script output to English where installation and verification scripts display messages.
- Improved documentation callouts using `> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, and `> [!WARNING]` to make guidance easier to scan.

### Fixed
- Removed duplicated Git checkout-to-switch rule wiring introduced during iterative refactoring.
- Restored and stabilized modern command correction coverage for `git`, `pip`, and `docker` workflows.
- Strengthened regression coverage around command migration suggestions and backward-compatible fallbacks.
- Cleaned up release documentation drift so the README family now matches the actual 3.33.0 behavior.

## [3.32.0] - 2026-04-24

- Previous release snapshot retained for historical comparison.