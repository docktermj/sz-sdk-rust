//! Rust trait definitions for the Senzing SDK.
//!
//! This crate defines the public interface for the Senzing entity resolution
//! SDK in Rust. It contains only trait definitions, error types, flag constants,
//! and parameter constants — no concrete implementations.
//!
//! Implementations are provided by separate crates:
//! - `sz-sdk-rust-core` — local C library FFI bindings
//! - `sz-sdk-rust-grpc` — gRPC remote access
//!
//! # Design note
//!
//! All traits in this crate are **synchronous**. This is a deliberate choice
//! that keeps the trait surface simple and compatible with both FFI and gRPC
//! backends. Async wrappers can be added by implementation crates if needed.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod config;
mod config_manager;
mod diagnostic;
mod engine;
mod error;
mod factory;
pub mod flags;
pub mod parameters;
mod product;

pub use config::SzConfig;
pub use config_manager::SzConfigManager;
pub use diagnostic::SzDiagnostic;
pub use engine::SzEngine;
pub use error::{SzComponent, SzError, SzErrorInspect, SzErrorKind, SzResult, SzResultExt};
pub use factory::SzAbstractFactory;
pub use product::SzProduct;
