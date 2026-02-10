# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

sz-sdk-rust is the **trait-definition crate** (`sz-sdk`) for the Senzing SDK in Rust. It defines public interfaces (traits) for the Senzing entity resolution platform but contains no concrete implementations. Licensed under Apache 2.0.

Implementations are provided by separate crates:
- `sz-sdk-rust-core` — local C library FFI bindings
- `sz-sdk-rust-grpc` — gRPC remote access

The Go SDK at `/home/senzing/senzing-garage.git/sz-sdk-go/` is the canonical reference for interface design. The C headers at `/opt/senzing/er/sdk/c/` define the underlying native API.

## Build Commands

```bash
cargo build              # Build the project
cargo test               # Run all tests
cargo test <test_name>   # Run a single test
cargo clippy             # Run linter
cargo fmt                # Format code
cargo doc --open         # Generate and view documentation
```

## Architecture

This crate defines 5 core traits plus a factory trait:

- **`SzEngine`** (`src/engine.rs`) — Primary entity resolution engine: record CRUD, entity queries, path/network finding, why/how analysis, export
- **`SzConfig`** (`src/config.rs`) — Configuration data source management (export, register/unregister data sources)
- **`SzConfigManager`** (`src/config_manager.rs`) — Persistent configuration store management; also acts as factory for `SzConfig` instances
- **`SzDiagnostic`** (`src/diagnostic.rs`) — Repository diagnostics and maintenance (performance checks, purge)
- **`SzProduct`** (`src/product.rs`) — Product license and version information
- **`SzAbstractFactory`** (`src/factory.rs`) — Factory pattern for creating all component instances

Supporting modules:
- **`flags`** (`src/flags.rs`) — Bitmask constants controlling response content, matching C header `libSzEngineFlags.h` and `libSzEngineFlagGroups.h`
- **`parameters`** (`src/parameters.rs`) — Common parameter constants (default config, logging, empty strings)
- **`error`** (`src/error.rs`) — `SzError` enum with 16 error variants using `thiserror`

## Conventions

- Trait methods use `snake_case` (Go's `AddRecord` → `add_record`)
- Flag constants use `SCREAMING_SNAKE_CASE` with `SZ_` prefix (matching C headers)
- All fallible methods return `Result<T, SzError>`
- Methods returning JSON data from the engine return `Result<String, SzError>`
- The `flags: i64` parameter on engine methods is a bitmask (see `src/flags.rs`)
- Main branch is `main`
