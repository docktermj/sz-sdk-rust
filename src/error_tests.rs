use super::error::{self, SzComponent, SzError, SzErrorInspect, SzErrorKind, SzResult, SzResultExt};
use std::fmt;

// ---------------------------------------------------------------------------
// from_code: verify each error type classification
// ---------------------------------------------------------------------------

#[test]
fn from_code_bad_input() {
    let err = SzError::new("test").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::BadInput);
    assert_eq!(err.code(), Some(2));
}

#[test]
fn from_code_configuration() {
    let err = SzError::new("test").with_code(14);
    assert_eq!(err.kind(), SzErrorKind::Configuration);
    assert_eq!(err.code(), Some(14));
}

#[test]
fn from_code_database() {
    let err = SzError::new("test").with_code(1000);
    assert_eq!(err.kind(), SzErrorKind::Database);
    assert_eq!(err.code(), Some(1000));
}

#[test]
fn from_code_database_connection_lost() {
    let err = SzError::new("test").with_code(1006);
    assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
    assert_eq!(err.code(), Some(1006));
}

#[test]
fn from_code_database_transient() {
    let err = SzError::new("test").with_code(1008);
    assert_eq!(err.kind(), SzErrorKind::DatabaseTransient);
    assert_eq!(err.code(), Some(1008));
}

#[test]
fn from_code_general() {
    let err = SzError::new("test").with_code(5);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(5));
}

#[test]
fn from_code_license() {
    let err = SzError::new("test").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::License);
    assert_eq!(err.code(), Some(999));
}

#[test]
fn from_code_not_found() {
    let err = SzError::new("test").with_code(33);
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert_eq!(err.code(), Some(33));
}

#[test]
fn from_code_not_initialized() {
    let err = SzError::new("test").with_code(48);
    assert_eq!(err.kind(), SzErrorKind::NotInitialized);
    assert_eq!(err.code(), Some(48));
}

#[test]
fn from_code_replace_conflict() {
    let err = SzError::new("test").with_code(7245);
    assert_eq!(err.kind(), SzErrorKind::ReplaceConflict);
    assert_eq!(err.code(), Some(7245));
}

#[test]
fn from_code_retry_timeout_exceeded() {
    let err = SzError::new("test").with_code(10);
    assert_eq!(err.kind(), SzErrorKind::RetryTimeoutExceeded);
    assert_eq!(err.code(), Some(10));
}

#[test]
fn from_code_unhandled() {
    let err = SzError::new("test").with_code(87);
    assert_eq!(err.kind(), SzErrorKind::Unhandled);
    assert_eq!(err.code(), Some(87));
}

#[test]
fn from_code_unknown_data_source() {
    let err = SzError::new("test").with_code(2207);
    assert_eq!(err.kind(), SzErrorKind::UnknownDataSource);
    assert_eq!(err.code(), Some(2207));
}

#[test]
fn from_code_unknown_code_defaults_to_general() {
    let err = SzError::new("unknown").with_code(999999);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(999999));
}

#[test]
fn from_code_preserves_message() {
    let err = SzError::new("my message").with_code(2);
    assert_eq!(err.message(), "my message");
}

// ---------------------------------------------------------------------------
// new()
// ---------------------------------------------------------------------------

#[test]
fn new_sets_message_and_defaults() {
    let err = SzError::new("something broke");
    assert_eq!(err.message(), "something broke");
    assert_eq!(err.code(), None);
    assert_eq!(err.kind(), SzErrorKind::default());
    assert_eq!(err.kind(), SzErrorKind::SzError);
    assert_eq!(err.component(), None);
}

#[test]
fn new_with_empty_message() {
    let err = SzError::new(String::new());
    assert_eq!(err.message(), "");
    assert_eq!(err.code(), None);
    assert_eq!(err.kind(), SzErrorKind::SzError);
}

// ---------------------------------------------------------------------------
// with_code()
// ---------------------------------------------------------------------------

#[test]
fn with_code_sets_code_and_kind() {
    let err = SzError::new("test").with_code(2);
    assert_eq!(err.code(), Some(2));
    assert_eq!(err.kind(), SzErrorKind::BadInput);
}

#[test]
fn with_code_i64_overflow_defaults_to_general() {
    let err = SzError::new("test").with_code(i64::MAX);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(i64::MAX));
}

#[test]
fn with_code_negative_defaults_to_general() {
    let err = SzError::new("test").with_code(-1);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(-1));
}

#[test]
fn with_code_zero_defaults_to_general() {
    let err = SzError::new("test").with_code(0);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(0));
}

#[test]
fn with_code_unknown_defaults_to_general() {
    let err = SzError::new("test").with_code(999999);
    assert_eq!(err.code(), Some(999999));
    assert_eq!(err.kind(), SzErrorKind::General);
}

#[test]
fn with_code_each_category() {
    let cases: &[(i64, SzErrorKind)] = &[
        (2, SzErrorKind::BadInput),
        (14, SzErrorKind::Configuration),
        (1000, SzErrorKind::Database),
        (1006, SzErrorKind::DatabaseConnectionLost),
        (1008, SzErrorKind::DatabaseTransient),
        (999, SzErrorKind::License),
        (33, SzErrorKind::NotFound),
        (48, SzErrorKind::NotInitialized),
        (7245, SzErrorKind::ReplaceConflict),
        (10, SzErrorKind::RetryTimeoutExceeded),
        (87, SzErrorKind::Unhandled),
        (2207, SzErrorKind::UnknownDataSource),
    ];
    for &(code, expected_kind) in cases {
        let err = SzError::new("test").with_code(code);
        assert_eq!(err.code(), Some(code), "code {code}");
        assert_eq!(err.kind(), expected_kind, "kind for code {code}");
    }
}

#[test]
fn with_code_called_twice_re_derives_kind() {
    let err = SzError::new("test")
        .with_code(2)    // BadInput
        .with_code(999); // License
    assert_eq!(err.kind(), SzErrorKind::License);
    assert_eq!(err.code(), Some(999));
}

#[test]
fn with_code_overrides_previous_code() {
    let err = SzError::new("test").with_code(2).with_code(1006);
    assert_eq!(err.code(), Some(1006));
    assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
}

#[test]
fn with_code_preserves_message_and_component() {
    let err = SzError::new("my message")
        .with_component(SzComponent::Engine)
        .with_code(33);
    assert_eq!(err.message(), "my message");
    assert_eq!(err.component(), Some(SzComponent::Engine));
    assert_eq!(err.code(), Some(33));
    assert_eq!(err.kind(), SzErrorKind::NotFound);
}

// ---------------------------------------------------------------------------
// with_kind
// ---------------------------------------------------------------------------

