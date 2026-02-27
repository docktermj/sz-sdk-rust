# sz-sdk-rust

Rust trait definitions for the [Senzing](https://senzing.com/) entity resolution SDK.

This crate (`sz-sdk`) defines the public interface — traits, error types, flag
constants, and parameter constants — but contains **no concrete implementations**.

## Implementation crates

| Crate              | Description                  |
|--------------------|------------------------------|
| `sz-sdk-rust-core` | Local C library FFI bindings |
| `sz-sdk-rust-grpc` | gRPC remote access           |

## Usage

Add `sz-sdk` to your `Cargo.toml`:

```toml
[dependencies]
sz-sdk = "0.1"
```

Then import the traits you need:

```rust,ignore
use sz_sdk::{SzEngine, SzError};
```

## Traits

- **`SzEngine`** — Primary entity resolution engine (record CRUD, entity queries, why/how analysis)
- **`SzConfig`** — Configuration data source management
- **`SzConfigManager`** — Persistent configuration store; factory for `SzConfig` instances
- **`SzDiagnostic`** — Repository diagnostics and maintenance
- **`SzProduct`** — License and version information
- **`SzAbstractFactory`** — Factory for creating all component instances

## Design

All traits are **synchronous by design**, keeping the surface simple and compatible
with both FFI and gRPC backends. Async wrappers may be provided by implementation
crates.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
