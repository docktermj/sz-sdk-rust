use crate::errortypes::{SzError as SzErrorType, SZ_ERROR_TYPES};
use std::fmt;

/// Classification of an [`SzError`] into one of 16 error categories.
///
/// Mirrors the error-type taxonomy used across all Senzing SDK language
/// bindings.  Use [`SzError::kind()`] to inspect, or match directly.
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
    /// Human-readable label used in `Display` output.
    fn label(self) -> &'static str {
        match self {
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
        }
    }
}

/// Error type for the Senzing SDK, following the `std::io::Error` /
/// `io::ErrorKind` pattern.
///
/// Each error carries a numeric Senzing error code, a human-readable message,
/// and an [`SzErrorKind`] that classifies the error into one of the canonical
/// Senzing error categories.
#[derive(Debug)]
pub struct SzError {
    code: i32,
    message: String,
    kind: SzErrorKind,
}

impl SzError {
    /// Creates an `SzError` with explicit code, message, and kind.
    pub fn new(code: i32, message: String, kind: SzErrorKind) -> Self {
        Self {
            code,
            message,
            kind,
        }
    }

    /// Creates the appropriate `SzError` for a Senzing error code.
    ///
    /// Looks up the error code in the `SZ_ERROR_TYPES` map to determine the
    /// [`SzErrorKind`].  Unknown codes default to [`SzErrorKind::General`].
    pub fn from_code(code: i32, message: String) -> Self {
        let kind = match SZ_ERROR_TYPES.get(&code) {
            Some(SzErrorType::SzBadInputError) => SzErrorKind::BadInput,
            Some(SzErrorType::SzConfigurationError) => SzErrorKind::Configuration,
            Some(SzErrorType::SzDatabaseConnectionLostError) => SzErrorKind::DatabaseConnectionLost,
            Some(SzErrorType::SzDatabaseError) => SzErrorKind::Database,
            Some(SzErrorType::SzDatabaseTransientError) => SzErrorKind::DatabaseTransient,
            Some(SzErrorType::SzError) => SzErrorKind::General,
            Some(SzErrorType::SzLicenseError) => SzErrorKind::License,
            Some(SzErrorType::SzNotFoundError) => SzErrorKind::NotFound,
            Some(SzErrorType::SzNotInitializedError) => SzErrorKind::NotInitialized,
            Some(SzErrorType::SzReplaceConflictError) => SzErrorKind::ReplaceConflict,
            Some(SzErrorType::SzRetryTimeoutExceededError) => SzErrorKind::RetryTimeoutExceeded,
            Some(SzErrorType::SzUnhandledError) => SzErrorKind::Unhandled,
            Some(SzErrorType::SzUnknownDataSourceError) => SzErrorKind::UnknownDataSource,
            None => SzErrorKind::General,
        };
        Self {
            code,
            message,
            kind,
        }
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

    /// Returns `true` if this is a bad input error (BadInput, NotFound, UnknownDataSource).
    pub fn is_bad_input(&self) -> bool {
        matches!(
            self.kind,
            SzErrorKind::BadInput | SzErrorKind::NotFound | SzErrorKind::UnknownDataSource
        )
    }

    /// Returns `true` if this is a general error (General, Configuration, ReplaceConflict, Sdk).
    pub fn is_general(&self) -> bool {
        matches!(
            self.kind,
            SzErrorKind::General
                | SzErrorKind::Configuration
                | SzErrorKind::ReplaceConflict
                | SzErrorKind::Sdk
        )
    }

    /// Returns `true` if this is a retryable error (Retryable, DatabaseConnectionLost, DatabaseTransient, RetryTimeoutExceeded).
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.kind,
            SzErrorKind::Retryable
                | SzErrorKind::DatabaseConnectionLost
                | SzErrorKind::DatabaseTransient
                | SzErrorKind::RetryTimeoutExceeded
        )
    }

    /// Returns `true` if this is an unrecoverable error (Unrecoverable, Database, License, NotInitialized, Unhandled).
    pub fn is_unrecoverable(&self) -> bool {
        matches!(
            self.kind,
            SzErrorKind::Unrecoverable
                | SzErrorKind::Database
                | SzErrorKind::License
                | SzErrorKind::NotInitialized
                | SzErrorKind::Unhandled
        )
    }
}

impl fmt::Display for SzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (code {}): {}",
            self.kind.label(),
            self.code,
            self.message
        )
    }
}

impl std::error::Error for SzError {}

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