#[test]
fn with_kind_overrides_default() {
    let err = SzError::new("test").with_kind(SzErrorKind::License);
    assert_eq!(err.kind(), SzErrorKind::License);
}

#[test]
fn with_kind_overrides_code_derived_kind() {
    let err = SzError::new("test")
        .with_code(2) // would set BadInput
        .with_kind(SzErrorKind::Unrecoverable);
    assert_eq!(err.kind(), SzErrorKind::Unrecoverable);
    assert_eq!(err.code(), Some(2));
}

#[test]
fn clone_preserves_kind_explicit() {
    let err = SzError::new("test")
        .with_kind(SzErrorKind::License)
        .with_code(2); // kind stays License because kind_explicit=true
    let cloned = err.clone().with_code(999); // should still stay License
    assert_eq!(cloned.kind(), SzErrorKind::License);
    assert_eq!(cloned.code(), Some(999));
}

#[test]
fn with_kind_before_with_code_preserves_kind() {
    let err = SzError::new("test")
        .with_kind(SzErrorKind::License)
        .with_code(2); // code 2 would normally derive BadInput
    assert_eq!(err.kind(), SzErrorKind::License);
    assert_eq!(err.code(), Some(2));
}

// ---------------------------------------------------------------------------
// with_message
// ---------------------------------------------------------------------------

#[test]
fn with_message_called_twice_overwrites() {
    let err = SzError::new("first").with_message("second");
    assert_eq!(err.message(), "second");
}

#[test]
fn with_component_called_twice_overwrites() {
    let err = SzError::new("test")
        .with_component(SzComponent::Engine)
        .with_component(SzComponent::Config);
    assert_eq!(err.component(), Some(SzComponent::Config));
}

#[test]
fn with_message_overrides_initial() {
    let err = SzError::new("initial").with_message("replaced");
    assert_eq!(err.message(), "replaced");
}

#[test]
fn with_message_accepts_string() {
    let msg = String::from("owned message");
    let err = SzError::new("initial").with_message(msg);
    assert_eq!(err.message(), "owned message");
}

// ---------------------------------------------------------------------------
// From<SzErrorKind> for SzError
// ---------------------------------------------------------------------------

#[test]
fn from_kind_then_with_code_preserves_kind() {
    let err = SzError::from(SzErrorKind::Unrecoverable).with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Unrecoverable);
    assert_eq!(err.code(), Some(2));
}

#[test]
fn from_kind_produces_no_code() {
    let err: SzError = SzErrorKind::BadInput.into();
    assert_eq!(err.code(), None);
    assert_eq!(err.message(), "");
    assert_eq!(err.kind(), SzErrorKind::BadInput);
}

// ---------------------------------------------------------------------------
// SzErrorKind::is_* methods
// ---------------------------------------------------------------------------

#[test]
fn kind_is_bad_input() {
    assert!(SzErrorKind::BadInput.is_bad_input());
    assert!(SzErrorKind::NotFound.is_bad_input());
    assert!(SzErrorKind::UnknownDataSource.is_bad_input());
    assert!(!SzErrorKind::General.is_bad_input());
}

#[test]
fn kind_is_general() {
    assert!(SzErrorKind::General.is_general());
    assert!(SzErrorKind::Configuration.is_general());
    assert!(SzErrorKind::ReplaceConflict.is_general());
    assert!(SzErrorKind::Sdk.is_general());
    assert!(!SzErrorKind::BadInput.is_general());
}

#[test]
fn kind_is_retryable() {
    assert!(SzErrorKind::Retryable.is_retryable());
    assert!(SzErrorKind::DatabaseConnectionLost.is_retryable());
    assert!(SzErrorKind::DatabaseTransient.is_retryable());
    assert!(SzErrorKind::RetryTimeoutExceeded.is_retryable());
    assert!(!SzErrorKind::General.is_retryable());
}

#[test]
fn kind_is_sz_error() {
    // Every variant is an SzError — it's the root of the hierarchy.
    assert!(SzErrorKind::SzError.is_sz_error());
    assert!(SzErrorKind::BadInput.is_sz_error());
    assert!(SzErrorKind::NotFound.is_sz_error());
    assert!(SzErrorKind::UnknownDataSource.is_sz_error());
    assert!(SzErrorKind::General.is_sz_error());
    assert!(SzErrorKind::Configuration.is_sz_error());
    assert!(SzErrorKind::ReplaceConflict.is_sz_error());
    assert!(SzErrorKind::Sdk.is_sz_error());
    assert!(SzErrorKind::Retryable.is_sz_error());
    assert!(SzErrorKind::DatabaseConnectionLost.is_sz_error());
    assert!(SzErrorKind::DatabaseTransient.is_sz_error());
    assert!(SzErrorKind::RetryTimeoutExceeded.is_sz_error());
    assert!(SzErrorKind::Unrecoverable.is_sz_error());
    assert!(SzErrorKind::Database.is_sz_error());
    assert!(SzErrorKind::License.is_sz_error());
    assert!(SzErrorKind::NotInitialized.is_sz_error());
    assert!(SzErrorKind::Unhandled.is_sz_error());
}

#[test]
fn kind_sz_error_is_not_in_subcategories() {
    // SzError is the root — it does not belong to any subcategory.
    assert!(!SzErrorKind::SzError.is_bad_input());
    assert!(!SzErrorKind::SzError.is_general());
    assert!(!SzErrorKind::SzError.is_retryable());
    assert!(!SzErrorKind::SzError.is_unrecoverable());
}

#[test]
fn kind_is_unrecoverable() {
    assert!(SzErrorKind::Unrecoverable.is_unrecoverable());
    assert!(SzErrorKind::Database.is_unrecoverable());
    assert!(SzErrorKind::License.is_unrecoverable());
    assert!(SzErrorKind::NotInitialized.is_unrecoverable());
    assert!(SzErrorKind::Unhandled.is_unrecoverable());
    assert!(!SzErrorKind::General.is_unrecoverable());
}

// ---------------------------------------------------------------------------
// SzErrorKind::severity
// ---------------------------------------------------------------------------

#[test]
fn kind_severity_critical() {
    assert_eq!(SzErrorKind::License.severity(), "critical");
    assert_eq!(SzErrorKind::Unrecoverable.severity(), "critical");
    assert_eq!(SzErrorKind::Unhandled.severity(), "critical");
}

#[test]
fn kind_severity_high() {
    assert_eq!(SzErrorKind::Database.severity(), "high");
    assert_eq!(SzErrorKind::NotInitialized.severity(), "high");
}

#[test]
fn kind_severity_medium() {
    assert_eq!(SzErrorKind::Configuration.severity(), "medium");
    assert_eq!(SzErrorKind::DatabaseConnectionLost.severity(), "medium");
    assert_eq!(SzErrorKind::DatabaseTransient.severity(), "medium");
}

