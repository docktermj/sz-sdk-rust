//! Error types for the Senzing SDK.
//!
//! This module provides [`SzError`], [`SzErrorKind`], and [`SzComponent`] for
//! constructing, classifying, and inspecting Senzing errors.  It also provides
//! the [`SzErrorInspect`] trait for walking error chains and the
//! [`SzResultExt`] trait for ergonomic result handling.
//!
//! # Creating errors with named constructors
//!
//! Named constructors are the most readable way to create an error with a
//! specific kind.  Each constructor sets `kind_explicit`, so a subsequent
//! [`with_code`](SzError::with_code) call stores the code without overriding
//! the kind.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind, SzComponent};
//!
//! // Simple named constructor
//! let err = SzError::not_found("entity 42");
//! assert_eq!(err.kind(), SzErrorKind::NotFound);
//! assert_eq!(err.message(), "entity 42");
//!
//! // Chain with builder methods for full detail
//! let err = SzError::database_transient("deadlock detected")
//!     .with_code(1008)
//!     .with_component(SzComponent::Engine);
//! assert_eq!(err.kind(), SzErrorKind::DatabaseTransient);
//! assert_eq!(err.code(), Some(1008));
//! assert_eq!(err.component(), Some(SzComponent::Engine));
//! ```
//!
//! # Creating errors from numeric codes
//!
//! When you have a Senzing error code (e.g. from the C SDK), use
//! [`SzError::new`] with [`with_code`](SzError::with_code).  The kind is
//! derived automatically from the code.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind};
//!
//! let err = SzError::new("connection dropped").with_code(1006);
//! assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
//! assert!(err.is_retryable());
//! ```
//!
//! # Hierarchy-aware error classification
//!
//! [`SzErrorKind`] forms a tree.  The convenience predicates on [`SzError`]
//! and the [`SzErrorKind::is`] method honor the hierarchy so you can match
//! on parent categories.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind};
//!
//! let err = SzError::database_connection_lost("server unreachable");
//!
//! // Leaf kind
//! assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
//!
//! // Parent categories
//! assert!(err.is_retryable());                           // convenience predicate
//! assert!(err.is(SzErrorKind::Retryable));               // hierarchy-aware check
//! assert!(err.is(SzErrorKind::SzError));                 // root matches everything
//! assert!(!err.is(SzErrorKind::BadInput));                // wrong branch
//! ```
//!
//! # Handling errors with match
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind, SzResult};
//!
//! fn process() -> SzResult<String> {
//!     Err(SzError::not_found("record 99"))
//! }
//!
//! match process() {
//!     Ok(value) => println!("{value}"),
//!     Err(ref e) if e.is_retryable() => println!("retry: {e}"),
//!     Err(ref e) if e.is_bad_input() => println!("bad input: {e}"),
//!     Err(e) => println!("other error: {e}"),
//! }
//! ```
//!
//! # Inspecting wrapped errors with `SzErrorInspect`
//!
//! When an `SzError` is wrapped inside another error type (e.g. `anyhow` or a
//! custom wrapper), [`SzErrorInspect`] walks the `.source()` chain to find it.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind, SzErrorInspect};
//!
//! let err: Box<dyn std::error::Error> =
//!     Box::new(SzError::database_transient("deadlock"));
//! assert!(err.is_sz_retryable());
//! assert!(err.is_sz(SzErrorKind::DatabaseTransient));
//! ```
//!
//! # Recovering from retryable errors with `SzResultExt`
//!
//! [`SzResultExt`] adds helper methods to [`SzResult<T>`] for inline retry
//! and filter logic.
//!
//! ```
//! use sz_sdk::{SzError, SzResult, SzResultExt};
//!
//! let result: SzResult<String> =
//!     Err(SzError::database_transient("deadlock"));
//!
//! // or_retry: recover from retryable errors inline
//! let recovered = result.or_retry(|_| Ok("recovered".into()));
//! assert_eq!(recovered.unwrap(), "recovered");
//!
//! // filter_retryable: convert retryable errors to Ok(None)
//! let result2: SzResult<String> =
//!     Err(SzError::database_transient("deadlock"));
//! let filtered = result2.filter_retryable();
//! assert_eq!(filtered.unwrap(), None);
//! ```
//!
//! # Retry loop with backoff
//!
//! A realistic retry pattern using [`SzResult`], [`is_retryable()`](SzError::is_retryable),
//! attempt counting, and exponential backoff.
//!
//! ```no_run
//! use sz_sdk::{SzError, SzResult};
//!
//! fn add_record(engine: &mut dyn sz_sdk::SzEngine, record: &str) -> SzResult<()> {
//!     let max_attempts = 5;
//!     for attempt in 1..=max_attempts {
//!         match engine.add_record("CUSTOMERS", "1001", record, 0) {
//!             Ok(info) => {
//!                 let _ = info; // process info if needed
//!                 return Ok(());
//!             }
//!             Err(e) if e.is_retryable() && attempt < max_attempts => {
//!                 let backoff = std::time::Duration::from_millis(100 * 2_u64.pow(attempt - 1));
//!                 eprintln!("attempt {attempt}/{max_attempts} failed ({}), retrying…", e.kind());
//!                 std::thread::sleep(backoff);
//!             }
//!             Err(e) => return Err(e),
//!         }
//!     }
//!     unreachable!()
//! }
//! ```
//!
//! # Handling Senzing errors in mixed-error functions
//!
//! When a function returns `Box<dyn Error>` because it calls both Senzing
//! and non-Senzing operations, use [`SzErrorInspect`] at the call site to
//! classify the error.
//!
//! ```no_run
//! use sz_sdk::{SzError, SzErrorInspect, SzErrorKind};
//! use std::error::Error;
//!
//! /// Loads a record from a file and adds it to the engine.
//! fn load_record(
//!     engine: &mut dyn sz_sdk::SzEngine,
//!     path: &str,
//! ) -> Result<(), Box<dyn Error>> {
//!     let record = std::fs::read_to_string(path)?;         // io::Error
//!     engine.add_record("DS", "1", &record, 0)?;           // SzError
//!     Ok(())
//! }
//!
//! // At the call site, classify the error:
//! # fn example(engine: &mut dyn sz_sdk::SzEngine) {
//! match load_record(engine, "record.json") {
//!     Ok(()) => println!("loaded"),
//!     Err(ref e) if e.is_sz_retryable() => eprintln!("retryable: {e}"),
//!     Err(ref e) if e.is_sz_bad_input() => eprintln!("bad input: {e}"),
//!     Err(ref e) if e.is_sz_error() => eprintln!("other Senzing error: {e}"),
//!     Err(e) => eprintln!("non-Senzing error: {e}"),
//! }
//! # }
//! ```
//!
//! # Custom error enum with `SzErrorInspect`
//!
//! [`SzErrorInspect`] is implemented for all `E: Error + 'static`, so it
//! works on custom error enums without any extra plumbing — as long as the
//! enum's [`source()`](std::error::Error::source) method exposes the inner
//! error.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind, SzErrorInspect};
//! use std::fmt;
//!
//! #[derive(Debug)]
//! enum AppError {
//!     Senzing(SzError),
//!     Io(std::io::Error),
//! }
//!
//! impl fmt::Display for AppError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         match self {
//!             AppError::Senzing(e) => write!(f, "senzing: {e}"),
//!             AppError::Io(e) => write!(f, "io: {e}"),
//!         }
//!     }
//! }
//!
//! impl std::error::Error for AppError {
//!     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//!         match self {
//!             AppError::Senzing(e) => Some(e),
//!             AppError::Io(e) => Some(e),
//!         }
//!     }
//! }
//!
//! // SzErrorInspect walks through AppError's source chain automatically
//! let err = AppError::Senzing(SzError::not_found("entity 42"));
//! assert!(err.is_sz_bad_input());
//! assert!(err.is_sz(SzErrorKind::NotFound));
//!
//! let err = AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "gone"));
//! assert!(!err.is_sz_error()); // no SzError in the chain
//! ```
//!
//! # Inspecting error details for logging
//!
//! Extract code, message, severity, category, and hierarchy from an
//! [`SzError`] for structured logging.
//!
//! ```
//! use sz_sdk::{SzError, SzErrorKind, SzComponent};
//!
//! let err = SzError::database_connection_lost("server unreachable")
//!     .with_code(1006)
//!     .with_component(SzComponent::Engine);
//!
//! // Structured fields for logging
//! assert_eq!(err.code(), Some(1006));
//! assert_eq!(err.message(), "server unreachable");
//! assert_eq!(err.severity(), "medium");
//! assert_eq!(err.category(), "database_connection_lost");
//! assert_eq!(err.component(), Some(SzComponent::Engine));
//! assert_eq!(
//!     err.hierarchy(),
//!     &[SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable],
//! );
//!
//! // Example: emit as a structured log line
//! let log_line = format!(
//!     "code={} severity={} category={} component={} msg={}",
//!     err.code().unwrap_or(0),
//!     err.severity(),
//!     err.category(),
//!     err.component_name(),
//!     err.message(),
//! );
//! assert!(log_line.contains("severity=medium"));
//! assert!(log_line.contains("component=SzEngine"));
//! ```

