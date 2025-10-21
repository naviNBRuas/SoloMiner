# SoloMiner

A professional, high-performance, cross-platform SoloMiner software.

SoloMiner is a fully modular, scalable, and intelligent solo mining engine that can autonomously mine Bitcoin and other mineable cryptocurrencies. It is designed for efficiency, adaptability, and optimization per system hardware profile.

## Features

*   **Cross-Platform:** Runs on Windows, macOS, Linux, Android, and iOS.
*   **Multiple Architectures:** Supports x86, x86_64, ARM, and ARM64.
*   **Modular Engine:** Plugin-based system for adding new mining algorithms.
*   **Orchestrator:** Automatically manages mining instances based on hardware detection.
*   **Resource Governor:** Dynamically adjusts CPU/GPU utilization with "performance" and "conservative" modes.
*   **Web Dashboard:** Real-time monitoring of hashrate, system load, and other metrics.

## Development Plan

The development of SoloMiner is divided into several phases. You can find the detailed development plan in the [development_phases.md](development_phases.md) file.

## Getting Started

*To be updated as development progresses.*

### Prerequisites

*   Rust (latest stable version)

### Building

```bash
cargo build --release
```

### Running

```bash
./target/release/solominer --help
```

## Configuration

SoloMiner uses a `config.toml` file for application-wide settings. An example `config.toml` is provided in the project root.

```toml
# SoloMiner Configuration File

[miner]
difficulty = "0000"

[logging]
level = "info"
```

*   `miner.difficulty`: Sets the mining difficulty (e.g., number of leading zeros in the hash).
*   `logging.level`: Sets the logging level (e.g., "info", "debug", "error").

Additionally, the `WALLET_ADDRESS` is configured via a `.env` file in the project root.

## Docker Support

To build and run SoloMiner using Docker:

```bash
docker build -t solominer .
docker run -p 8080:8080 solominer start --algorithm sha256 --mode performance
# Or to run the dashboard
docker run -p 8080:8080 solominer dashboard
```

## Cross-Compilation

SoloMiner supports cross-compilation for various platforms. To cross-compile, you need to add the desired target and install the corresponding toolchain. For example, to compile for `aarch64-unknown-linux-gnu`:

```bash
rustup target add aarch64-unknown-linux-gnu
# Install the C/C++ toolchain for the target if needed (e.g., for OpenSSL dependencies)
sudo apt install gcc-aarch64-linux-gnu

cargo build --release --target aarch64-unknown-linux-gnu
```

Refer to the Rust documentation for more details on cross-compilation for specific targets.

## Release Process

To create a new release of SoloMiner, follow these steps:

1.  **Update Version:** Update the `version` field in `Cargo.toml`.
2.  **Build Release Binaries:** Build the project for all desired targets using cross-compilation (see "Cross-Compilation" section).
    ```bash
cargo build --release --all-targets
    ```
3.  **Run Tests:** Ensure all tests pass.
    ```bash
cargo test --all-targets
    ```
4.  **Create Docker Image:** Build and tag the Docker image.
    ```bash
docker build -t solominer:vX.Y.Z .
    ```
5.  **Create Release Notes:** Document all changes, new features, and bug fixes in a `CHANGELOG.md` (to be created).
6.  **Tag Release:** Create a Git tag for the new version.
    ```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push --tags
    ```
7.  **Publish Assets:** Upload the release binaries and Docker image to appropriate platforms.