#[test]
fn kind_severity_low() {
    assert_eq!(SzErrorKind::BadInput.severity(), "low");
    assert_eq!(SzErrorKind::General.severity(), "low");
    assert_eq!(SzErrorKind::NotFound.severity(), "low");
    assert_eq!(SzErrorKind::ReplaceConflict.severity(), "low");
    assert_eq!(SzErrorKind::Retryable.severity(), "low");
    assert_eq!(SzErrorKind::RetryTimeoutExceeded.severity(), "low");
    assert_eq!(SzErrorKind::Sdk.severity(), "low");
    assert_eq!(SzErrorKind::SzError.severity(), "low");
    assert_eq!(SzErrorKind::UnknownDataSource.severity(), "low");
}

// ---------------------------------------------------------------------------
// SzError::severity
// ---------------------------------------------------------------------------

#[test]
fn sz_error_severity_delegates_to_kind() {
    assert_eq!(
        SzError::new("test").with_code(999).severity(),
        "critical"
    ); // License
    assert_eq!(SzError::new("test").with_code(1000).severity(), "high"); // Database
    assert_eq!(SzError::new("test").with_code(1006).severity(), "medium"); // DatabaseConnectionLost
    assert_eq!(SzError::new("test").with_code(2).severity(), "low"); // BadInput
}

// ---------------------------------------------------------------------------
// Free function: severity
// ---------------------------------------------------------------------------

#[test]
fn free_fn_severity_returns_some() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(999));
    assert_eq!(error::severity(&*err), Some("critical"));
}

#[test]
fn free_fn_severity_returns_none_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert_eq!(error::severity(&*err), None);
}

// ---------------------------------------------------------------------------
// SzErrorKind Display
// ---------------------------------------------------------------------------

#[test]
fn kind_display() {
    assert_eq!(format!("{}", SzErrorKind::BadInput), "bad input");
    assert_eq!(format!("{}", SzErrorKind::Database), "database error");
    assert_eq!(format!("{}", SzErrorKind::Sdk), "SDK error");
    assert_eq!(format!("{}", SzErrorKind::SzError), "SzError error");
}

// ---------------------------------------------------------------------------
// Clone
// ---------------------------------------------------------------------------

#[test]
fn clone_preserves_fields() {
    let err = SzError::new("test").with_code(2);
    let cloned = err.clone();
    assert_eq!(cloned.code(), Some(2));
    assert_eq!(cloned.message(), "test");
    assert_eq!(cloned.kind(), SzErrorKind::BadInput);
}

// ---------------------------------------------------------------------------
// SzComponent Display
// ---------------------------------------------------------------------------

#[test]
fn component_display() {
    assert_eq!(format!("{}", SzComponent::Config), "SzConfig");
    assert_eq!(format!("{}", SzComponent::ConfigManager), "SzConfigManager");
    assert_eq!(format!("{}", SzComponent::Diagnostic), "SzDiagnostic");
    assert_eq!(format!("{}", SzComponent::Engine), "SzEngine");
    assert_eq!(format!("{}", SzComponent::Product), "SzProduct");
}

// ---------------------------------------------------------------------------
// with_component / component()
// ---------------------------------------------------------------------------

#[test]
fn with_component_sets_component() {
    let err = SzError::new("test").with_code(2).with_component(SzComponent::Engine);
    assert_eq!(err.component(), Some(SzComponent::Engine));
}

#[test]
fn component_is_none_by_default() {
    let err = SzError::new("test").with_code(2);
    assert_eq!(err.component(), None);
}

#[test]
fn component_is_none_for_from_kind() {
    let err: SzError = SzErrorKind::BadInput.into();
    assert_eq!(err.component(), None);
}

#[test]
fn clone_preserves_component() {
    let err = SzError::new("test").with_code(2).with_component(SzComponent::Diagnostic);
    let cloned = err.clone();
    assert_eq!(cloned.component(), Some(SzComponent::Diagnostic));
}

#[test]
fn with_component_all_variants() {
    for (component, expected) in [
        (SzComponent::Config, "SzConfig"),
        (SzComponent::ConfigManager, "SzConfigManager"),
        (SzComponent::Diagnostic, "SzDiagnostic"),
        (SzComponent::Engine, "SzEngine"),
        (SzComponent::Product, "SzProduct"),
    ] {
        let err = SzError::new("test").with_code(5).with_component(component);
        assert_eq!(err.component(), Some(component));
        assert_eq!(format!("{}", component), expected);
    }
}

#[test]
fn free_fn_component_returns_some() {
    let err: Box<dyn std::error::Error> =
        Box::new(SzError::new("test").with_code(2).with_component(SzComponent::Engine));
    assert_eq!(error::component(&*err), Some(SzComponent::Engine));
}

#[test]
fn free_fn_component_returns_none_for_no_component() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(2));
    assert_eq!(err.downcast_ref::<SzError>().unwrap().component(), None);
    assert_eq!(error::component(&*err), None);
}

#[test]
fn free_fn_component_returns_none_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert_eq!(error::component(&*err), None);
}

// ---------------------------------------------------------------------------
// component_name()
// ---------------------------------------------------------------------------

#[test]
fn component_name_returns_name_when_set() {
    let err = SzError::new("test").with_code(2).with_component(SzComponent::Engine);
    assert_eq!(err.component_name(), "SzEngine");
}

#[test]
fn component_name_returns_empty_when_none() {
    let err = SzError::new("test").with_code(2);
    assert_eq!(err.component_name(), "");
}

#[test]
fn component_name_all_variants() {
    for (component, expected) in [
        (SzComponent::Config, "SzConfig"),
        (SzComponent::ConfigManager, "SzConfigManager"),
        (SzComponent::Diagnostic, "SzDiagnostic"),
        (SzComponent::Engine, "SzEngine"),
        (SzComponent::Product, "SzProduct"),
    ] {
        let err = SzError::new("test").with_code(5).with_component(component);
        assert_eq!(err.component_name(), expected);
    }
}

#[test]
fn component_name_matches_display() {
    let err = SzError::new("test").with_code(2).with_component(SzComponent::Engine);
    assert_eq!(err.component_name(), format!("{}", SzComponent::Engine));
}

// ---------------------------------------------------------------------------
// with_source / Error::source()
// ---------------------------------------------------------------------------

#[test]
fn with_source_chains_cause() {
    let cause = std::io::Error::other("disk full");
    let err = SzError::new("db failed").with_code(1000).with_source(cause);
    let source = std::error::Error::source(&err).expect("should have a source");
    assert!(source.to_string().contains("disk full"));
}

#[test]
fn source_is_none_by_default() {
    let err = SzError::new("test").with_code(2);
    assert!(std::error::Error::source(&err).is_none());
}

