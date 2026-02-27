# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

sz-sdk-rust is the **trait-definition crate** (`sz-sdk`) for the Senzing SDK in Rust.
It defines public interfaces (traits) for the Senzing entity resolution platform but contains no concrete implementations.
Licensed under Apache 2.0.

Implementations are provided by separate crates:

- `sz-sdk-rust-core` — local C library FFI bindings
- `sz-sdk-rust-grpc` — gRPC remote access

sz-sdk-rust also contains reusable components that are non-implementation specific and can be used by any of the implementations.

The C headers at `/opt/senzing/er/sdk/c/` on Linux platform define the underlying native API.

## Build Commands

```bash
cargo build              # Build the project
cargo test               # Run all tests
cargo test <test_name>   # Run a single test
cargo clippy             # Run linter
cargo fmt                # Format code
cargo doc --open         # Generate and view documentation
```

## Quality Gates

All of the following must pass before committing:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
```

CI (`.github/workflows/ci.yml`) runs these same checks on every push and PR to `main`.

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
- **`error`** (`src/error/`) — `SzError` with hierarchical `SzErrorKind` classification (16 error kinds), `SzComponent`, `SzErrorInspect` trait, and `SzResultExt` extension trait

`error` is a canonical implementation that can be used by implementation classes.

Standards:

- Senzing uses a synchronous design — all traits are sync-only by deliberate choice

## Conventions

- Trait methods use `snake_case` (Go's `AddRecord` → `add_record`)
- Flag constants use `SCREAMING_SNAKE_CASE` with `SZ_` prefix (matching C headers)
- All fallible methods return `Result<T, SzError>`
- Methods returning JSON data from the engine return `Result<String, SzError>`
- The `flags: i64` parameter on engine methods is a bitmask (see `src/flags.rs`)
- Main branch is `main`

## Crate-Level Lints

`lib.rs` enforces:

- `#![deny(unsafe_code)]` — no unsafe code allowed
- `#![warn(missing_docs)]` — public items should have doc comments
- `flags.rs` uses `#![allow(missing_docs)]` since flag constants are self-documenting by name

## Supply Chain

`deny.toml` configures `cargo-deny` for license and vulnerability auditing of dependencies.