pub(crate) mod errortypes;
#[cfg(test)]
mod tests;

use errortypes::{SzError as SzErrorType, SZ_ERROR_TYPES};
use std::fmt;
use std::sync::Arc;

/// Identifies which Senzing SDK component produced an error.
///
/// Mirrors the five core subsystems of the Senzing SDK.  Implementations
/// set this when constructing an [`SzError`] so callers can determine
/// *which* component failed without parsing the error message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SzComponent {
    /// The configuration component ([`SzConfig`](crate::SzConfig)).
    Config,
    /// The configuration manager component ([`SzConfigManager`](crate::SzConfigManager)).
    ConfigManager,
    /// The diagnostic component ([`SzDiagnostic`](crate::SzDiagnostic)).
    Diagnostic,
    /// The entity resolution engine component ([`SzEngine`](crate::SzEngine)).
    Engine,
    /// The product information component ([`SzProduct`](crate::SzProduct)).
    Product,
}

impl SzComponent {
    /// Returns the component name as a static string slice.
    pub fn as_str(self) -> &'static str {
        match self {
            SzComponent::Config => "SzConfig",
            SzComponent::ConfigManager => "SzConfigManager",
            SzComponent::Diagnostic => "SzDiagnostic",
            SzComponent::Engine => "SzEngine",
            SzComponent::Product => "SzProduct",
        }
    }
}