#[test]
fn clone_drops_source() {
    let cause = std::io::Error::other("disk full");
    let err = SzError::new("db failed").with_code(1000).with_source(cause);
    let cloned = err.clone();
    assert!(std::error::Error::source(&cloned).is_none());
}

// ---------------------------------------------------------------------------
// is_bad_input hierarchy: BadInput, NotFound, UnknownDataSource
// ---------------------------------------------------------------------------

#[test]
fn is_bad_input_for_bad_input() {
    let err = SzError::new("test").with_code(2); // BadInput
    assert!(err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_retryable());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_bad_input_for_not_found() {
    let err = SzError::new("test").with_code(33); // NotFound
    assert!(err.is_bad_input());
}

#[test]
fn is_bad_input_for_unknown_data_source() {
    let err = SzError::new("test").with_code(2207); // UnknownDataSource
    assert!(err.is_bad_input());
}

// ---------------------------------------------------------------------------
// is_general hierarchy: General, Configuration, ReplaceConflict, Sdk
// ---------------------------------------------------------------------------

#[test]
fn is_general_for_general() {
    let err = SzError::new("test").with_code(5); // General
    assert!(err.is_general());
    assert!(!err.is_bad_input());
    assert!(!err.is_retryable());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_general_for_configuration() {
    let err = SzError::new("test").with_code(14); // Configuration
    assert!(err.is_general());
}

#[test]
fn is_general_for_replace_conflict() {
    let err = SzError::new("test").with_code(7245); // ReplaceConflict
    assert!(err.is_general());
}

// ---------------------------------------------------------------------------
// is_retryable hierarchy: Retryable, DatabaseConnectionLost,
//                         DatabaseTransient, RetryTimeoutExceeded
// ---------------------------------------------------------------------------

#[test]
fn is_retryable_for_database_connection_lost() {
    let err = SzError::new("test").with_code(1006); // DatabaseConnectionLost
    assert!(err.is_retryable());
    assert!(!err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_unrecoverable());
}

#[test]
fn is_retryable_for_database_transient() {
    let err = SzError::new("test").with_code(1008); // DatabaseTransient
    assert!(err.is_retryable());
}

#[test]
fn is_retryable_for_retry_timeout_exceeded() {
    let err = SzError::new("test").with_code(10); // RetryTimeoutExceeded
    assert!(err.is_retryable());
}

// ---------------------------------------------------------------------------
// is_unrecoverable hierarchy: Unrecoverable, Database, License,
//                             NotInitialized, Unhandled
// ---------------------------------------------------------------------------

#[test]
fn is_unrecoverable_for_database() {
    let err = SzError::new("test").with_code(1000); // Database
    assert!(err.is_unrecoverable());
    assert!(!err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_retryable());
}

#[test]
fn is_unrecoverable_for_license() {
    let err = SzError::new("test").with_code(999); // License
    assert!(err.is_unrecoverable());
}

#[test]
fn is_unrecoverable_for_not_initialized() {
    let err = SzError::new("test").with_code(48); // NotInitialized
    assert!(err.is_unrecoverable());
}

#[test]
fn is_unrecoverable_for_unhandled() {
    let err = SzError::new("test").with_code(87); // Unhandled
    assert!(err.is_unrecoverable());
}

// ---------------------------------------------------------------------------
// Free functions: downcast from Box<dyn Error>
// ---------------------------------------------------------------------------

#[test]
fn free_fn_is_sz_error_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(2));
    assert!(error::is_sz_error(&*err));
}

#[test]
fn free_fn_is_sz_error_false_for_other_error() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_sz_error(&*err));
}

#[test]
fn free_fn_is_bad_input_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(33));
    assert!(error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_bad_input_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_bad_input_false_for_wrong_category() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1000));
    assert!(!error::is_bad_input(&*err));
}

#[test]
fn free_fn_is_general_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(14));
    assert!(error::is_general(&*err));
}

#[test]
fn free_fn_is_general_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_general(&*err));
}

#[test]
fn free_fn_is_retryable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1006));
    assert!(error::is_retryable(&*err));
}

#[test]
fn free_fn_is_retryable_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_retryable(&*err));
}

#[test]
fn free_fn_is_unrecoverable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(999));
    assert!(error::is_unrecoverable(&*err));
}

#[test]
fn free_fn_is_unrecoverable_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is_unrecoverable(&*err));
}

// ---------------------------------------------------------------------------
// Hierarchy-aware `is` — SzErrorKind, SzError, and free function
// ---------------------------------------------------------------------------

#[test]
fn kind_is_retryable_matches_children() {
    assert!(SzErrorKind::Retryable.is(SzErrorKind::Retryable));
    assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::Retryable));
    assert!(SzErrorKind::DatabaseTransient.is(SzErrorKind::Retryable));
    assert!(SzErrorKind::RetryTimeoutExceeded.is(SzErrorKind::Retryable));
    assert!(!SzErrorKind::General.is(SzErrorKind::Retryable));
    assert!(!SzErrorKind::BadInput.is(SzErrorKind::Retryable));
}

#[test]
fn kind_is_bad_input_matches_children() {
    assert!(SzErrorKind::BadInput.is(SzErrorKind::BadInput));
    assert!(SzErrorKind::NotFound.is(SzErrorKind::BadInput));
    assert!(SzErrorKind::UnknownDataSource.is(SzErrorKind::BadInput));
    assert!(!SzErrorKind::Retryable.is(SzErrorKind::BadInput));
}

#[test]
fn kind_is_general_matches_children() {
    assert!(SzErrorKind::General.is(SzErrorKind::General));
    assert!(SzErrorKind::Configuration.is(SzErrorKind::General));
    assert!(SzErrorKind::ReplaceConflict.is(SzErrorKind::General));
    assert!(SzErrorKind::Sdk.is(SzErrorKind::General));
    assert!(!SzErrorKind::Database.is(SzErrorKind::General));
}

#[test]
fn kind_is_unrecoverable_matches_children() {
    assert!(SzErrorKind::Unrecoverable.is(SzErrorKind::Unrecoverable));
    assert!(SzErrorKind::Database.is(SzErrorKind::Unrecoverable));
    assert!(SzErrorKind::License.is(SzErrorKind::Unrecoverable));
    assert!(SzErrorKind::NotInitialized.is(SzErrorKind::Unrecoverable));
    assert!(SzErrorKind::Unhandled.is(SzErrorKind::Unrecoverable));
    assert!(!SzErrorKind::Retryable.is(SzErrorKind::Unrecoverable));
}

