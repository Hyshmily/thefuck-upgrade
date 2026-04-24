# Changelog

All notable changes to this project are documented in this file.

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