impl fmt::Display for SzComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Classification of an [`SzError`] into one of 17 error categories.
///
/// Mirrors the error-type taxonomy used across all Senzing SDK language
/// bindings.  Use [`SzError::kind()`] to inspect, or match directly.
///
/// The hierarchy is rooted at [`SzErrorKind::SzError`]:
///
/// ```text
/// SzError
/// ├── BadInput
/// │   ├── NotFound
/// │   └── UnknownDataSource
/// ├── General
/// │   ├── Configuration
/// │   ├── ReplaceConflict
/// │   └── Sdk
/// ├── Retryable
/// │   ├── DatabaseConnectionLost
/// │   ├── DatabaseTransient
/// │   └── RetryTimeoutExceeded
/// └── Unrecoverable
///     ├── Database
///     ├── License
///     ├── NotInitialized
///     └── Unhandled
/// ```
///
/// Converting an `SzErrorKind` into an `SzError` produces an error with
/// no code and an empty message — useful for quick construction in tests
/// or when only the category matters (mirrors `std::io::Error: From<ErrorKind>`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SzErrorKind {
    /// The input provided to the API was invalid.
    BadInput,
    /// A configuration error occurred.
    Configuration,
    /// An unrecoverable database error occurred.
    Database,
    /// The database connection was lost (retryable).
    DatabaseConnectionLost,
    /// A transient database error occurred (retryable).
    DatabaseTransient,
    /// A general error that does not fit other categories.
    General,
    /// A license-related error occurred.
    License,
    /// The requested entity or record was not found.
    NotFound,
    /// The component has not been initialized.
    NotInitialized,
    /// A replace-default-config conflict was detected.
    ReplaceConflict,
    /// A transient error that may succeed on retry.
    Retryable,
    /// The retry timeout was exceeded.
    RetryTimeoutExceeded,
    /// An error originating from the SDK wrapper layer.
    Sdk,
    /// Root of the error hierarchy (default kind).
    #[default] // Alphabetical position preserved; #[default] marks the hierarchy root.
    SzError,
    /// An unhandled error occurred.
    Unhandled,
    /// The specified data source is not recognized.
    UnknownDataSource,
    /// An unrecoverable error occurred.
    Unrecoverable,
}

impl SzErrorKind {
    /// Returns `true` if this is a bad input kind (BadInput, NotFound, UnknownDataSource).
    pub fn is_bad_input(self) -> bool {
        matches!(
            self,
            SzErrorKind::BadInput | SzErrorKind::NotFound | SzErrorKind::UnknownDataSource
        )
    }

    /// Returns `true` if this is a general kind (General, Configuration, ReplaceConflict, Sdk).
    pub fn is_general(self) -> bool {
        matches!(
            self,
            SzErrorKind::General
                | SzErrorKind::Configuration
                | SzErrorKind::ReplaceConflict
                | SzErrorKind::Sdk
        )
    }

    /// Returns `true` if this is a retryable kind (Retryable, DatabaseConnectionLost, DatabaseTransient, RetryTimeoutExceeded).
    pub fn is_retryable(self) -> bool {
        matches!(
            self,
            SzErrorKind::Retryable
                | SzErrorKind::DatabaseConnectionLost
                | SzErrorKind::DatabaseTransient
                | SzErrorKind::RetryTimeoutExceeded
        )
    }

    /// Returns `true` if this is an unrecoverable kind (Unrecoverable, Database, License, NotInitialized, Unhandled).
    pub fn is_unrecoverable(self) -> bool {
        matches!(
            self,
            SzErrorKind::Unrecoverable
                | SzErrorKind::Database
                | SzErrorKind::License
                | SzErrorKind::NotInitialized
                | SzErrorKind::Unhandled
        )
    }

    /// Returns `true` if this is a database-related kind, regardless of retryability (Database, DatabaseConnectionLost, DatabaseTransient).
    pub fn is_database(self) -> bool {
        matches!(
            self,
            SzErrorKind::Database
                | SzErrorKind::DatabaseConnectionLost
                | SzErrorKind::DatabaseTransient
        )
    }

    /// Returns `true` for any `SzErrorKind` variant — i.e., any Senzing error.
    ///
    /// `SzError` is the root of the error hierarchy, so every variant
    /// is an `SzError`.  This always returns `true`.
    pub fn is_sz_error(self) -> bool {
        true
    }

    /// Hierarchy-aware kind check.
    ///
    /// Returns `true` if `self` matches `kind`, honoring the error-type
    /// hierarchy.  For parent categories (`SzError`, `BadInput`, `Database`,
    /// `General`, `Retryable`, `Unrecoverable`) all child kinds also match.
    /// For leaf kinds the comparison is an exact equality check.
    ///
    /// # Examples
    /// ```
    /// # use sz_sdk::SzErrorKind;
    /// assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::Retryable));
    /// assert!(SzErrorKind::Retryable.is(SzErrorKind::Retryable));
    /// assert!(!SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::BadInput));
    /// assert!(SzErrorKind::General.is(SzErrorKind::SzError));
    /// ```
    pub fn is(self, kind: SzErrorKind) -> bool {
        match kind {
            SzErrorKind::BadInput => self.is_bad_input(),
            SzErrorKind::Database => self.is_database(),
            SzErrorKind::General => self.is_general(),
            SzErrorKind::Retryable => self.is_retryable(),
            SzErrorKind::Unrecoverable => self.is_unrecoverable(),
            SzErrorKind::SzError => true,
            other => self == other,
        }
    }

