use super::error::{self, SzError, SzErrorKind};

// ---------------------------------------------------------------------------
// from_code: verify each error type classification
// ---------------------------------------------------------------------------

#[test]
fn from_code_bad_input() {
    let err = SzError::from_code(2, "test".into());
    assert_eq!(err.kind(), SzErrorKind::BadInput);
    assert_eq!(err.code(), 2);
}

#[test]
fn from_code_configuration() {
    let err = SzError::from_code(14, "test".into());
    assert_eq!(err.kind(), SzErrorKind::Configuration);
    assert_eq!(err.code(), 14);
}

#[test]
fn from_code_database() {
    let err = SzError::from_code(1000, "test".into());
    assert_eq!(err.kind(), SzErrorKind::Database);
    assert_eq!(err.code(), 1000);
}

#[test]
fn from_code_database_connection_lost() {
    let err = SzError::from_code(1006, "test".into());
    assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
    assert_eq!(err.code(), 1006);
}

#[test]
fn from_code_database_transient() {
    let err = SzError::from_code(1008, "test".into());
    assert_eq!(err.kind(), SzErrorKind::DatabaseTransient);
    assert_eq!(err.code(), 1008);
}

#[test]
fn from_code_general() {
    let err = SzError::from_code(5, "test".into());
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), 5);
}

#[test]
fn from_code_license() {
    let err = SzError::from_code(999, "test".into());
    assert_eq!(err.kind(), SzErrorKind::License);
    assert_eq!(err.code(), 999);
}

#[test]
fn from_code_not_found() {
    let err = SzError::from_code(33, "test".into());
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.code(), 33);
}

#[test]
fn from_code_not_initialized() {
    let err = SzError::from_code(48, "test".into());
    assert_eq!(err.kind(), SzErrorKind::NotInitialized);
    assert_eq!(err.code(), 48);
}

#[test]
fn from_code_replace_conflict() {
    let err = SzError::from_code(7245, "test".into());
    assert_eq!(err.kind(), SzErrorKind::ReplaceConflict);
    assert_eq!(err.code(), 7245);
}

#[test]
fn from_code_retry_timeout_exceeded() {
    let err = SzError::from_code(10, "test".into());
    assert_eq!(err.kind(), SzErrorKind::RetryTimeoutExceeded);
    assert_eq!(err.code(), 10);
}

#[test]
fn from_code_unhandled() {
    let err = SzError::from_code(87, "test".into());
    assert_eq!(err.kind(), SzErrorKind::Unhandled);
    assert_eq!(err.code(), 87);
}

#[test]
fn from_code_unknown_data_source() {
    let err = SzError::from_code(2207, "test".into());
    assert_eq!(err.kind(), SzErrorKind::UnknownDataSource);
    assert_eq!(err.code(), 2207);
}

#[test]
fn from_code_unknown_code_defaults_to_general() {
    let err = SzError::from_code(999999, "unknown".into());
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), 999999);
}

#[test]
fn from_code_preserves_message() {
    let err = SzError::from_code(2, "my message".into());
    assert_eq!(err.message(), "my message");
}

// ---------------------------------------------------------------------------
// is_bad_input hierarchy: BadInput, NotFound, UnknownDataSource
// ---------------------------------------------------------------------------

