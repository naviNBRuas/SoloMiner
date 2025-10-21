# Changelog

## 0.1.0 - 2023-10-26

### Added

- Initial project structure.
- CLI with `start`, `stop`, `status`, and `dashboard` commands.
- Simulated SHA-256 and RandomX mining algorithms.
- Multi-threading support for mining tasks.
- Web dashboard for real-time metrics.
- Configuration loading from `config.toml` and `.env`.
- Docker support.
- Cross-compilation support.

### Changed

- Refactored `MinerAlgorithm` trait for better multi-threading support.
- Updated `start_mining` function to support continuous mining and timeouts for testing.

### Fixed

- Corrected module structure for tests.