    /// Returns the severity level for this error kind.
    ///
    /// Severity levels:
    /// - `"critical"` — License, Unrecoverable, Unhandled
    /// - `"high"` — Database, NotInitialized
    /// - `"medium"` — Configuration, DatabaseConnectionLost, DatabaseTransient
    /// - `"low"` — all others (BadInput, General, NotFound, ReplaceConflict,
    ///   Retryable, RetryTimeoutExceeded, Sdk, UnknownDataSource)
    pub fn severity(self) -> &'static str {
        match self {
            SzErrorKind::License | SzErrorKind::Unrecoverable | SzErrorKind::Unhandled => {
                "critical"
            }
            SzErrorKind::Database | SzErrorKind::NotInitialized => "high",
            SzErrorKind::Configuration
            | SzErrorKind::DatabaseConnectionLost
            | SzErrorKind::DatabaseTransient => "medium",
            _ => "low",
        }
    }

    /// Returns the error kind hierarchy as a static slice, ordered leaf-first.
    ///
    /// For leaf kinds the slice contains the leaf followed by its parent(s).
    /// For parent kinds the slice contains only that parent.
    /// For the root `SzError` the slice contains only `SzError`.
    ///
    /// This is zero-allocation — every call returns a `&'static` reference.
    ///
    /// # Examples
    /// ```
    /// # use sz_sdk::SzErrorKind;
    /// assert_eq!(
    ///     SzErrorKind::DatabaseConnectionLost.hierarchy(),
    ///     &[SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable],
    /// );
    /// assert_eq!(
    ///     SzErrorKind::Retryable.hierarchy(),
    ///     &[SzErrorKind::Retryable],
    /// );
    /// ```
    pub fn hierarchy(self) -> &'static [SzErrorKind] {
        match self {
            // BadInput family
            SzErrorKind::BadInput => &[SzErrorKind::BadInput],
            SzErrorKind::NotFound => &[SzErrorKind::NotFound, SzErrorKind::BadInput],
            SzErrorKind::UnknownDataSource => {
                &[SzErrorKind::UnknownDataSource, SzErrorKind::BadInput]
            }
            // General family
            SzErrorKind::General => &[SzErrorKind::General],
            SzErrorKind::Configuration => &[SzErrorKind::Configuration, SzErrorKind::General],
            SzErrorKind::ReplaceConflict => &[SzErrorKind::ReplaceConflict, SzErrorKind::General],
            SzErrorKind::Sdk => &[SzErrorKind::Sdk, SzErrorKind::General],
            // Retryable family
            SzErrorKind::Retryable => &[SzErrorKind::Retryable],
            SzErrorKind::DatabaseConnectionLost => {
                &[SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable]
            }
            SzErrorKind::DatabaseTransient => {
                &[SzErrorKind::DatabaseTransient, SzErrorKind::Retryable]
            }
            SzErrorKind::RetryTimeoutExceeded => {
                &[SzErrorKind::RetryTimeoutExceeded, SzErrorKind::Retryable]
            }
            // Unrecoverable family
            SzErrorKind::Unrecoverable => &[SzErrorKind::Unrecoverable],
            SzErrorKind::Database => &[SzErrorKind::Database, SzErrorKind::Unrecoverable],
            SzErrorKind::License => &[SzErrorKind::License, SzErrorKind::Unrecoverable],
            SzErrorKind::NotInitialized => {
                &[SzErrorKind::NotInitialized, SzErrorKind::Unrecoverable]
            }
            SzErrorKind::Unhandled => &[SzErrorKind::Unhandled, SzErrorKind::Unrecoverable],
            // Root
            SzErrorKind::SzError => &[SzErrorKind::SzError],
        }
    }

    /// Returns `true` if this is a configuration error.
    pub fn is_configuration(self) -> bool {
        matches!(self, SzErrorKind::Configuration)
    }

    /// Returns `true` if this is a license error.
    pub fn is_license(self) -> bool {
        matches!(self, SzErrorKind::License)
    }

    /// Returns `true` if this is a not-found error.
    pub fn is_not_found(self) -> bool {
        matches!(self, SzErrorKind::NotFound)
    }

    /// Returns `true` if this is a not-initialized error.
    pub fn is_not_initialized(self) -> bool {
        matches!(self, SzErrorKind::NotInitialized)
    }

    /// Returns `true` if this is a replace-conflict error.
    pub fn is_replace_conflict(self) -> bool {
        matches!(self, SzErrorKind::ReplaceConflict)
    }

    /// Returns `true` if this is an SDK error.
    pub fn is_sdk(self) -> bool {
        matches!(self, SzErrorKind::Sdk)
    }

    /// Returns `true` if this is an unhandled error.
    pub fn is_unhandled(self) -> bool {
        matches!(self, SzErrorKind::Unhandled)
    }

    /// Returns `true` if this is an unknown-data-source error.
    pub fn is_unknown_data_source(self) -> bool {
        matches!(self, SzErrorKind::UnknownDataSource)
    }

    /// Returns the error category as a static string slug.
    ///
    /// Useful for structured logging, metrics, and error reporting systems.
    pub fn category(self) -> &'static str {
        match self {
            SzErrorKind::BadInput => "bad_input",
            SzErrorKind::Configuration => "configuration",
            SzErrorKind::Database => "database",
            SzErrorKind::DatabaseConnectionLost => "database_connection_lost",
            SzErrorKind::DatabaseTransient => "database_transient",
            SzErrorKind::General => "general",
            SzErrorKind::License => "license",
            SzErrorKind::NotFound => "not_found",
            SzErrorKind::NotInitialized => "not_initialized",
            SzErrorKind::ReplaceConflict => "replace_conflict",
            SzErrorKind::Retryable => "retryable",
            SzErrorKind::RetryTimeoutExceeded => "retry_timeout_exceeded",
            SzErrorKind::Sdk => "sdk",
            SzErrorKind::SzError => "sz_error",
            SzErrorKind::Unhandled => "unhandled",
            SzErrorKind::UnknownDataSource => "unknown_data_source",
            SzErrorKind::Unrecoverable => "unrecoverable",
        }
    }
}

