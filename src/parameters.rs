//! Common parameter constants for the Senzing SDK.
//!
//! These constants provide named values for frequently used parameters
//! such as empty strings, default configuration selectors, and logging
//! toggles. Using these constants instead of raw literals improves
//! readability and reduces the chance of errors.

/// Use the default configuration during initialization.
pub const SZ_INITIALIZE_WITH_DEFAULT_CONFIGURATION: i64 = 0;

/// Empty attributes string.
pub const SZ_NO_ATTRIBUTES: &str = "";

/// Empty avoidance string (no entities/records to avoid).
pub const SZ_NO_AVOIDANCE: &str = "";

/// Disable verbose logging.
pub const SZ_NO_LOGGING: i64 = 0;

/// Empty required data sources string.
pub const SZ_NO_REQUIRED_DATASOURCES: &str = "";

/// Empty search profile string (use default profile).
pub const SZ_NO_SEARCH_PROFILE: &str = "";

/// Enable verbose logging.
pub const SZ_VERBOSE_LOGGING: i64 = 1;

/// Flags value indicating no info should be returned.
pub const SZ_WITHOUT_INFO: i64 = 0;
