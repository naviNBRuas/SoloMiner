# SoloMiner Development Phases

This document outlines the phased development plan for the SoloMiner project.

## Phase 1: Core Engine & CLI (Proof of Concept) - **COMPLETED**

*   **Technology Stack:**
    *   **Language:** Rust
    *   **CLI:** `clap`
    *   **Web Dashboard:** `actix-web`
    *   **Configuration:** `dotenv`
*   **Tasks:**
    *   Initialize a new Rust project with a modular structure (`core`, `orchestrator`, `telemetry`).
    *   Create a basic CLI for starting/stopping the miner and configuration.
    *   Implement a simulated mining loop for the SHA-256 algorithm.
    *   Set up the `.env` file for basic configuration (e.g., a dummy wallet address).

## Phase 2: Web Dashboard & Telemetry - **COMPLETED**

*   **Tasks:**
    *   Integrate `actix-web` to serve a simple web dashboard.
    *   Display real-time (simulated) hashrate, system load, and other metrics on the dashboard.

## Phase 3: Orchestrator & Resource Governor - **COMPLETED**

*   **Tasks:**
    *   Implement basic hardware detection (e.g., CPU cores).
    *   Add logic to the orchestrator to manage the number of (simulated) mining threads.
    *   Introduce "performance" and "conservative" modes to adjust resource usage.
    *   **Multi-threading for mining tasks implemented.**

## Phase 4: Plugin System & Algorithm Expansion - **COMPLETED**

*   **Tasks:**
    *   Design a plugin architecture for adding new mining algorithms.
    *   Add a second simulated algorithm (e.g., RandomX) as a proof of concept for the plugin system.

## Future Goals

*   Full implementation of various mining algorithms.
*   Advanced hardware monitoring (GPU, thermals).
*   Native GUI development.
*   Containerization with Docker.
*   Remote management capabilities.