impl fmt::Display for SzErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SzErrorKind::BadInput => "bad input",
            SzErrorKind::Configuration => "configuration error",
            SzErrorKind::Database => "database error",
            SzErrorKind::DatabaseConnectionLost => "database connection lost",
            SzErrorKind::DatabaseTransient => "database transient error",
            SzErrorKind::General => "general error",
            SzErrorKind::License => "license error",
            SzErrorKind::NotFound => "not found",
            SzErrorKind::NotInitialized => "not initialized",
            SzErrorKind::ReplaceConflict => "replace conflict",
            SzErrorKind::Retryable => "retryable error",
            SzErrorKind::RetryTimeoutExceeded => "retry timeout exceeded",
            SzErrorKind::Sdk => "SDK error",
            SzErrorKind::SzError => "Senzing error",
            SzErrorKind::Unhandled => "unhandled error",
            SzErrorKind::UnknownDataSource => "unknown data source",
            SzErrorKind::Unrecoverable => "unrecoverable error",
        };
        f.write_str(label)
    }
}

impl From<SzErrorType> for SzErrorKind {
    fn from(value: SzErrorType) -> Self {
        match value {
            SzErrorType::SzBadInputError => SzErrorKind::BadInput,
            SzErrorType::SzConfigurationError => SzErrorKind::Configuration,
            SzErrorType::SzDatabaseConnectionLostError => SzErrorKind::DatabaseConnectionLost,
            SzErrorType::SzDatabaseError => SzErrorKind::Database,
            SzErrorType::SzDatabaseTransientError => SzErrorKind::DatabaseTransient,
            SzErrorType::SzError => SzErrorKind::General,
            SzErrorType::SzLicenseError => SzErrorKind::License,
            SzErrorType::SzNotFoundError => SzErrorKind::NotFound,
            SzErrorType::SzNotInitializedError => SzErrorKind::NotInitialized,
            SzErrorType::SzReplaceConflictError => SzErrorKind::ReplaceConflict,
            SzErrorType::SzRetryTimeoutExceededError => SzErrorKind::RetryTimeoutExceeded,
            SzErrorType::SzUnhandledError => SzErrorKind::Unhandled,
            SzErrorType::SzUnknownDataSourceError => SzErrorKind::UnknownDataSource,
        }
    }
}

/// Error type for the Senzing SDK, following the `std::io::Error` /
/// `io::ErrorKind` pattern.
///
/// Each error carries a numeric Senzing error code, a human-readable message,
/// an [`SzErrorKind`] that classifies the error, an optional [`SzComponent`]
/// identifying the originating subsystem, and an optional source error.
#[derive(Debug)]
#[non_exhaustive]
pub struct SzError {
    code: Option<i64>,
    message: String,
    kind: SzErrorKind,
    kind_explicit: bool,
    component: Option<SzComponent>,
    details: Option<String>,
    source: Option<Arc<dyn std::error::Error + Send + Sync>>,
}

// Compile-time assertion that SzError is Send + Sync.
const _: () = {
    fn _assert_send_sync<T: Send + Sync>() {}
    fn _check() {
        _assert_send_sync::<SzError>();
    }
};

impl Clone for SzError {
    /// Clones the error, preserving the source chain via shared ownership.
    fn clone(&self) -> Self {
        Self {
            code: self.code,
            message: self.message.clone(),
            kind: self.kind,
            kind_explicit: self.kind_explicit,
            component: self.component,
            details: self.details.clone(),
            source: self.source.clone(),
        }
    }
}