#[test]
fn kind_is_sz_error_matches_all() {
    // SzError is the root — every variant matches via is().
    assert!(SzErrorKind::SzError.is(SzErrorKind::SzError));
    assert!(SzErrorKind::BadInput.is(SzErrorKind::SzError));
    assert!(SzErrorKind::NotFound.is(SzErrorKind::SzError));
    assert!(SzErrorKind::UnknownDataSource.is(SzErrorKind::SzError));
    assert!(SzErrorKind::General.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Configuration.is(SzErrorKind::SzError));
    assert!(SzErrorKind::ReplaceConflict.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Sdk.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Retryable.is(SzErrorKind::SzError));
    assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::SzError));
    assert!(SzErrorKind::DatabaseTransient.is(SzErrorKind::SzError));
    assert!(SzErrorKind::RetryTimeoutExceeded.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Unrecoverable.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Database.is(SzErrorKind::SzError));
    assert!(SzErrorKind::License.is(SzErrorKind::SzError));
    assert!(SzErrorKind::NotInitialized.is(SzErrorKind::SzError));
    assert!(SzErrorKind::Unhandled.is(SzErrorKind::SzError));
}

#[test]
fn kind_is_leaf_is_exact() {
    assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::DatabaseConnectionLost));
    assert!(!SzErrorKind::DatabaseTransient.is(SzErrorKind::DatabaseConnectionLost));
    assert!(!SzErrorKind::Retryable.is(SzErrorKind::DatabaseConnectionLost));
}

#[test]
fn sz_error_is_hierarchy_aware() {
    let err = SzError::new("test").with_code(1006); // DatabaseConnectionLost
    assert!(err.is(SzErrorKind::Retryable));
    assert!(err.is(SzErrorKind::DatabaseConnectionLost));
    assert!(err.is(SzErrorKind::SzError));
    assert!(!err.is(SzErrorKind::BadInput));
    assert!(!err.is(SzErrorKind::DatabaseTransient));
}

#[test]
fn sz_error_is_sz_error_always_true() {
    // Every SzError is a Senzing error regardless of kind.
    assert!(SzError::new("test").with_code(2).is_sz_error()); // BadInput
    assert!(SzError::new("test").with_code(14).is_sz_error()); // Configuration
    assert!(SzError::new("test").with_code(1006).is_sz_error()); // DatabaseConnectionLost
    assert!(SzError::new("test").with_code(999).is_sz_error()); // License
    assert!(SzError::new(String::new()).is_sz_error());
}

#[test]
fn sz_error_from_sz_error_kind() {
    let err: SzError = SzErrorKind::SzError.into();
    assert_eq!(err.code(), None);
    assert_eq!(err.message(), "");
    assert_eq!(err.kind(), SzErrorKind::SzError);
    assert!(err.is_sz_error());
    assert!(!err.is_bad_input());
    assert!(!err.is_general());
    assert!(!err.is_retryable());
    assert!(!err.is_unrecoverable());
}

#[test]
fn free_fn_is_hierarchy_aware() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1006));
    assert!(error::is(&*err, SzErrorKind::Retryable));
    assert!(error::is(&*err, SzErrorKind::DatabaseConnectionLost));
    assert!(!error::is(&*err, SzErrorKind::BadInput));
}

#[test]
fn free_fn_is_sz_error_kind_matches_all() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1006));
    assert!(error::is(&*err, SzErrorKind::SzError));

    let err2: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(2));
    assert!(error::is(&*err2, SzErrorKind::SzError));

    let non_sz: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is(&*non_sz, SzErrorKind::SzError));
}

#[test]
fn free_fn_is_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!error::is(&*err, SzErrorKind::Retryable));
    assert!(!error::is(&*err, SzErrorKind::General));
}

// ---------------------------------------------------------------------------
// Display / std::error::Error trait
// ---------------------------------------------------------------------------

#[test]
fn display_without_code() {
    let err = SzError::new("something failed");
    let display = format!("{err}");
    assert_eq!(display, "SzError error: something failed");
}

#[test]
fn display_includes_code_and_message() {
    let err = SzError::new("Invalid Message").with_code(2);
    let display = format!("{err}");
    assert!(display.contains("2"), "should contain error code");
    assert!(
        display.contains("Invalid Message"),
        "should contain message"
    );
}

#[test]
fn sz_error_implements_std_error() {
    let err = SzError::new("test").with_code(2);
    let _: &dyn std::error::Error = &err;
}

// ---------------------------------------------------------------------------
// Examples of handling SzError or Box<dyn std::error::Error>
// ---------------------------------------------------------------------------

#[test]
fn example_error_handling_with_multiple_err_match_arms_as_error_simplified() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) if err.is_sz_error() => println!("SzError: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn example_error_handling_with_multiple_err_match_arms_as_error_detailed() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) if err.is_sz(SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(err) if err.is_sz_bad_input() => println!("Bad input: {err}"),
        Err(err) if err.is_sz_retryable() => println!("Retryable: {err}"),
        Err(err) if err.is_sz_unrecoverable() => println!("Unrecoverable: {err}"),
        Err(err) if err.is_sz_general() => println!("General: {err}"),
        Err(err) if err.is_sz_error() => println!("SzError: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn example_error_handling_with_two_level_match_as_sz_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => match err.sz_error() {
            Some(szerr) => {
                println!(
                    "SzError kind: {}, code: {:?}, retryable: {}, component: {}, severity: {}, message: {}",
                    szerr.kind(),
                    szerr.code(),
                    szerr.is_retryable(),
                    szerr.component_name(),
                    szerr.severity(),
                    szerr.message()
                );
            }
            None => println!("Non-SzError error: {err}"),
        },
    }
}

#[test]
fn example_error_handling_with_match_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => match () {
            _ if err.is_sz_bad_input() => println!("Bad input: {err}"),
            _ if err.is_sz_retryable() => println!("Retryable: {err}"),
            _ if err.is_sz_unrecoverable() => println!("Unrecoverable: {err}"),
            _ if err.is_sz_general() => println!("General: {err}"),
            _ if err.is_sz_error() => println!("SzError: {err}"),
            _ => println!("Non-SzError error: {err}"),
        },
    }
}

#[test]
fn example_error_handling_with_two_level_match_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => match () {
            _ if error::is(&*err, SzErrorKind::DatabaseConnectionLost) => {
                println!("Database connection lost — reconnecting: {err}");
            }
            _ if error::is_bad_input(&*err) => println!("Bad input: {err}"),
            _ if error::is_retryable(&*err) => println!("Retryable: {err}"),
            _ if error::is_unrecoverable(&*err) => println!("Unrecoverable: {err}"),
            _ if error::is_general(&*err) => println!("General: {err}"),
            _ if error::is_sz_error(&*err) => println!("SzError: {err}"),
            _ => println!("Non-SzError error: {err}"),
        },
    }
}