#[test]
fn is_bad_input_for_bad_input() {
    let err = SzError::from_code(2, "test".into()); // BadInput
    assert!(err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_retryable());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_bad_input_for_not_found() {
    let err = SzError::from_code(33, "test".into()); // NotFound
    assert!(err.is_bad_input());
}

#[test]
fn is_bad_input_for_unknown_data_source() {
    let err = SzError::from_code(2207, "test".into()); // UnknownDataSource
    assert!(err.is_bad_input());
}

// ---------------------------------------------------------------------------
// is_general hierarchy: General, Configuration, ReplaceConflict, Sdk
// ---------------------------------------------------------------------------

#[test]
fn is_general_for_general() {
    let err = SzError::from_code(5, "test".into()); // General
    assert!(err.is_general());
    assert!(!err.is_bad_input());
    assert!(!err.is_retryable());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_general_for_configuration() {
    let err = SzError::from_code(14, "test".into()); // Configuration
    assert!(err.is_general());
}

#[test]
fn is_general_for_replace_conflict() {
    let err = SzError::from_code(7245, "test".into()); // ReplaceConflict
    assert!(err.is_general());
}

// ---------------------------------------------------------------------------
// is_retryable hierarchy: Retryable, DatabaseConnectionLost,
//                         DatabaseTransient, RetryTimeoutExceeded
// ---------------------------------------------------------------------------

#[test]
fn is_retryable_for_database_connection_lost() {
    let err = SzError::from_code(1006, "test".into()); // DatabaseConnectionLost
    assert!(err.is_retryable());
    assert!(!err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_retryable_for_database_transient() {
    let err = SzError::from_code(1008, "test".into()); // DatabaseTransient
    assert!(err.is_retryable());
}

#[test]
fn is_retryable_for_retry_timeout_exceeded() {
    let err = SzError::from_code(10, "test".into()); // RetryTimeoutExceeded
    assert!(err.is_retryable());
}

// ---------------------------------------------------------------------------
// is_unrecoverable hierarchy: Unrecoverable, Database, License,
//                             NotInitialized, Unhandled
// ---------------------------------------------------------------------------

#[test]
fn is_unrecoverable_for_database() {
    let err = SzError::from_code(1000, "test".into()); // Database
    assert!(err.is_unrecoverable());
    assert!(!err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_retryable());
}

#[test]
fn is_unrecoverable_for_license() {
    let err = SzError::from_code(999, "test".into()); // License
    assert!(err.is_unrecoverable());
}

#[test]
fn is_unrecoverable_for_not_initialized() {
    let err = SzError::from_code(48, "test".into()); // NotInitialized
    assert!(err.is_unrecoverable());
}

#[test]
fn is_unrecoverable_for_unhandled() {
    let err = SzError::from_code(87, "test".into()); // Unhandled
    assert!(err.is_unrecoverable());
}

// ---------------------------------------------------------------------------
// Free functions: downcast from Box<dyn Error>
// ---------------------------------------------------------------------------

#[test]
fn free_fn_is_sz_error_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(2, "test".into()));
    assert!(error::is_sz_error(&*err));
}

#[test]
fn free_fn_is_sz_error_false_for_other_error() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_sz_error(&*err));
}

#[test]
fn free_fn_is_bad_input_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(33, "test".into()));
    assert!(error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_bad_input_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_bad_input_false_for_wrong_category() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(1000, "test".into()));
    assert!(!error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_general_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(14, "test".into()));
    assert!(error::is_general(&*err));
}

#[test]
fn free_fn_is_general_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_general(&*err));
}

#[test]
fn free_fn_is_retryable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(1006, "test".into()));
    assert!(error::is_retryable(&*err));
}

#[test]
fn free_fn_is_retryable_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_retryable(&*err));
}

#[test]
fn free_fn_is_unrecoverable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::from_code(999, "test".into()));
    assert!(error::is_unrecoverable(&*err));
}

#[test]
fn free_fn_is_unrecoverable_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_unrecoverable(&*err));
}

// ---------------------------------------------------------------------------
// Display / std::error::Error trait
// ---------------------------------------------------------------------------

#[test]
fn display_includes_code_and_message() {
    let err = SzError::from_code(2, "Invalid Message".into());
    let display = format!("{err}");
    assert!(display.contains("2"), "should contain error code");
    assert!(
        display.contains("Invalid Message"),
        "should contain message"
    );
}

#[test]
fn sz_error_implements_std_error() {
    let err = SzError::from_code(2, "test".into());
    let _: &dyn std::error::Error = &err;
}