impl SzError {
    /// Creates an `SzError` with a message and default kind.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
            kind: SzErrorKind::default(),
            kind_explicit: false,
            component: None,
            details: None,
            source: None,
        }
    }

    /// Creates a bad-input error.
    pub fn bad_input(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::BadInput)
    }

    /// Creates a configuration error.
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Configuration)
    }

    /// Creates a database error.
    pub fn database(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Database)
    }

    /// Creates a database-connection-lost error.
    pub fn database_connection_lost(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::DatabaseConnectionLost)
    }

    /// Creates a database-transient error.
    pub fn database_transient(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::DatabaseTransient)
    }

    /// Creates a general error.
    pub fn general(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::General)
    }

    /// Creates a license error.
    pub fn license(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::License)
    }

    /// Creates a not-found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::NotFound)
    }

    /// Creates a not-initialized error.
    pub fn not_initialized(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::NotInitialized)
    }

    /// Creates a replace-conflict error.
    pub fn replace_conflict(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::ReplaceConflict)
    }

    /// Creates a retryable error.
    pub fn retryable(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Retryable)
    }

    /// Creates a retry-timeout-exceeded error.
    pub fn retry_timeout_exceeded(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::RetryTimeoutExceeded)
    }

    /// Creates an SDK error.
    pub fn sdk(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Sdk)
    }

    /// Creates an unhandled error.
    pub fn unhandled(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Unhandled)
    }

    /// Creates an unknown-data-source error.
    pub fn unknown_data_source(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::UnknownDataSource)
    }

    /// Creates an unrecoverable error.
    pub fn unrecoverable(message: impl Into<String>) -> Self {
        Self::new(message).with_kind(SzErrorKind::Unrecoverable)
    }

    /// Wraps a non-Senzing error as an [`SzErrorKind::Sdk`] error.
    ///
    /// The error's [`Display`](std::fmt::Display) text becomes the message,
    /// and the original error is chained as the
    /// [source](std::error::Error::source).  Use named constructors +
    /// [`with_source()`](Self::with_source) when you need a specific kind or
    /// custom message.
    ///
    /// # Examples
    ///
    /// ```
    /// use sz_sdk::{SzError, SzErrorKind};
    /// use std::error::Error;
    /// use std::io;
    ///
    /// let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "gone");
    /// let err = SzError::wrap(io_err);
    /// assert_eq!(err.kind(), SzErrorKind::Sdk);
    /// assert!(err.source().is_some());
    /// ```
    pub fn wrap(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        let message = source.to_string();
        Self::new(message)
            .with_kind(SzErrorKind::Sdk)
            .with_source(source)
    }

    /// Sets the error code, returning `self`.
    ///
    /// If [`with_kind`](Self::with_kind) has **not** been called (and the
    /// error was not created via `From<SzErrorKind>`), the kind is derived
    /// from the code using the `SZ_ERROR_TYPES` lookup table, defaulting
    /// to [`SzErrorKind::SzError`] for unknown codes.
    ///
    /// If `with_kind` **has** been called, the explicitly set kind is
    /// preserved and the code is stored without changing the kind.
    pub fn with_code(mut self, code: i64) -> Self {
        if !self.kind_explicit {
            self.kind = SZ_ERROR_TYPES
                .get(&code)
                .copied()
                .map(SzErrorKind::from)
                .unwrap_or_default();
        }
        self.code = Some(code);
        self
    }

    /// Sets the component that produced this error and returns `self`.
    pub fn with_component(mut self, component: SzComponent) -> Self {
        self.component = Some(component);
        self
    }

    /// Sets supplementary details for this error and returns `self`.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Sets the error kind and returns `self`.
    pub fn with_kind(mut self, kind: SzErrorKind) -> Self {
        self.kind = kind;
        self.kind_explicit = true;
        self
    }

    /// Sets the error message and returns `self`.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets the source (cause) of this error and returns `self`.
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }

    /// Returns the numeric Senzing error code, if set.
    pub fn code(&self) -> Option<i64> {
        self.code
    }

    /// Returns the human-readable error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the [`SzErrorKind`] that classifies this error.
    pub fn kind(&self) -> SzErrorKind {
        self.kind
    }

    /// Returns the [`SzComponent`] that produced this error, if set.
    pub fn component(&self) -> Option<SzComponent> {
        self.component
    }

    /// Returns the supplementary details, if set.
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Returns the component name as a string, or `""` if no component is set.
    pub fn component_name(&self) -> &str {
        match self.component {
            Some(c) => c.as_str(),
            None => "",
        }
    }

    /// Returns `true` if this error's kind matches the given [`SzErrorKind`].
    pub fn is_kind(&self, kind: SzErrorKind) -> bool {
        self.kind == kind
    }

    /// Hierarchy-aware kind check.
    ///
    /// Returns `true` if this error's kind matches `kind`, honoring the
    /// error-type hierarchy.  See [`SzErrorKind::is`] for details.
    pub fn is(&self, kind: SzErrorKind) -> bool {
        self.kind.is(kind)
    }

    /// Returns `true` if this is a bad input error (BadInput, NotFound, UnknownDataSource).
    pub fn is_bad_input(&self) -> bool {
        self.kind.is_bad_input()
    }

    /// Returns `true` if this is a general error (General, Configuration, ReplaceConflict, Sdk).
    pub fn is_general(&self) -> bool {
        self.kind.is_general()
    }

    /// Returns `true` if this is a retryable error (Retryable, DatabaseConnectionLost, DatabaseTransient, RetryTimeoutExceeded).
    pub fn is_retryable(&self) -> bool {
        self.kind.is_retryable()
    }

    /// Returns `true` if this is an unrecoverable error (Unrecoverable, Database, License, NotInitialized, Unhandled).
    pub fn is_unrecoverable(&self) -> bool {
        self.kind.is_unrecoverable()
    }

    /// Returns `true` if this is a database-related error, regardless of retryability (Database, DatabaseConnectionLost, DatabaseTransient).
    pub fn is_database(&self) -> bool {
        self.kind.is_database()
    }

    /// Returns `true` — every `SzError` is a Senzing error.
    pub fn is_sz_error(&self) -> bool {
        self.kind.is_sz_error()
    }

    /// Returns the severity level for this error.
    ///
    /// See [`SzErrorKind::severity`] for the mapping.
    pub fn severity(&self) -> &'static str {
        self.kind.severity()
    }

    /// Returns the error category as a static string slug.
    ///
    /// See [`SzErrorKind::category`] for the mapping.
    pub fn category(&self) -> &'static str {
        self.kind.category()
    }

    /// Returns the error kind hierarchy as a static slice, ordered leaf-first.
    ///
    /// See [`SzErrorKind::hierarchy`] for details.
    pub fn hierarchy(&self) -> &'static [SzErrorKind] {
        self.kind.hierarchy()
    }

    /// Returns `true` if this is a configuration error.
    pub fn is_configuration(&self) -> bool {
        self.kind.is_configuration()
    }

    /// Returns `true` if this is a license error.
    pub fn is_license(&self) -> bool {
        self.kind.is_license()
    }

    /// Returns `true` if this is a not-found error.
    pub fn is_not_found(&self) -> bool {
        self.kind.is_not_found()
    }

    /// Returns `true` if this is a not-initialized error.
    pub fn is_not_initialized(&self) -> bool {
        self.kind.is_not_initialized()
    }

    /// Returns `true` if this is a replace-conflict error.
    pub fn is_replace_conflict(&self) -> bool {
        self.kind.is_replace_conflict()
    }

    /// Returns `true` if this is an SDK error.
    pub fn is_sdk(&self) -> bool {
        self.kind.is_sdk()
    }

    /// Returns `true` if this is an unhandled error.
    pub fn is_unhandled(&self) -> bool {
        self.kind.is_unhandled()
    }

    /// Returns `true` if this is an unknown-data-source error.
    pub fn is_unknown_data_source(&self) -> bool {
        self.kind.is_unknown_data_source()
    }

    /// Finds the first [`SzError`] in an error's source chain.
    ///
    /// Checks the error itself first, then walks [`.source()`](std::error::Error::source).
    /// Returns `None` if no `SzError` is found anywhere in the chain.
    ///
    /// Prefer [`SzErrorInspect`] trait methods when the trait is in scope;
    /// use this when you have a bare `&dyn Error` and bringing the trait
    /// into scope is awkward.
    ///
    /// # Examples
    ///
    /// Direct hit — the error itself is an `SzError`:
    ///
    /// ```
    /// use sz_sdk::{SzError, SzErrorKind};
    ///
    /// let err = SzError::not_found("entity 42");
    /// let found = SzError::find_in_chain(&err).unwrap();
    /// assert_eq!(found.kind(), SzErrorKind::NotFound);
    /// ```
    ///
    /// Chain walking — `SzError` is wrapped inside another error:
    ///
    /// ```
    /// use sz_sdk::{SzError, SzErrorKind};
    ///
    /// let inner = SzError::database_transient("deadlock");
    /// // Wrap SzError as the source of another SzError
    /// let outer = SzError::new("wrapper").with_source(inner);
    /// let found = SzError::find_in_chain(&outer).unwrap();
    /// // find_in_chain returns the first SzError it finds (the outer one)
    /// assert_eq!(found.message(), "wrapper");
    /// ```
    ///
    /// No match — returns `None` for non-Senzing errors:
    ///
    /// ```
    /// use sz_sdk::SzError;
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
    /// assert!(SzError::find_in_chain(&io_err).is_none());
    /// ```
    pub fn find_in_chain<'a>(
        mut err: &'a (dyn std::error::Error + 'static),
    ) -> Option<&'a SzError> {
        loop {
            if let Some(sz) = err.downcast_ref::<SzError>() {
                return Some(sz);
            }
            err = err.source()?;
        }
    }
}