#[test]
fn example_error_handling_with_match_and_multiple_err_match_arms_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) if error::is(&*err, SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(err) if error::is_bad_input(&*err) => println!("Bad input: {err}"),
        Err(err) if error::is_retryable(&*err) => println!("Retryable: {err}"),
        Err(err) if error::is_unrecoverable(&*err) => println!("Unrecoverable: {err}"),
        Err(err) if error::is_general(&*err) => println!("General: {err}"),
        Err(err) if error::is_sz_error(&*err) => println!("SzError: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn example_error_handling_with_match_and_multiple_err_match_arms_as_error_2() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) if error::is(&*err, SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(err) if error::is_bad_input(&*err) => println!("Bad input: {err}"),
        Err(err) if error::is_retryable(&*err) => println!("Retryable: {err}"),
        Err(err) if error::is_unrecoverable(&*err) => println!("Unrecoverable: {err}"),
        Err(err) if error::is_general(&*err) => println!("General: {err}"),
        Err(err) if error::is_sz_error(&*err) => println!("General: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn example_error_handling_with_match_and_if_else_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => {
            if error::is_sz_error(&*err) {
                println!(
                    "SzError (bad_input: {}, retryable: {}, unrecoverable: {}): {err}",
                    error::is_bad_input(&*err),
                    error::is_retryable(&*err),
                    error::is_unrecoverable(&*err),
                );
            } else {
                println!("Non-SzError error: {err}");
            }
        }
    }
}

#[test]
fn example_error_handling_with_match_and_if_else_as_sz_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => {
            if let Some(szerr) = error::as_sz_error(&*err) {
                println!(
                    "SzError kind: {}, code: {:?}, retryable: {}, component: {}, severity: {}, message: {}",
                    szerr.kind(),
                    szerr.code(),
                    szerr.is_retryable(),
                    szerr.component_name(),
                    szerr.severity(),
                    szerr.message()
                );
            } else {
                println!("Non-SzError error: {err}");
            }
        }
    }
}

#[test]
fn example_match_on_kind_as_sz_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => {
            if let Some(szerr) = error::as_sz_error(&*err) {
                match szerr.kind() {
                    SzErrorKind::NotFound | SzErrorKind::UnknownDataSource => {
                        assert_eq!(szerr.code(), Some(33));
                    }
                    SzErrorKind::DatabaseConnectionLost | SzErrorKind::DatabaseTransient => {
                        panic!("unexpected retryable error");
                    }
                    kind if kind.is_unrecoverable() => {
                        panic!("unexpected unrecoverable error: {kind}");
                    }
                    other => {
                        panic!("unexpected kind: {other}");
                    }
                }
            } else {
                println!("Non-SzError error: {err}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Error propagation with `?` and Box<dyn std::error::Error>
// ---------------------------------------------------------------------------

/// Simulates a child that fails with an SzError.
fn example_senzing_function() -> Result<String, SzError> {
    Err(SzError::new("record not found").with_code(33))
}

/// Simulates a child that fails with a non-Senzing error.
fn example_non_senzing_function() -> Result<String, std::io::Error> {
    Err(std::io::Error::other("disk full"))
}

/// Parent that calls both children using `?`.  Both error types auto-convert
/// into `Box<dyn Error>` so the caller gets a single unified return type.
fn example_customer_function() -> Result<String, Box<dyn std::error::Error>> {
    let value = example_senzing_function()?;
    let _ = example_non_senzing_function()?;
    Ok(value)
}

#[test]
fn propagation_with_unwrap_err_and_as_sz_error() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // Differentiate: try to downcast to SzError first.
    if let Some(sz) = error::as_sz_error(&*err) {
        // We have full access to Senzing-specific fields.
        assert_eq!(sz.kind(), SzErrorKind::NotFound);
        assert_eq!(sz.code(), Some(33));
        assert_eq!(sz.message(), "record not found");
        assert!(sz.is_bad_input());
        assert!(sz.is_kind(SzErrorKind::NotFound));
        assert!(!sz.is_kind(SzErrorKind::DatabaseConnectionLost));
    } else {
        panic!("expected an SzError");
    }

    // The free functions also work on Box<dyn Error>.
    assert!(error::is_sz_error(&*err));
    assert!(error::is_bad_input(&*err));
}

#[test]
fn propagation_with_unwrap_err_and_downcast() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // Differentiate: try to downcast to SzError first.
    if let Some(sz) = err.downcast_ref::<SzError>() {
        // We have full access to Senzing-specific fields.
        assert_eq!(sz.kind(), SzErrorKind::NotFound);
        assert_eq!(sz.code(), Some(33));
        assert_eq!(sz.message(), "record not found");
        assert!(sz.is_bad_input());
        assert!(sz.is_kind(SzErrorKind::NotFound));
        assert!(!sz.is_kind(SzErrorKind::DatabaseConnectionLost));
    } else {
        panic!("expected an SzError");
    }

    // The free functions also work on Box<dyn Error>.
    assert!(error::is_sz_error(&*err));
    assert!(error::is_bad_input(&*err));
}

#[test]
fn propagation_match_on_kind() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // Downcast to SzError, then match on kind() to choose a recovery strategy.
    if let Some(sz) = err.downcast_ref::<SzError>() {
        match sz.kind() {
            SzErrorKind::NotFound | SzErrorKind::UnknownDataSource => {
                // Handle "not found" style errors.
                assert_eq!(sz.code(), Some(33));
            }
            SzErrorKind::DatabaseConnectionLost | SzErrorKind::DatabaseTransient => {
                panic!("unexpected retryable error");
            }
            other => {
                panic!("unexpected kind: {other}");
            }
        }
    } else if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
        panic!("unexpected io error: {io_err}");
    } else {
        panic!("unknown error type");
    }
}

#[test]
fn propagation_using_free_functions() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // ── Approach 1: free functions ──────────────────────────────────────
    // The caller never writes `downcast_ref` — the free functions handle
    // the downcasting internally and return a simple bool.
    assert!(error::is_sz_error(&*err));
    assert!(error::is_bad_input(&*err));
    assert!(!error::is_retryable(&*err));
    assert!(!error::is_unrecoverable(&*err));

    // Free functions also safely return false for non-Senzing errors.
    let io_err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("oops"));
    assert!(!error::is_sz_error(&*io_err));
    assert!(!error::is_bad_input(&*io_err));
}

