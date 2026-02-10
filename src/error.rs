use thiserror::Error;

/// Error types for the Senzing SDK, matching the canonical error categories
/// defined across all Senzing SDK language bindings.
#[derive(Debug, Error)]
pub enum SzError {
    #[error("bad input (code {code}): {message}")]
    BadInput { code: i32, message: String },

    #[error("configuration error (code {code}): {message}")]
    Configuration { code: i32, message: String },

    #[error("database error (code {code}): {message}")]
    Database { code: i32, message: String },

    #[error("database connection lost (code {code}): {message}")]
    DatabaseConnectionLost { code: i32, message: String },

    #[error("database transient error (code {code}): {message}")]
    DatabaseTransient { code: i32, message: String },

    #[error("general error (code {code}): {message}")]
    General { code: i32, message: String },

    #[error("license error (code {code}): {message}")]
    License { code: i32, message: String },

    #[error("not found (code {code}): {message}")]
    NotFound { code: i32, message: String },

    #[error("not initialized (code {code}): {message}")]
    NotInitialized { code: i32, message: String },

    #[error("replace conflict (code {code}): {message}")]
    ReplaceConflict { code: i32, message: String },

    #[error("retryable error (code {code}): {message}")]
    Retryable { code: i32, message: String },

    #[error("retry timeout exceeded (code {code}): {message}")]
    RetryTimeoutExceeded { code: i32, message: String },

    #[error("SDK error (code {code}): {message}")]
    Sdk { code: i32, message: String },

    #[error("unhandled error (code {code}): {message}")]
    Unhandled { code: i32, message: String },

    #[error("unknown data source (code {code}): {message}")]
    UnknownDataSource { code: i32, message: String },

    #[error("unrecoverable error (code {code}): {message}")]
    Unrecoverable { code: i32, message: String },
}