impl fmt::Display for SzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.code, self.details.as_deref()) {
            (Some(code), Some(details)) => {
                write!(
                    f,
                    "{} (code {}): {} [{}]",
                    self.kind, code, self.message, details
                )
            }
            (Some(code), None) => {
                write!(f, "{} (code {}): {}", self.kind, code, self.message)
            }
            (None, Some(details)) => {
                write!(f, "{}: {} [{}]", self.kind, self.message, details)
            }
            (None, None) => write!(f, "{}: {}", self.kind, self.message),
        }
    }
}

impl std::error::Error for SzError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl From<SzErrorKind> for SzError {
    fn from(kind: SzErrorKind) -> Self {
        Self {
            code: None,
            message: String::new(),
            kind,
            kind_explicit: true,
            component: None,
            details: None,
            source: None,
        }
    }
}

// ---------------------------------------------------------------------------
// SzErrorInspect extension trait
// ---------------------------------------------------------------------------

/// Extension trait for inspecting any error (or error chain) for an embedded [`SzError`].
///
/// Walks the [`.source()`](std::error::Error::source) chain, so it finds
/// `SzError` even when wrapped by middleware layers like `anyhow` or custom
/// wrapper errors.  This is the primary way to classify errors in code that
/// deals with multiple error types.
///
/// Bring the trait into scope with `use sz_sdk::SzErrorInspect;`.  If you
/// have a bare `&dyn Error` and importing the trait is awkward, use the
/// static method [`SzError::find_in_chain`] instead.
///
/// # Available methods
///
/// | Method | Returns |
/// |--------|---------|
/// | [`sz_error()`](SzErrorInspect::sz_error) | `Option<&SzError>` — the first `SzError` in the chain |
/// | [`is_sz_retryable()`](SzErrorInspect::is_sz_retryable) | `bool` — chain contains a retryable error |
/// | [`is_sz_unrecoverable()`](SzErrorInspect::is_sz_unrecoverable) | `bool` — chain contains an unrecoverable error |
/// | [`is_sz_bad_input()`](SzErrorInspect::is_sz_bad_input) | `bool` — chain contains a bad-input error |
/// | [`is_sz_general()`](SzErrorInspect::is_sz_general) | `bool` — chain contains a general error |
/// | [`is_sz_database()`](SzErrorInspect::is_sz_database) | `bool` — chain contains a database error |
/// | [`is_sz_error()`](SzErrorInspect::is_sz_error) | `bool` — chain contains any `SzError` |
/// | [`is_sz(kind)`](SzErrorInspect::is_sz) | `bool` — hierarchy-aware kind check |
/// | [`sz_component()`](SzErrorInspect::sz_component) | `Option<SzComponent>` |
/// | [`sz_severity()`](SzErrorInspect::sz_severity) | `Option<&'static str>` |
///
/// # Example: classifying a wrapped error
///
/// ```
/// use sz_sdk::{SzError, SzErrorKind, SzErrorInspect};
/// use std::fmt;
///
/// // A custom wrapper whose source() exposes the inner error
/// #[derive(Debug)]
/// struct AppError(Box<dyn std::error::Error + Send + Sync>);
/// impl fmt::Display for AppError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "app: {}", self.0)
///     }
/// }
/// impl std::error::Error for AppError {
///     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
///         Some(&*self.0)
///     }
/// }
///
/// let inner = SzError::database_transient("deadlock");
/// let wrapped = AppError(Box::new(inner));
///
/// // SzErrorInspect walks the chain automatically
/// assert!(wrapped.is_sz_retryable());
/// assert!(wrapped.is_sz(SzErrorKind::DatabaseTransient));
/// assert!(wrapped.is_sz(SzErrorKind::Retryable));  // hierarchy-aware
/// assert!(!wrapped.is_sz(SzErrorKind::BadInput));
/// ```
pub trait SzErrorInspect {
    /// Returns a reference to the first `SzError` in the source chain, if any.
    fn sz_error(&self) -> Option<&SzError>;