#[test]
fn propagation_using_free_functions_2() {
    // ── Approach 2: Box::downcast() ────────────────────────────────────
    // Consumes the Box<dyn Error> and returns Result<Box<SzError>, Box<dyn Error>>.
    // On success you get direct field access; on failure the box is returned
    // for further attempts (e.g. downcast to std::io::Error).
    let result = example_customer_function();
    let err = result.unwrap_err();

    match err.downcast::<SzError>() {
        Ok(sz) => {
            assert_eq!(sz.kind(), SzErrorKind::NotFound);
            assert_eq!(sz.code(), Some(33));
            assert_eq!(sz.message(), "record not found");
            assert!(sz.is_bad_input());
        }
        Err(other) => {
            panic!("expected SzError, got: {other}");
        }
    }

    // When the downcast fails, the original box is returned for a second attempt.
    let non_sz: Box<dyn std::error::Error> = Box::new(std::io::Error::other("disk full"));
    match non_sz.downcast::<SzError>() {
        Ok(_) => panic!("should not be an SzError"),
        Err(original) => {
            // The original box is intact — try another type.
            let io = original
                .downcast::<std::io::Error>()
                .expect("should be an io::Error");
            assert_eq!(io.to_string(), "disk full");
        }
    }
}

#[test]
fn propagation_with_2_level_match_using_downcast() {
    match example_customer_function() {
        Ok(the_string) => println!("{}", the_string),
        Err(err) => match err.downcast::<SzError>() {
            Ok(sz) => {
                println!(
                    "SzError [{:?}] (kind: {}, bad_input: {}, retryable: {}, unrecoverable: {}): {}",
                    sz.code(),
                    sz.kind(),
                    sz.is_bad_input(),
                    sz.is_retryable(),
                    sz.is_unrecoverable(),
                    sz.message(),
                );
            }
            Err(other) => {
                println!("Other error: {other}");
            }
        },
    }
}

#[test]
fn propagation_with_two_level_match_no_downcast() {
    match example_customer_function() {
        Ok(the_string) => println!("{}", the_string),
        Err(err) => match () {
            _ if error::is_bad_input(&*err) => {
                println!("Bad input: {err}");
            }
            _ if error::is_retryable(&*err) => {
                println!("Retryable: {err}");
            }
            _ if error::is_unrecoverable(&*err) => {
                println!("Unrecoverable: {err}");
            }
            _ if error::is_general(&*err) => {
                println!("General: {err}");
            }
            _ => {
                println!("Other error: {err}");
            }
        },
    }
}

#[test]
// Note: Although this is a testcase, it is not a good example of use.
// Reason:  the Match "_" catches non-SzErrors AND successful results.
fn propagation_with_unwrap_err_and_match() {
    let result = example_customer_function();
    let err = result.unwrap_err();
    match () {
        _ if error::is_kind(&*err, SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        _ if error::is_bad_input(&*err) => println!("Bad input: {err}"),
        _ if error::is_retryable(&*err) => println!("Retryable: {err}"),
        _ if error::is_unrecoverable(&*err) => println!("Unrecoverable: {err}"),
        _ if error::is_general(&*err) => println!("General: {err}"),
        _ if error::is_sz_error(&*err) => println!("SzError: {err}"),
        _ => println!("Either no error or non-SzError error: {err}"),
    }
}

#[test]
// Similar to example_error_handling_with_match_and_multiple_err_as_error,
// but uses "Err(ref err) instead of Err(err)".
// Not considered best practice, but kept as a test.
fn propagation_with_match_and_multiple_ref_err_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(ref err) if error::is_kind(&**err, SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(ref err) if error::is_bad_input(&**err) => println!("Bad input: {err}"),
        Err(ref err) if error::is_retryable(&**err) => println!("Retryable: {err}"),
        Err(ref err) if error::is_unrecoverable(&**err) => println!("Unrecoverable: {err}"),
        Err(ref err) if error::is_general(&**err) => println!("General: {err}"),
        Err(ref err) if error::is_sz_error(&**err) => println!("SzError: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn verify_that_error_is_works() {
    let result = example_customer_function();
    let err = result.unwrap_err();
    assert!(err.is::<SzError>());
}

// ---------------------------------------------------------------------------
// SzErrorInspect extension trait
// ---------------------------------------------------------------------------

/// A simple wrapper error for testing chain walking.
#[derive(Debug)]
struct WrapperError(Box<dyn std::error::Error + Send + Sync>);

impl fmt::Display for WrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wrapper: {}", self.0)
    }
}

impl std::error::Error for WrapperError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}

#[test]
fn inspect_sz_error_on_direct_sz_error() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("not found").with_code(33));
    assert!(err.sz_error().is_some());
    assert_eq!(err.sz_error().unwrap().code(), Some(33));
}

#[test]
fn inspect_sz_error_returns_none_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(err.sz_error().is_none());
}

#[test]
fn inspect_sz_error_through_chain() {
    let sz = SzError::new("connection lost").with_code(1006);
    let wrapped: Box<dyn std::error::Error> = Box::new(WrapperError(Box::new(sz)));
    let found = wrapped.sz_error().expect("should find SzError in chain");
    assert_eq!(found.code(), Some(1006));
    assert_eq!(found.kind(), SzErrorKind::DatabaseConnectionLost);
}

#[test]
fn inspect_is_sz_retryable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1006));
    assert!(err.is_sz_retryable());
}

#[test]
fn inspect_is_sz_retryable_false_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!err.is_sz_retryable());
}

#[test]
fn inspect_is_sz_unrecoverable_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("license").with_code(999));
    assert!(err.is_sz_unrecoverable());
}

#[test]
fn inspect_is_sz_bad_input_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("not found").with_code(33));
    assert!(err.is_sz_bad_input());
}

#[test]
fn inspect_is_sz_general_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("config").with_code(14));
    assert!(err.is_sz_general());
}

#[test]
fn inspect_is_sz_hierarchy_aware() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(1006));
    assert!(err.is_sz(SzErrorKind::Retryable));
    assert!(err.is_sz(SzErrorKind::DatabaseConnectionLost));
    assert!(err.is_sz(SzErrorKind::SzError));
    assert!(!err.is_sz(SzErrorKind::BadInput));
}

#[test]
fn inspect_is_sz_error_kind_through_chain() {
    let sz = SzError::new("not found").with_code(33);
    let wrapped: Box<dyn std::error::Error> = Box::new(WrapperError(Box::new(sz)));
    assert!(wrapped.is_sz(SzErrorKind::SzError));
    assert!(wrapped.is_sz(SzErrorKind::BadInput));
    assert!(!wrapped.is_sz(SzErrorKind::Retryable));
}

#[test]
fn inspect_chain_walking_through_wrapper() {
    let sz = SzError::new("transient").with_code(1008);
    let wrapped: Box<dyn std::error::Error> = Box::new(WrapperError(Box::new(sz)));
    assert!(wrapped.is_sz_retryable());
    assert!(!wrapped.is_sz_bad_input());
    assert!(wrapped.is_sz(SzErrorKind::DatabaseTransient));
}

