use crate::errortypes::{SzError as SzErrorType, SZ_ERROR_TYPES};
use std::fmt;

/// Classification of an [`SzError`] into one of 16 error categories.
///
/// Mirrors the error-type taxonomy used across all Senzing SDK language
/// bindings.  Use [`SzError::kind()`] to inspect, or match directly.
///
/// Converting an `SzErrorKind` into an `SzError` produces an error with
/// code `0` and an empty message — useful for quick construction in tests
/// or when only the category matters (mirrors `std::io::Error: From<ErrorKind>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Hierarchy-aware kind check.
    ///
    /// Returns `true` if `self` matches `kind`, honoring the error-type
    /// hierarchy.  For the four parent categories (`BadInput`, `General`,
    /// `Retryable`, `Unrecoverable`) all child kinds also match.  For
    /// leaf kinds the comparison is an exact equality check.
    ///
    /// # Examples
    /// ```
    /// # use sz_sdk::SzErrorKind;
    /// assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::Retryable));
    /// assert!(SzErrorKind::Retryable.is(SzErrorKind::Retryable));
    /// assert!(!SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::BadInput));
    /// ```
    pub fn is(self, kind: SzErrorKind) -> bool {
        match kind {
            SzErrorKind::BadInput => self.is_bad_input(),
            SzErrorKind::General => self.is_general(),
            SzErrorKind::Retryable => self.is_retryable(),
            SzErrorKind::Unrecoverable => self.is_unrecoverable(),
            other => self == other,
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
/// an [`SzErrorKind`] that classifies the error, and an optional source error.
#[derive(Debug)]
pub struct SzError {
    code: i32,
    message: String,
    kind: SzErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Clone for SzError {
    /// Clones the error, dropping the source chain (which is not cloneable).
    fn clone(&self) -> Self {
        Self {
            code: self.code,
            message: self.message.clone(),
            kind: self.kind,
            source: None,
        }
    }
}

impl SzError {
    /// Creates an `SzError` with explicit code, message, and kind.
    pub fn new(code: i32, message: String, kind: SzErrorKind) -> Self {
        Self {
            code,
            message,
            kind,
            source: None,
        }
    }

    /// Creates the appropriate `SzError` for a Senzing error code.
    ///
    /// Looks up the error code in the `SZ_ERROR_TYPES` map to determine the
    /// [`SzErrorKind`].  Unknown codes default to [`SzErrorKind::General`].
    pub fn from_code(code: i32, message: String) -> Self {
        let kind = SZ_ERROR_TYPES
            .get(&code)
            .copied()
            .map(SzErrorKind::from)
            .unwrap_or(SzErrorKind::General);
        Self {
            code,
            message,
            kind,
            source: None,
        }
    }

    /// Sets the source (cause) of this error and returns `self`.
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the numeric Senzing error code.
    pub fn code(&self) -> i32 {
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
}

impl fmt::Display for SzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {}): {}", self.kind, self.code, self.message)
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
            code: 0,
            message: String::new(),
            kind,
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
    err.downcast_ref::<SzError>()
        .is_some_and(|e| e.is(kind))
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