    /// Returns `true` if the chain contains a retryable `SzError`.
    fn is_sz_retryable(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_retryable())
    }

    /// Returns `true` if the chain contains an unrecoverable `SzError`.
    fn is_sz_unrecoverable(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_unrecoverable())
    }

    /// Returns `true` if the chain contains a bad-input `SzError`.
    fn is_sz_bad_input(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_bad_input())
    }

    /// Returns `true` if the chain contains a general `SzError`.
    fn is_sz_general(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_general())
    }

    /// Returns `true` if the chain contains a database-related `SzError`.
    fn is_sz_database(&self) -> bool {
        self.sz_error().is_some_and(|e| e.is_database())
    }

    /// Returns `true` if the chain contains any `SzError`.
    fn is_sz_error(&self) -> bool {
        self.sz_error().is_some()
    }

    /// Hierarchy-aware kind check on the first `SzError` in the chain.
    fn is_sz(&self, kind: SzErrorKind) -> bool {
        self.sz_error().is_some_and(|e| e.is(kind))
    }

    /// Returns the [`SzComponent`] of the first `SzError` in the chain, if any.
    fn sz_component(&self) -> Option<SzComponent> {
        self.sz_error().and_then(|e| e.component())
    }

    /// Returns the severity of the first `SzError` in the chain, if any.
    fn sz_severity(&self) -> Option<&'static str> {
        self.sz_error().map(|e| e.severity())
    }
}

impl SzErrorInspect for dyn std::error::Error + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + Sync + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

impl<E: std::error::Error + 'static> SzErrorInspect for E {
    fn sz_error(&self) -> Option<&SzError> {
        SzError::find_in_chain(self)
    }
}

// ---------------------------------------------------------------------------
// SzResult type alias
// ---------------------------------------------------------------------------

/// Result type alias for Senzing SDK operations.
///
/// All trait methods in this crate return `SzResult<T>` instead of
/// `Result<T, SzError>`.
pub type SzResult<T> = Result<T, SzError>;

// ---------------------------------------------------------------------------
// SzResultExt extension trait
// ---------------------------------------------------------------------------

/// Extension trait for [`SzResult<T>`] providing error-classification helpers.
///
/// These methods let you handle retryable errors inline without explicit
/// match arms.  Bring the trait into scope with `use sz_sdk::SzResultExt;`.
///
/// # Examples
///
/// ```
/// use sz_sdk::{SzError, SzErrorKind, SzResult, SzResultExt};
///
/// fn might_fail() -> SzResult<String> {
///     Err(SzError::new("transient").with_code(1008))
/// }
///
/// let result = might_fail().or_retry(|_| Ok("recovered".into()));
/// assert_eq!(result.unwrap(), "recovered");
/// ```
pub trait SzResultExt<T> {
    /// If the error is retryable, call `f`; otherwise propagate the error unchanged.
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>;

    /// Converts a retryable error into `Ok(None)`, propagates all others as `Err`.
    ///
    /// Useful for filtering retryable failures out of a processing loop.
    fn filter_retryable(self) -> Result<Option<T>, SzError>;

    /// Returns `true` if the result is an error and that error is retryable.
    fn is_retryable_err(&self) -> bool;

    /// Returns `true` if the result is an error and that error is unrecoverable.
    fn is_unrecoverable_err(&self) -> bool;

    /// Returns `true` if the result is an error and that error is bad input.
    fn is_bad_input_err(&self) -> bool;

    /// Returns `true` if the result is an error and that error is general.
    fn is_general_err(&self) -> bool;

    /// Returns `true` if the result is an error and that error is database-related.
    fn is_database_err(&self) -> bool;
}

impl<T> SzResultExt<T> for SzResult<T> {
    fn or_retry<F>(self, f: F) -> SzResult<T>
    where
        F: FnOnce(SzError) -> SzResult<T>,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) if e.is_retryable() => f(e),
            Err(e) => Err(e),
        }
    }

    fn filter_retryable(self) -> Result<Option<T>, SzError> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(e) if e.is_retryable() => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn is_retryable_err(&self) -> bool {
        matches!(self, Err(e) if e.is_retryable())
    }

    fn is_unrecoverable_err(&self) -> bool {
        matches!(self, Err(e) if e.is_unrecoverable())
    }

    fn is_bad_input_err(&self) -> bool {
        matches!(self, Err(e) if e.is_bad_input())
    }

    fn is_general_err(&self) -> bool {
        matches!(self, Err(e) if e.is_general())
    }

    fn is_database_err(&self) -> bool {
        matches!(self, Err(e) if e.is_database())
    }
}
