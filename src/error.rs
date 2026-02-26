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

use crate::errortypes::{SzError as SzErrorType, SZ_ERROR_TYPES};
use std::fmt;
use std::sync::Arc;

/// Identifies which Senzing SDK component produced an error.
///
/// Mirrors the five core subsystems of the Senzing SDK.  Implementations
/// set this when constructing an [`SzError`] so callers can determine
/// *which* component failed without parsing the error message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SzComponent {
    Config,
    ConfigManager,
    Diagnostic,
    Engine,
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
/// code `0` and an empty message — useful for quick construction in tests
/// or when only the category matters (mirrors `std::io::Error: From<ErrorKind>`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SzErrorKind {
    BadInput,
    Configuration,
    Database,
    DatabaseConnectionLost,
    DatabaseTransient,
    General,
    License,
    NotFound,
    NotInitialized,
    ReplaceConflict,
    Retryable,
    RetryTimeoutExceeded,
    Sdk,
    #[default]
    SzError,
    Unhandled,
    UnknownDataSource,
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
    /// hierarchy.  For parent categories (`SzError`, `BadInput`, `General`,
    /// `Retryable`, `Unrecoverable`) all child kinds also match.  For
    /// leaf kinds the comparison is an exact equality check.
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
pub struct SzError {
    code: Option<i64>,
    message: String,
    kind: SzErrorKind,
    kind_explicit: bool,
    component: Option<SzComponent>,
    source: Option<Arc<dyn std::error::Error + Send + Sync>>,
}

impl Clone for SzError {
    /// Clones the error, preserving the source chain via shared ownership.
    fn clone(&self) -> Self {
        Self {
            code: self.code,
            message: self.message.clone(),
            kind: self.kind,
            kind_explicit: self.kind_explicit,
            component: self.component,
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

    /// Sets the error code, returning `self`.
    ///
    /// If [`with_kind`](Self::with_kind) has **not** been called (and the
    /// error was not created via `From<SzErrorKind>`), the kind is derived
    /// from the code using the `SZ_ERROR_TYPES` lookup table, defaulting
    /// to [`SzErrorKind::General`] for unknown codes.
    ///
    /// If `with_kind` **has** been called, the explicitly set kind is
    /// preserved and the code is stored without changing the kind.
    pub fn with_code(mut self, code: i64) -> Self {
        if !self.kind_explicit {
            self.kind = SZ_ERROR_TYPES
                .get(&code)
                .copied()
                .map(SzErrorKind::from)
                .unwrap_or(SzErrorKind::General);
        }
        self.code = Some(code);
        self
    }

    /// Sets the component that produced this error and returns `self`.
    pub fn with_component(mut self, component: SzComponent) -> Self {
        self.component = Some(component);
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
}

impl fmt::Display for SzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "{} (code {}): {}", self.kind, code, self.message),
            None => write!(f, "{}: {}", self.kind, self.message),
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
            source: None,
        }
    }
}

/// If the error is an [`SzError`], returns a reference to it;
/// otherwise returns `None`.
///
/// This is a convenience wrapper around `downcast_ref::<SzError>()`
/// so callers don't need to name the concrete type.
pub fn as_sz_error<'a>(err: &'a (dyn std::error::Error + 'static)) -> Option<&'a SzError> {
    err.downcast_ref::<SzError>()
}

/// If the error is an [`SzError`], returns the [`SzComponent`] that produced it.
pub fn component(err: &(dyn std::error::Error + 'static)) -> Option<SzComponent> {
    err.downcast_ref::<SzError>().and_then(|e| e.component())
}

/// If the error is an [`SzError`], returns its severity level;
/// otherwise returns `None`.
pub fn severity(err: &(dyn std::error::Error + 'static)) -> Option<&'static str> {
    err.downcast_ref::<SzError>().map(|e| e.severity())
}

/// Returns `true` if the error is an `SzError` with the given [`SzErrorKind`].
pub fn is_kind(err: &(dyn std::error::Error + 'static), kind: SzErrorKind) -> bool {
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.kind() == kind)
}

/// Hierarchy-aware kind check on a `dyn Error`.
///
/// Returns `true` if the error is an [`SzError`] whose kind matches `kind`,
/// honoring the error-type hierarchy.  For parent categories (`BadInput`,
/// `General`, `Retryable`, `Unrecoverable`) all child kinds also match.
/// Returns `false` for non-`SzError` values.
///
/// See [`SzErrorKind::is`] for the hierarchy rules.
pub fn is(err: &(dyn std::error::Error + 'static), kind: SzErrorKind) -> bool {
    err.downcast_ref::<SzError>().is_some_and(|e| e.is(kind))
}

/// Returns `true` if the error is an `SzError`.
pub fn is_sz_error(err: &(dyn std::error::Error + 'static)) -> bool {
    err.downcast_ref::<SzError>().is_some()
}

/// Returns `true` if the error is an `SzError` in the bad input category.
pub fn is_bad_input(err: &(dyn std::error::Error + 'static)) -> bool {
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.is_bad_input())
}

/// Returns `true` if the error is an `SzError` in the general category.
pub fn is_general(err: &(dyn std::error::Error + 'static)) -> bool {
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.is_general())
}

/// Returns `true` if the error is an `SzError` in the retryable category.
pub fn is_retryable(err: &(dyn std::error::Error + 'static)) -> bool {
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.is_retryable())
}

/// Returns `true` if the error is an `SzError` in the unrecoverable category.
pub fn is_unrecoverable(err: &(dyn std::error::Error + 'static)) -> bool {
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.is_unrecoverable())
}

// ---------------------------------------------------------------------------
// SzErrorInspect extension trait
// ---------------------------------------------------------------------------

/// Walks the error source chain looking for an `SzError`.
fn find_sz_error<'a>(mut err: &'a (dyn std::error::Error + 'static)) -> Option<&'a SzError> {
    loop {
        if let Some(sz) = err.downcast_ref::<SzError>() {
            return Some(sz);
        }
        err = err.source()?;
    }
}

/// Extension trait for inspecting any error (or error chain) for an embedded [`SzError`].
///
/// Walks the `.source()` chain, so it finds `SzError` even when wrapped
/// by middleware layers like `anyhow` or custom wrapper errors.
///
/// Callers bring this into scope with `use sz_sdk::SzErrorInspect;`.
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

    /// Returns `true` if the chain contains any `SzError`.
    fn is_sz_error(&self) -> bool {
        self.sz_error().is_some()
    }

    /// Hierarchy-aware kind check on the first `SzError` in the chain.
    fn is_sz(&self, kind: SzErrorKind) -> bool {
        self.sz_error().is_some_and(|e| e.is(kind))
    }
}

impl SzErrorInspect for dyn std::error::Error + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        find_sz_error(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        find_sz_error(self)
    }
}

impl SzErrorInspect for dyn std::error::Error + Send + Sync + 'static {
    fn sz_error(&self) -> Option<&SzError> {
        find_sz_error(self)
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
}