#[test]
fn inspect_send_sync_dyn_error() {
    let err: Box<dyn std::error::Error + Send + Sync> =
        Box::new(SzError::new("test").with_code(33));
    assert!(err.is_sz_bad_input());
    assert!(!err.is_sz_retryable());
}

// ---------------------------------------------------------------------------
// SzResult type alias
// ---------------------------------------------------------------------------

#[test]
fn sz_result_ok_is_result() {
    let r: SzResult<i32> = Ok(42);
    assert_eq!(r.unwrap(), 42);
}

#[test]
fn sz_result_err_is_result() {
    let r: SzResult<i32> = Err(SzError::new("boom").with_code(2));
    assert!(r.is_err());
}

// ---------------------------------------------------------------------------
// SzResultExt
// ---------------------------------------------------------------------------

#[test]
fn or_retry_recovers_retryable() {
    let r: SzResult<String> = Err(SzError::new("transient").with_code(1008));
    let recovered = r.or_retry(|_| Ok("recovered".into()));
    assert_eq!(recovered.unwrap(), "recovered");
}

#[test]
fn or_retry_propagates_non_retryable() {
    let r: SzResult<String> = Err(SzError::new("bad").with_code(2));
    let result = r.or_retry(|_| Ok("should not reach".into()));
    assert!(result.is_err());
    assert!(result.unwrap_err().is_bad_input());
}

#[test]
fn or_retry_passes_through_ok() {
    let r: SzResult<String> = Ok("hello".into());
    let result = r.or_retry(|_| Ok("should not reach".into()));
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn or_retry_closure_can_fail() {
    let r: SzResult<String> = Err(SzError::new("transient").with_code(1008));
    let result = r.or_retry(|_| Err(SzError::new("retry also failed").with_code(999)));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), SzErrorKind::License);
}

#[test]
fn filter_retryable_converts_retryable_to_none() {
    let r: SzResult<String> = Err(SzError::new("transient").with_code(1006));
    let filtered = r.filter_retryable();
    assert_eq!(filtered.unwrap(), None);
}

#[test]
fn filter_retryable_propagates_non_retryable() {
    let r: SzResult<String> = Err(SzError::new("bad").with_code(2));
    let filtered = r.filter_retryable();
    assert!(filtered.is_err());
}

#[test]
fn filter_retryable_passes_through_ok() {
    let r: SzResult<String> = Ok("data".into());
    let filtered = r.filter_retryable();
    assert_eq!(filtered.unwrap(), Some("data".into()));
}

#[test]
fn is_retryable_err_true() {
    let r: SzResult<()> = Err(SzError::new("transient").with_code(1008));
    assert!(r.is_retryable_err());
}

#[test]
fn is_retryable_err_false_for_non_retryable() {
    let r: SzResult<()> = Err(SzError::new("bad").with_code(2));
    assert!(!r.is_retryable_err());
}

#[test]
fn is_retryable_err_false_for_ok() {
    let r: SzResult<()> = Ok(());
    assert!(!r.is_retryable_err());
}

#[test]
fn is_unrecoverable_err_true() {
    let r: SzResult<()> = Err(SzError::new("fatal").with_code(999));
    assert!(r.is_unrecoverable_err());
}

#[test]
fn is_unrecoverable_err_false_for_ok() {
    let r: SzResult<()> = Ok(());
    assert!(!r.is_unrecoverable_err());
}

#[test]
fn is_bad_input_err_true() {
    let r: SzResult<()> = Err(SzError::new("nope").with_code(33));
    assert!(r.is_bad_input_err());
}

#[test]
fn is_bad_input_err_false_for_ok() {
    let r: SzResult<()> = Ok(());
    assert!(!r.is_bad_input_err());
}

// ---------------------------------------------------------------------------
// Named constructors
// ---------------------------------------------------------------------------

#[test]
fn named_bad_input() {
    let err = SzError::bad_input("invalid").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::BadInput);
    assert_eq!(err.message(), "invalid");
    assert_eq!(err.code(), Some(999));
}

#[test]
fn named_configuration() {
    let err = SzError::configuration("bad config").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Configuration);
    assert_eq!(err.message(), "bad config");
}

#[test]
fn named_database() {
    let err = SzError::database("schema error").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Database);
    assert_eq!(err.message(), "schema error");
}

#[test]
fn named_database_connection_lost() {
    let err = SzError::database_connection_lost("gone").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::DatabaseConnectionLost);
    assert!(err.is_retryable());
}

#[test]
fn named_database_transient() {
    let err = SzError::database_transient("deadlock").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::DatabaseTransient);
    assert!(err.is_retryable());
}

#[test]
fn named_general() {
    let err = SzError::general("something").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::General);
}

#[test]
fn named_license() {
    let err = SzError::license("expired").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::License);
    assert!(err.is_unrecoverable());
}

#[test]
fn named_not_found() {
    let err = SzError::not_found("entity 42").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::NotFound);
    assert!(err.is_bad_input());
}

#[test]
fn named_not_initialized() {
    let err = SzError::not_initialized("call init first").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::NotInitialized);
    assert!(err.is_unrecoverable());
}

#[test]
fn named_replace_conflict() {
    let err = SzError::replace_conflict("conflict").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::ReplaceConflict);
}

#[test]
fn named_retryable() {
    let err = SzError::retryable("try again").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Retryable);
    assert!(err.is_retryable());
}

#[test]
fn named_retry_timeout_exceeded() {
    let err = SzError::retry_timeout_exceeded("timed out").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::RetryTimeoutExceeded);
    assert!(err.is_retryable());
}

#[test]
fn named_sdk() {
    let err = SzError::sdk("sdk issue").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::Sdk);
}

#[test]
fn named_unhandled() {
    let err = SzError::unhandled("unexpected").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Unhandled);
    assert!(err.is_unrecoverable());
}

#[test]
fn named_unknown_data_source() {
    let err = SzError::unknown_data_source("FAKE").with_code(999);
    assert_eq!(err.kind(), SzErrorKind::UnknownDataSource);
    assert!(err.is_bad_input());
}

#[test]
fn named_unrecoverable() {
    let err = SzError::unrecoverable("fatal").with_code(2);
    assert_eq!(err.kind(), SzErrorKind::Unrecoverable);
    assert!(err.is_unrecoverable());
}

#[test]
fn named_constructor_chains_with_builder() {
    use std::error::Error;
    let err = SzError::database_transient("deadlock")
        .with_code(1008)
        .with_component(SzComponent::Engine)
        .with_source(std::io::Error::other("underlying"));
    assert_eq!(err.kind(), SzErrorKind::DatabaseTransient);
    assert_eq!(err.code(), Some(1008));
    assert_eq!(err.component(), Some(SzComponent::Engine));
    assert!(err.source().is_some());
}
