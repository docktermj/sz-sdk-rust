use super::{SzComponent, SzError, SzErrorInspect, SzErrorKind, SzResult, SzResultExt};
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
fn from_code_unknown_code_defaults_to_sz_error() {
    let err = SzError::new("unknown").with_code(999999);
    assert_eq!(err.kind(), SzErrorKind::SzError);
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
fn with_code_i64_overflow_defaults_to_sz_error() {
    let err = SzError::new("test").with_code(i64::MAX);
    assert_eq!(err.kind(), SzErrorKind::SzError);
    assert_eq!(err.code(), Some(i64::MAX));
}

#[test]
fn with_code_negative_defaults_to_sz_error() {
    let err = SzError::new("test").with_code(-1);
    assert_eq!(err.kind(), SzErrorKind::SzError);
    assert_eq!(err.code(), Some(-1));
}

#[test]
fn with_code_zero_defaults_to_general() {
    let err = SzError::new("test").with_code(0);
    assert_eq!(err.kind(), SzErrorKind::General);
    assert_eq!(err.code(), Some(0));
}

#[test]
fn with_code_unknown_defaults_to_sz_error() {
    let err = SzError::new("test").with_code(999999);
    assert_eq!(err.code(), Some(999999));
    assert_eq!(err.kind(), SzErrorKind::SzError);
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
        .with_code(2) // BadInput
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
    assert_eq!(SzError::new("test").with_code(999).severity(), "critical"); // License
    assert_eq!(SzError::new("test").with_code(1000).severity(), "high"); // Database
    assert_eq!(SzError::new("test").with_code(1006).severity(), "medium"); // DatabaseConnectionLost
    assert_eq!(SzError::new("test").with_code(2).severity(), "low"); // BadInput
}

// ---------------------------------------------------------------------------
// Free function: severity
// ---------------------------------------------------------------------------

#[test]
fn inspect_sz_severity_returns_some() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(999));
    assert_eq!(err.sz_severity(), Some("critical"));
}

#[test]
fn inspect_sz_severity_returns_none_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert_eq!(err.sz_severity(), None);
}

// ---------------------------------------------------------------------------
// SzErrorKind Display
// ---------------------------------------------------------------------------

#[test]
fn kind_display() {
    assert_eq!(format!("{}", SzErrorKind::BadInput), "bad input");
    assert_eq!(format!("{}", SzErrorKind::Database), "database error");
    assert_eq!(format!("{}", SzErrorKind::Sdk), "SDK error");
    assert_eq!(format!("{}", SzErrorKind::SzError), "Senzing error");
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
    let err = SzError::new("test")
        .with_code(2)
        .with_component(SzComponent::Engine);
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
    let err = SzError::new("test")
        .with_code(2)
        .with_component(SzComponent::Diagnostic);
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
fn inspect_sz_component_returns_some() {
    let err: Box<dyn std::error::Error> = Box::new(
        SzError::new("test")
            .with_code(2)
            .with_component(SzComponent::Engine),
    );
    assert_eq!(err.sz_component(), Some(SzComponent::Engine));
}

#[test]
fn inspect_sz_component_returns_none_for_no_component() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::new("test").with_code(2));
    assert_eq!(err.sz_component(), None);
}

#[test]
fn inspect_sz_component_returns_none_for_non_sz() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert_eq!(err.sz_component(), None);
}

// ---------------------------------------------------------------------------
// component_name()
// ---------------------------------------------------------------------------

#[test]
fn component_name_returns_name_when_set() {
    let err = SzError::new("test")
        .with_code(2)
        .with_component(SzComponent::Engine);
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
    let err = SzError::new("test")
        .with_code(2)
        .with_component(SzComponent::Engine);
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
fn clone_preserves_source() {
    let cause = std::io::Error::other("disk full");
    let err = SzError::new("db failed").with_code(1000).with_source(cause);
    let cloned = err.clone();
    let source =
        std::error::Error::source(&cloned).expect("source should be preserved after clone");
    assert!(source.to_string().contains("disk full"));
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

// ---------------------------------------------------------------------------
// Hierarchy-aware `is` — SzErrorKind and SzError
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
fn kind_is_database_matches_children() {
    assert!(SzErrorKind::Database.is(SzErrorKind::Database));
    assert!(SzErrorKind::DatabaseConnectionLost.is(SzErrorKind::Database));
    assert!(SzErrorKind::DatabaseTransient.is(SzErrorKind::Database));
    assert!(!SzErrorKind::Retryable.is(SzErrorKind::Database));
    assert!(!SzErrorKind::BadInput.is(SzErrorKind::Database));
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

// ---------------------------------------------------------------------------
// Display / std::error::Error trait
// ---------------------------------------------------------------------------

#[test]
fn display_without_code() {
    let err = SzError::new("something failed");
    let display = format!("{err}");
    assert_eq!(display, "Senzing error: something failed");
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
            _ if err.is_sz(SzErrorKind::DatabaseConnectionLost) => {
                println!("Database connection lost — reconnecting: {err}");
            }
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
fn example_error_handling_with_match_and_multiple_err_match_arms_as_error() {
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
fn example_error_handling_with_match_and_multiple_err_match_arms_as_error_2() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) if err.is_sz(SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(err) if err.is_sz_bad_input() => println!("Bad input: {err}"),
        Err(err) if err.is_sz_retryable() => println!("Retryable: {err}"),
        Err(err) if err.is_sz_unrecoverable() => println!("Unrecoverable: {err}"),
        Err(err) if err.is_sz_general() => println!("General: {err}"),
        Err(err) if err.is_sz_error() => println!("General: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn example_error_handling_with_match_and_if_else_as_error() {
    match example_customer_function() {
        Ok(response) => println!("{}", response),
        Err(err) => {
            if err.is_sz_error() {
                println!(
                    "SzError (bad_input: {}, retryable: {}, unrecoverable: {}): {err}",
                    err.is_sz_bad_input(),
                    err.is_sz_retryable(),
                    err.is_sz_unrecoverable(),
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
            if let Some(szerr) = err.sz_error() {
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
            if let Some(szerr) = err.sz_error() {
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
fn propagation_with_unwrap_err_and_sz_error() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // Differentiate: use SzErrorInspect to find the SzError in the chain.
    if let Some(sz) = err.sz_error() {
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

    // The SzErrorInspect trait methods also work on Box<dyn Error>.
    assert!(err.is_sz_error());
    assert!(err.is_sz_bad_input());
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

    // The SzErrorInspect trait methods also work on Box<dyn Error>.
    assert!(err.is_sz_error());
    assert!(err.is_sz_bad_input());
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
fn propagation_using_inspect_trait() {
    let result = example_customer_function();
    let err = result.unwrap_err();

    // SzErrorInspect trait methods handle downcasting internally and
    // walk the source chain.
    assert!(err.is_sz_error());
    assert!(err.is_sz_bad_input());
    assert!(!err.is_sz_retryable());
    assert!(!err.is_sz_unrecoverable());

    // Trait methods also safely return false for non-Senzing errors.
    let io_err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("oops"));
    assert!(!io_err.is_sz_error());
    assert!(!io_err.is_sz_bad_input());
}

#[test]
fn propagation_using_box_downcast() {
    // ── Approach: Box::downcast() ──────────────────────────────────────
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
            _ if err.is_sz_bad_input() => {
                println!("Bad input: {err}");
            }
            _ if err.is_sz_retryable() => {
                println!("Retryable: {err}");
            }
            _ if err.is_sz_unrecoverable() => {
                println!("Unrecoverable: {err}");
            }
            _ if err.is_sz_general() => {
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
        _ if err.is_sz(SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        _ if err.is_sz_bad_input() => println!("Bad input: {err}"),
        _ if err.is_sz_retryable() => println!("Retryable: {err}"),
        _ if err.is_sz_unrecoverable() => println!("Unrecoverable: {err}"),
        _ if err.is_sz_general() => println!("General: {err}"),
        _ if err.is_sz_error() => println!("SzError: {err}"),
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
        Err(ref err) if err.is_sz(SzErrorKind::DatabaseConnectionLost) => {
            println!("Database connection lost — reconnecting: {err}");
        }
        Err(ref err) if err.is_sz_bad_input() => println!("Bad input: {err}"),
        Err(ref err) if err.is_sz_retryable() => println!("Retryable: {err}"),
        Err(ref err) if err.is_sz_unrecoverable() => println!("Unrecoverable: {err}"),
        Err(ref err) if err.is_sz_general() => println!("General: {err}"),
        Err(ref err) if err.is_sz_error() => println!("SzError: {err}"),
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
    assert!(r.is_ok());
    assert_eq!(r.ok(), Some(42));
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

#[test]
fn is_general_err_true() {
    let r: SzResult<()> = Err(SzError::configuration("bad config"));
    assert!(r.is_general_err());
}

#[test]
fn is_general_err_false_for_ok() {
    let r: SzResult<()> = Ok(());
    assert!(!r.is_general_err());
}

#[test]
fn is_database_err_true() {
    let r: SzResult<()> = Err(SzError::database_transient("deadlock"));
    assert!(r.is_database_err());
}

#[test]
fn is_database_err_false_for_ok() {
    let r: SzResult<()> = Ok(());
    assert!(!r.is_database_err());
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

// ---------------------------------------------------------------------------
// Blanket SzErrorInspect impl: concrete types without dyn coercion
// ---------------------------------------------------------------------------

#[test]
fn blanket_impl_sz_error_directly() {
    let err = SzError::database_transient("deadlock");
    assert!(err.is_sz_retryable());
    assert!(err.sz_error().is_some());
}

#[test]
fn blanket_impl_io_error() {
    let err = std::io::Error::other("not senzing");
    assert!(!err.is_sz_retryable());
    assert!(err.sz_error().is_none());
}

#[test]
fn blanket_impl_custom_wrapper() {
    #[derive(Debug)]
    enum AppError {
        Senzing(SzError),
    }
    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::Senzing(e) => write!(f, "app: {e}"),
            }
        }
    }
    impl std::error::Error for AppError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AppError::Senzing(e) => Some(e),
            }
        }
    }

    let app_err = AppError::Senzing(SzError::database_transient("deadlock"));
    assert!(app_err.is_sz_retryable());
    assert!(app_err.sz_error().is_some());
}

// ---------------------------------------------------------------------------
// is_database: cross-cutting predicate spanning retryable + unrecoverable
// ---------------------------------------------------------------------------

#[test]
fn is_database_for_database() {
    let err = SzError::database("schema error");
    assert!(err.is_database());
    assert!(err.is_unrecoverable());
}

#[test]
fn is_database_for_database_connection_lost() {
    let err = SzError::database_connection_lost("gone");
    assert!(err.is_database());
    assert!(err.is_retryable());
}

#[test]
fn is_database_for_database_transient() {
    let err = SzError::database_transient("deadlock");
    assert!(err.is_database());
    assert!(err.is_retryable());
}

#[test]
fn is_database_false_for_non_database() {
    assert!(!SzError::license("expired").is_database());
    assert!(!SzError::not_found("missing").is_database());
    assert!(!SzError::configuration("bad config").is_database());
}

#[test]
fn kind_is_database() {
    assert!(SzErrorKind::Database.is_database());
    assert!(SzErrorKind::DatabaseConnectionLost.is_database());
    assert!(SzErrorKind::DatabaseTransient.is_database());
    assert!(!SzErrorKind::License.is_database());
    assert!(!SzErrorKind::Retryable.is_database());
}

// ---------------------------------------------------------------------------
// category: string slug for structured logging
// ---------------------------------------------------------------------------

#[test]
fn category_all_kinds() {
    assert_eq!(SzErrorKind::BadInput.category(), "bad_input");
    assert_eq!(SzErrorKind::Configuration.category(), "configuration");
    assert_eq!(SzErrorKind::Database.category(), "database");
    assert_eq!(
        SzErrorKind::DatabaseConnectionLost.category(),
        "database_connection_lost"
    );
    assert_eq!(
        SzErrorKind::DatabaseTransient.category(),
        "database_transient"
    );
    assert_eq!(SzErrorKind::General.category(), "general");
    assert_eq!(SzErrorKind::License.category(), "license");
    assert_eq!(SzErrorKind::NotFound.category(), "not_found");
    assert_eq!(SzErrorKind::NotInitialized.category(), "not_initialized");
    assert_eq!(SzErrorKind::ReplaceConflict.category(), "replace_conflict");
    assert_eq!(SzErrorKind::Retryable.category(), "retryable");
    assert_eq!(
        SzErrorKind::RetryTimeoutExceeded.category(),
        "retry_timeout_exceeded"
    );
    assert_eq!(SzErrorKind::Sdk.category(), "sdk");
    assert_eq!(SzErrorKind::SzError.category(), "sz_error");
    assert_eq!(SzErrorKind::Unhandled.category(), "unhandled");
    assert_eq!(
        SzErrorKind::UnknownDataSource.category(),
        "unknown_data_source"
    );
    assert_eq!(SzErrorKind::Unrecoverable.category(), "unrecoverable");
}

#[test]
fn category_on_sz_error() {
    assert_eq!(
        SzError::database_transient("x").category(),
        "database_transient"
    );
    assert_eq!(SzError::license("x").category(), "license");
    assert_eq!(SzError::not_found("x").category(), "not_found");
    assert_eq!(SzError::bad_input("x").category(), "bad_input");
    assert_eq!(SzError::configuration("x").category(), "configuration");
}

// ---------------------------------------------------------------------------
// hierarchy: leaf-first type chain for every SzErrorKind
// ---------------------------------------------------------------------------

#[test]
fn hierarchy_bad_input() {
    assert_eq!(SzErrorKind::BadInput.hierarchy(), &[SzErrorKind::BadInput]);
}

#[test]
fn hierarchy_not_found() {
    assert_eq!(
        SzErrorKind::NotFound.hierarchy(),
        &[SzErrorKind::NotFound, SzErrorKind::BadInput]
    );
}

#[test]
fn hierarchy_unknown_data_source() {
    assert_eq!(
        SzErrorKind::UnknownDataSource.hierarchy(),
        &[SzErrorKind::UnknownDataSource, SzErrorKind::BadInput]
    );
}

#[test]
fn hierarchy_general() {
    assert_eq!(SzErrorKind::General.hierarchy(), &[SzErrorKind::General]);
}

#[test]
fn hierarchy_configuration() {
    assert_eq!(
        SzErrorKind::Configuration.hierarchy(),
        &[SzErrorKind::Configuration, SzErrorKind::General]
    );
}

#[test]
fn hierarchy_replace_conflict() {
    assert_eq!(
        SzErrorKind::ReplaceConflict.hierarchy(),
        &[SzErrorKind::ReplaceConflict, SzErrorKind::General]
    );
}

#[test]
fn hierarchy_sdk() {
    assert_eq!(
        SzErrorKind::Sdk.hierarchy(),
        &[SzErrorKind::Sdk, SzErrorKind::General]
    );
}

#[test]
fn hierarchy_retryable() {
    assert_eq!(
        SzErrorKind::Retryable.hierarchy(),
        &[SzErrorKind::Retryable]
    );
}

#[test]
fn hierarchy_database_connection_lost() {
    assert_eq!(
        SzErrorKind::DatabaseConnectionLost.hierarchy(),
        &[SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable]
    );
}

#[test]
fn hierarchy_database_transient() {
    assert_eq!(
        SzErrorKind::DatabaseTransient.hierarchy(),
        &[SzErrorKind::DatabaseTransient, SzErrorKind::Retryable]
    );
}

#[test]
fn hierarchy_retry_timeout_exceeded() {
    assert_eq!(
        SzErrorKind::RetryTimeoutExceeded.hierarchy(),
        &[SzErrorKind::RetryTimeoutExceeded, SzErrorKind::Retryable]
    );
}

#[test]
fn hierarchy_unrecoverable() {
    assert_eq!(
        SzErrorKind::Unrecoverable.hierarchy(),
        &[SzErrorKind::Unrecoverable]
    );
}

#[test]
fn hierarchy_database() {
    assert_eq!(
        SzErrorKind::Database.hierarchy(),
        &[SzErrorKind::Database, SzErrorKind::Unrecoverable]
    );
}

#[test]
fn hierarchy_license() {
    assert_eq!(
        SzErrorKind::License.hierarchy(),
        &[SzErrorKind::License, SzErrorKind::Unrecoverable]
    );
}

#[test]
fn hierarchy_not_initialized() {
    assert_eq!(
        SzErrorKind::NotInitialized.hierarchy(),
        &[SzErrorKind::NotInitialized, SzErrorKind::Unrecoverable]
    );
}

#[test]
fn hierarchy_unhandled() {
    assert_eq!(
        SzErrorKind::Unhandled.hierarchy(),
        &[SzErrorKind::Unhandled, SzErrorKind::Unrecoverable]
    );
}

#[test]
fn hierarchy_sz_error() {
    assert_eq!(SzErrorKind::SzError.hierarchy(), &[SzErrorKind::SzError]);
}

#[test]
fn hierarchy_on_sz_error_delegates() {
    let err = SzError::database_connection_lost("gone");
    assert_eq!(
        err.hierarchy(),
        &[SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable]
    );
}

// ---------------------------------------------------------------------------
// Leaf-level predicates on SzErrorKind
// ---------------------------------------------------------------------------

#[test]
fn kind_is_configuration() {
    assert!(SzErrorKind::Configuration.is_configuration());
    assert!(!SzErrorKind::General.is_configuration());
}

#[test]
fn kind_is_license() {
    assert!(SzErrorKind::License.is_license());
    assert!(!SzErrorKind::Unrecoverable.is_license());
}

#[test]
fn kind_is_not_found() {
    assert!(SzErrorKind::NotFound.is_not_found());
    assert!(!SzErrorKind::BadInput.is_not_found());
}

#[test]
fn kind_is_not_initialized() {
    assert!(SzErrorKind::NotInitialized.is_not_initialized());
    assert!(!SzErrorKind::Unrecoverable.is_not_initialized());
}

#[test]
fn kind_is_replace_conflict() {
    assert!(SzErrorKind::ReplaceConflict.is_replace_conflict());
    assert!(!SzErrorKind::General.is_replace_conflict());
}

#[test]
fn kind_is_sdk() {
    assert!(SzErrorKind::Sdk.is_sdk());
    assert!(!SzErrorKind::General.is_sdk());
}

#[test]
fn kind_is_unhandled() {
    assert!(SzErrorKind::Unhandled.is_unhandled());
    assert!(!SzErrorKind::Unrecoverable.is_unhandled());
}

#[test]
fn kind_is_unknown_data_source() {
    assert!(SzErrorKind::UnknownDataSource.is_unknown_data_source());
    assert!(!SzErrorKind::BadInput.is_unknown_data_source());
}

// ---------------------------------------------------------------------------
// Leaf-level predicates on SzError
// ---------------------------------------------------------------------------

#[test]
fn sz_error_is_configuration() {
    assert!(SzError::configuration("bad").is_configuration());
    assert!(!SzError::general("other").is_configuration());
}

#[test]
fn sz_error_is_license() {
    assert!(SzError::license("expired").is_license());
    assert!(!SzError::unrecoverable("other").is_license());
}

#[test]
fn sz_error_is_not_found() {
    assert!(SzError::not_found("missing").is_not_found());
    assert!(!SzError::bad_input("other").is_not_found());
}

#[test]
fn sz_error_is_not_initialized() {
    assert!(SzError::not_initialized("init").is_not_initialized());
    assert!(!SzError::unrecoverable("other").is_not_initialized());
}

#[test]
fn sz_error_is_replace_conflict() {
    assert!(SzError::replace_conflict("conflict").is_replace_conflict());
    assert!(!SzError::general("other").is_replace_conflict());
}

#[test]
fn sz_error_is_sdk() {
    assert!(SzError::sdk("sdk").is_sdk());
    assert!(!SzError::general("other").is_sdk());
}

#[test]
fn sz_error_is_unhandled() {
    assert!(SzError::unhandled("unexpected").is_unhandled());
    assert!(!SzError::unrecoverable("other").is_unhandled());
}

#[test]
fn sz_error_is_unknown_data_source() {
    assert!(SzError::unknown_data_source("FAKE").is_unknown_data_source());
    assert!(!SzError::bad_input("other").is_unknown_data_source());
}

// ---------------------------------------------------------------------------
// is_sz_database on SzErrorInspect
// ---------------------------------------------------------------------------

#[test]
fn inspect_is_sz_database_true() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::database("schema"));
    assert!(err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_connection_lost() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::database_connection_lost("gone"));
    assert!(err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_transient() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::database_transient("deadlock"));
    assert!(err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_false() {
    let err: Box<dyn std::error::Error> = Box::new(SzError::license("expired"));
    assert!(!err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_non_senzing() {
    let err: Box<dyn std::error::Error> = Box::new(std::io::Error::other("not senzing"));
    assert!(!err.is_sz_database());
}

// ---------------------------------------------------------------------------
// hierarchy: structural invariants
// ---------------------------------------------------------------------------

#[test]
fn hierarchy_first_element_is_self() {
    let all_kinds = [
        SzErrorKind::BadInput,
        SzErrorKind::Configuration,
        SzErrorKind::Database,
        SzErrorKind::DatabaseConnectionLost,
        SzErrorKind::DatabaseTransient,
        SzErrorKind::General,
        SzErrorKind::License,
        SzErrorKind::NotFound,
        SzErrorKind::NotInitialized,
        SzErrorKind::ReplaceConflict,
        SzErrorKind::Retryable,
        SzErrorKind::RetryTimeoutExceeded,
        SzErrorKind::Sdk,
        SzErrorKind::SzError,
        SzErrorKind::Unhandled,
        SzErrorKind::UnknownDataSource,
        SzErrorKind::Unrecoverable,
    ];
    for kind in all_kinds {
        let h = kind.hierarchy();
        assert!(!h.is_empty(), "{kind:?} hierarchy must not be empty");
        assert_eq!(h[0], kind, "{kind:?} hierarchy first element must be self");
    }
}

#[test]
fn hierarchy_parents_have_length_one() {
    let parents = [
        SzErrorKind::BadInput,
        SzErrorKind::General,
        SzErrorKind::Retryable,
        SzErrorKind::Unrecoverable,
        SzErrorKind::SzError,
    ];
    for kind in parents {
        assert_eq!(
            kind.hierarchy().len(),
            1,
            "{kind:?} is a parent/root and should have hierarchy length 1"
        );
    }
}

#[test]
fn hierarchy_leaves_have_length_two() {
    let leaves = [
        SzErrorKind::NotFound,
        SzErrorKind::UnknownDataSource,
        SzErrorKind::Configuration,
        SzErrorKind::ReplaceConflict,
        SzErrorKind::Sdk,
        SzErrorKind::DatabaseConnectionLost,
        SzErrorKind::DatabaseTransient,
        SzErrorKind::RetryTimeoutExceeded,
        SzErrorKind::Database,
        SzErrorKind::License,
        SzErrorKind::NotInitialized,
        SzErrorKind::Unhandled,
    ];
    for kind in leaves {
        assert_eq!(
            kind.hierarchy().len(),
            2,
            "{kind:?} is a leaf and should have hierarchy length 2"
        );
    }
}

#[test]
fn hierarchy_last_element_matches_parent_predicate() {
    // For each leaf, the last element of the hierarchy should be the parent
    // category, and is() should return true for that parent.
    let leaves_and_parents = [
        (SzErrorKind::NotFound, SzErrorKind::BadInput),
        (SzErrorKind::UnknownDataSource, SzErrorKind::BadInput),
        (SzErrorKind::Configuration, SzErrorKind::General),
        (SzErrorKind::ReplaceConflict, SzErrorKind::General),
        (SzErrorKind::Sdk, SzErrorKind::General),
        (SzErrorKind::DatabaseConnectionLost, SzErrorKind::Retryable),
        (SzErrorKind::DatabaseTransient, SzErrorKind::Retryable),
        (SzErrorKind::RetryTimeoutExceeded, SzErrorKind::Retryable),
        (SzErrorKind::Database, SzErrorKind::Unrecoverable),
        (SzErrorKind::License, SzErrorKind::Unrecoverable),
        (SzErrorKind::NotInitialized, SzErrorKind::Unrecoverable),
        (SzErrorKind::Unhandled, SzErrorKind::Unrecoverable),
    ];
    for (leaf, parent) in leaves_and_parents {
        let h = leaf.hierarchy();
        assert_eq!(
            *h.last().unwrap(),
            parent,
            "{leaf:?} hierarchy last element should be {parent:?}"
        );
        assert!(leaf.is(parent), "{leaf:?}.is({parent:?}) should be true");
    }
}

#[test]
fn hierarchy_is_consistent_with_is_method() {
    // Every element in hierarchy() should cause is() to return true.
    let all_kinds = [
        SzErrorKind::BadInput,
        SzErrorKind::Configuration,
        SzErrorKind::Database,
        SzErrorKind::DatabaseConnectionLost,
        SzErrorKind::DatabaseTransient,
        SzErrorKind::General,
        SzErrorKind::License,
        SzErrorKind::NotFound,
        SzErrorKind::NotInitialized,
        SzErrorKind::ReplaceConflict,
        SzErrorKind::Retryable,
        SzErrorKind::RetryTimeoutExceeded,
        SzErrorKind::Sdk,
        SzErrorKind::SzError,
        SzErrorKind::Unhandled,
        SzErrorKind::UnknownDataSource,
        SzErrorKind::Unrecoverable,
    ];
    for kind in all_kinds {
        for ancestor in kind.hierarchy() {
            assert!(
                kind.is(*ancestor),
                "{kind:?}.is({ancestor:?}) should be true since {ancestor:?} is in its hierarchy"
            );
        }
    }
}

#[test]
fn hierarchy_is_zero_allocation() {
    // Calling hierarchy() twice returns the same pointer, proving static allocation.
    let h1 = SzErrorKind::DatabaseTransient.hierarchy();
    let h2 = SzErrorKind::DatabaseTransient.hierarchy();
    assert!(std::ptr::eq(h1, h2));
}

// ---------------------------------------------------------------------------
// hierarchy: delegation through SzError for every family
// ---------------------------------------------------------------------------

#[test]
fn hierarchy_delegates_for_all_families() {
    // One representative from each family.
    let cases: Vec<(SzError, &[SzErrorKind])> = vec![
        (
            SzError::not_found("x"),
            &[SzErrorKind::NotFound, SzErrorKind::BadInput],
        ),
        (
            SzError::configuration("x"),
            &[SzErrorKind::Configuration, SzErrorKind::General],
        ),
        (
            SzError::database_transient("x"),
            &[SzErrorKind::DatabaseTransient, SzErrorKind::Retryable],
        ),
        (
            SzError::license("x"),
            &[SzErrorKind::License, SzErrorKind::Unrecoverable],
        ),
        (SzError::new("x"), &[SzErrorKind::SzError]),
    ];
    for (err, expected) in cases {
        assert_eq!(
            err.hierarchy(),
            expected,
            "hierarchy mismatch for {:?}",
            err.kind()
        );
    }
}

#[test]
fn hierarchy_delegates_for_parent_kinds() {
    assert_eq!(
        SzError::bad_input("x").hierarchy(),
        &[SzErrorKind::BadInput]
    );
    assert_eq!(SzError::general("x").hierarchy(), &[SzErrorKind::General]);
    assert_eq!(
        SzError::retryable("x").hierarchy(),
        &[SzErrorKind::Retryable]
    );
    assert_eq!(
        SzError::unrecoverable("x").hierarchy(),
        &[SzErrorKind::Unrecoverable]
    );
}

// ---------------------------------------------------------------------------
// Leaf predicates: exhaustive negative checks on SzErrorKind
// ---------------------------------------------------------------------------

#[test]
fn kind_leaf_predicates_false_for_all_other_kinds() {
    // Each predicate should return true for exactly one kind.
    type PredKindPair = (fn(SzErrorKind) -> bool, SzErrorKind);
    let predicate_kind_pairs: Vec<PredKindPair> = vec![
        (SzErrorKind::is_configuration, SzErrorKind::Configuration),
        (SzErrorKind::is_license, SzErrorKind::License),
        (SzErrorKind::is_not_found, SzErrorKind::NotFound),
        (SzErrorKind::is_not_initialized, SzErrorKind::NotInitialized),
        (
            SzErrorKind::is_replace_conflict,
            SzErrorKind::ReplaceConflict,
        ),
        (SzErrorKind::is_sdk, SzErrorKind::Sdk),
        (SzErrorKind::is_unhandled, SzErrorKind::Unhandled),
        (
            SzErrorKind::is_unknown_data_source,
            SzErrorKind::UnknownDataSource,
        ),
    ];
    let all_kinds = [
        SzErrorKind::BadInput,
        SzErrorKind::Configuration,
        SzErrorKind::Database,
        SzErrorKind::DatabaseConnectionLost,
        SzErrorKind::DatabaseTransient,
        SzErrorKind::General,
        SzErrorKind::License,
        SzErrorKind::NotFound,
        SzErrorKind::NotInitialized,
        SzErrorKind::ReplaceConflict,
        SzErrorKind::Retryable,
        SzErrorKind::RetryTimeoutExceeded,
        SzErrorKind::Sdk,
        SzErrorKind::SzError,
        SzErrorKind::Unhandled,
        SzErrorKind::UnknownDataSource,
        SzErrorKind::Unrecoverable,
    ];
    for (predicate, expected_kind) in &predicate_kind_pairs {
        for kind in &all_kinds {
            if *kind == *expected_kind {
                assert!(predicate(*kind), "{kind:?} should match its own predicate");
            } else {
                assert!(
                    !predicate(*kind),
                    "{kind:?} should NOT match the predicate for {expected_kind:?}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Leaf predicates: siblings don't cross-match
// ---------------------------------------------------------------------------

#[test]
fn leaf_predicates_sibling_isolation_bad_input_family() {
    // NotFound and UnknownDataSource are siblings under BadInput.
    assert!(!SzErrorKind::NotFound.is_unknown_data_source());
    assert!(!SzErrorKind::UnknownDataSource.is_not_found());
}

#[test]
fn leaf_predicates_sibling_isolation_general_family() {
    assert!(!SzErrorKind::Configuration.is_replace_conflict());
    assert!(!SzErrorKind::Configuration.is_sdk());
    assert!(!SzErrorKind::ReplaceConflict.is_configuration());
    assert!(!SzErrorKind::ReplaceConflict.is_sdk());
    assert!(!SzErrorKind::Sdk.is_configuration());
    assert!(!SzErrorKind::Sdk.is_replace_conflict());
}

#[test]
fn leaf_predicates_sibling_isolation_unrecoverable_family() {
    assert!(!SzErrorKind::Database.is_license());
    assert!(!SzErrorKind::Database.is_not_initialized());
    assert!(!SzErrorKind::Database.is_unhandled());
    assert!(!SzErrorKind::License.is_not_initialized());
    assert!(!SzErrorKind::License.is_unhandled());
    assert!(!SzErrorKind::NotInitialized.is_license());
    assert!(!SzErrorKind::Unhandled.is_license());
}

// ---------------------------------------------------------------------------
// Leaf predicates: parent kinds return false for leaf predicates
// ---------------------------------------------------------------------------

#[test]
fn leaf_predicates_false_for_parent_kinds() {
    // Parent kinds should not match any leaf predicate.
    let parents = [
        SzErrorKind::BadInput,
        SzErrorKind::General,
        SzErrorKind::Retryable,
        SzErrorKind::Unrecoverable,
        SzErrorKind::SzError,
    ];
    for parent in parents {
        assert!(!parent.is_configuration(), "{parent:?}");
        assert!(!parent.is_license(), "{parent:?}");
        assert!(!parent.is_not_found(), "{parent:?}");
        assert!(!parent.is_not_initialized(), "{parent:?}");
        assert!(!parent.is_replace_conflict(), "{parent:?}");
        assert!(!parent.is_sdk(), "{parent:?}");
        assert!(!parent.is_unhandled(), "{parent:?}");
        assert!(!parent.is_unknown_data_source(), "{parent:?}");
    }
}

// ---------------------------------------------------------------------------
// Leaf predicates on SzError: with_code preserves explicit kind
// ---------------------------------------------------------------------------

#[test]
fn leaf_predicate_survives_with_code() {
    // Named constructor sets kind_explicit, so with_code should not change it.
    let err = SzError::configuration("bad config").with_code(999);
    assert!(err.is_configuration());
    assert!(!err.is_license());
}

#[test]
fn leaf_predicate_not_found_survives_with_code() {
    let err = SzError::not_found("entity 42").with_code(14);
    assert!(err.is_not_found());
    assert!(err.is_bad_input());
}

#[test]
fn leaf_predicate_from_code_derived_kind() {
    // When kind is derived from code (not explicit), leaf predicates should still work.
    let err = SzError::new("test").with_code(999);
    assert!(err.is_license());
    assert!(err.is_unrecoverable());
    assert!(!err.is_configuration());
}

// ---------------------------------------------------------------------------
// Leaf predicates on SzError: combined with builder methods
// ---------------------------------------------------------------------------

#[test]
fn leaf_predicate_with_component_and_source() {
    let err = SzError::not_initialized("call init first")
        .with_code(2)
        .with_component(SzComponent::Engine)
        .with_source(std::io::Error::other("underlying"));
    assert!(err.is_not_initialized());
    assert!(err.is_unrecoverable());
    assert!(!err.is_not_found());
}

#[test]
fn leaf_predicate_after_clone() {
    let err = SzError::sdk("issue");
    let cloned = err.clone();
    assert!(cloned.is_sdk());
    assert!(cloned.is_general());
    assert!(!cloned.is_configuration());
}

// ---------------------------------------------------------------------------
// Leaf predicates on SzError: from SzErrorKind conversion
// ---------------------------------------------------------------------------

#[test]
fn leaf_predicate_from_kind_conversion() {
    let err: SzError = SzErrorKind::License.into();
    assert!(err.is_license());
    assert!(err.is_unrecoverable());
    assert!(!err.is_database());
}

#[test]
fn leaf_predicate_from_kind_configuration() {
    let err: SzError = SzErrorKind::Configuration.into();
    assert!(err.is_configuration());
    assert!(err.is_general());
}

// ---------------------------------------------------------------------------
// is_sz_database on SzErrorInspect: blanket impl (concrete types)
// ---------------------------------------------------------------------------

#[test]
fn inspect_is_sz_database_blanket_concrete() {
    let err = SzError::database("schema error");
    assert!(err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_blanket_concrete_false() {
    let err = SzError::not_found("missing");
    assert!(!err.is_sz_database());
}

// ---------------------------------------------------------------------------
// is_sz_database on SzErrorInspect: through wrapped error chain
// ---------------------------------------------------------------------------

#[test]
fn inspect_is_sz_database_through_wrapper() {
    #[derive(Debug)]
    struct Wrapper(SzError);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "wrapped: {}", self.0)
        }
    }
    impl std::error::Error for Wrapper {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.0)
        }
    }

    let wrapper = Wrapper(SzError::database_connection_lost("gone"));
    assert!(wrapper.is_sz_database());
    assert!(wrapper.is_sz_retryable());
}

#[test]
fn inspect_is_sz_database_through_wrapper_false() {
    #[derive(Debug)]
    struct Wrapper(SzError);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "wrapped: {}", self.0)
        }
    }
    impl std::error::Error for Wrapper {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.0)
        }
    }

    let wrapper = Wrapper(SzError::configuration("bad"));
    assert!(!wrapper.is_sz_database());
    assert!(wrapper.is_sz_general());
}

// ---------------------------------------------------------------------------
// is_sz_database via Send + Sync dyn Error
// ---------------------------------------------------------------------------

#[test]
fn inspect_is_sz_database_send_sync() {
    let err: Box<dyn std::error::Error + Send + Sync> =
        Box::new(SzError::database_transient("deadlock"));
    assert!(err.is_sz_database());
}

#[test]
fn inspect_is_sz_database_send_only() {
    let err: Box<dyn std::error::Error + Send> = Box::new(SzError::database("corruption"));
    assert!(err.is_sz_database());
}

// ---------------------------------------------------------------------------
// hierarchy + leaf predicates: consistency check
// ---------------------------------------------------------------------------

#[test]
fn hierarchy_leaf_matches_own_leaf_predicate() {
    // For each leaf kind, ensure its leaf predicate returns true and its
    // hierarchy[0] is the same kind that the predicate checks.
    assert!(SzErrorKind::Configuration.hierarchy()[0].is_configuration());
    assert!(SzErrorKind::License.hierarchy()[0].is_license());
    assert!(SzErrorKind::NotFound.hierarchy()[0].is_not_found());
    assert!(SzErrorKind::NotInitialized.hierarchy()[0].is_not_initialized());
    assert!(SzErrorKind::ReplaceConflict.hierarchy()[0].is_replace_conflict());
    assert!(SzErrorKind::Sdk.hierarchy()[0].is_sdk());
    assert!(SzErrorKind::Unhandled.hierarchy()[0].is_unhandled());
    assert!(SzErrorKind::UnknownDataSource.hierarchy()[0].is_unknown_data_source());
}

#[test]
fn hierarchy_parent_matches_family_predicate() {
    // For each leaf, the parent (hierarchy[1]) should match the family predicate.
    assert!(SzErrorKind::NotFound.hierarchy()[1].is_bad_input());
    assert!(SzErrorKind::UnknownDataSource.hierarchy()[1].is_bad_input());
    assert!(SzErrorKind::Configuration.hierarchy()[1].is_general());
    assert!(SzErrorKind::ReplaceConflict.hierarchy()[1].is_general());
    assert!(SzErrorKind::Sdk.hierarchy()[1].is_general());
    assert!(SzErrorKind::DatabaseConnectionLost.hierarchy()[1].is_retryable());
    assert!(SzErrorKind::DatabaseTransient.hierarchy()[1].is_retryable());
    assert!(SzErrorKind::RetryTimeoutExceeded.hierarchy()[1].is_retryable());
    assert!(SzErrorKind::Database.hierarchy()[1].is_unrecoverable());
    assert!(SzErrorKind::License.hierarchy()[1].is_unrecoverable());
    assert!(SzErrorKind::NotInitialized.hierarchy()[1].is_unrecoverable());
    assert!(SzErrorKind::Unhandled.hierarchy()[1].is_unrecoverable());
}

// ---------------------------------------------------------------------------
// SzError::find_in_chain
// ---------------------------------------------------------------------------

/// Returns a four-level error chain with an SzError at the innermost level.
///
/// Chain (outermost → innermost):
///   WrapperError → WrapperError → WrapperError → SzError
fn err_chain_with_sz_error() -> Result<String, Box<dyn std::error::Error>> {
    // Level 1 (innermost): SzError
    let sz = SzError::database_connection_lost("connection reset by peer")
        .with_code(1006)
        .with_component(SzComponent::Engine)
        .with_details("host=db.example.com port=5432");

    // Level 2: first WrapperError around SzError
    let level2 = WrapperError(Box::new(sz));

    // Level 3: second WrapperError
    let level3 = WrapperError(Box::new(level2));

    // Level 4 (outermost): third WrapperError
    Err(Box::new(WrapperError(Box::new(level3))))
}

/// Returns a four-level error chain with no SzError at any level.
///
/// Chain (outermost → innermost):
///   WrapperError → WrapperError → WrapperError → io::Error
fn err_chain_with_error() -> Result<String, Box<dyn std::error::Error>> {
    // Level 1 (innermost): a plain io::Error
    let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe broken");

    // Level 2: first WrapperError around io::Error
    let level2 = WrapperError(Box::new(io_err));

    // Level 3: second WrapperError
    let level3 = WrapperError(Box::new(level2));

    // Level 4 (outermost): third WrapperError
    Err(Box::new(WrapperError(Box::new(level3))))
}

/// Parent that calls both children using `?`.  Both error types auto-convert
/// into `Box<dyn Error>` so the caller gets a single unified return type.
fn example_customer_function_with_err_chain() -> Result<String, Box<dyn std::error::Error>> {
    let value = err_chain_with_sz_error()?;
    let _ = err_chain_with_error()?;
    Ok(value)
}

#[test]
fn propagation_with_chain_with_multiple_err_match_arms_as_error_simplified() {
    match example_customer_function_with_err_chain() {
        Ok(response) => println!("{}", response),
        Err(err) if err.is_sz_error() => println!("SzError: {err}"),
        Err(err) => println!("Non-SzError error: {err}"),
    }
}

#[test]
fn err_chain_returns_sz_error() {
    // example_customer_function_with_err_chain() calls err_chain_with_sz_error() first,
    // which fails, so the propagated error contains the SzError from that call.
    let err = example_customer_function_with_err_chain().unwrap_err();

    // SzErrorInspect finds the SzError through the chain.
    assert!(err.is_sz_error());
    assert!(err.is_sz_retryable());
    assert!(!err.is_sz_bad_input());
    assert!(!err.is_sz_unrecoverable());

    // Hierarchy-aware kind check.
    assert!(err.is_sz(SzErrorKind::DatabaseConnectionLost));
    assert!(err.is_sz(SzErrorKind::Retryable));
    assert!(err.is_sz(SzErrorKind::SzError));
    assert!(!err.is_sz(SzErrorKind::BadInput));

    // Extract the SzError and verify all fields.
    let sz = err.sz_error().expect("should find SzError in chain");
    assert_eq!(sz.kind(), SzErrorKind::DatabaseConnectionLost);
    assert_eq!(sz.code(), Some(1006));
    assert_eq!(sz.component(), Some(SzComponent::Engine));
    assert_eq!(sz.message(), "connection reset by peer");
    assert_eq!(sz.details(), Some("host=db.example.com port=5432"));
    assert_eq!(sz.severity(), "medium");
    assert_eq!(sz.category(), "database_connection_lost");
    assert!(sz.is_database());
    assert!(sz.is_retryable());
}

#[test]
fn find_in_chain_direct_hit() {
    let err = SzError::not_found("entity 42");
    let found = SzError::find_in_chain(&err).unwrap();
    assert_eq!(found.kind(), SzErrorKind::NotFound);
    assert_eq!(found.message(), "entity 42");
}

#[test]
fn find_in_chain_walks_source_chain() {
    let inner = SzError::database_transient("deadlock");
    let wrapped: Box<dyn std::error::Error> = Box::new(WrapperError(Box::new(inner)));
    let found = SzError::find_in_chain(&*wrapped).unwrap();
    assert_eq!(found.kind(), SzErrorKind::DatabaseTransient);
}

#[test]
fn find_in_chain_no_match() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
    assert!(SzError::find_in_chain(&io_err).is_none());
}

#[test]
fn find_in_chain_deeply_nested() {
    // SzError inside WrapperError inside another WrapperError
    let sz = SzError::license("expired");
    let mid = WrapperError(Box::new(sz));
    let outer = WrapperError(Box::new(mid));
    let found = SzError::find_in_chain(&outer).unwrap();
    assert_eq!(found.kind(), SzErrorKind::License);
}

#[test]
fn find_in_chain_four_levels() {
    // Level 1 (innermost): SzError
    let sz = SzError::database_connection_lost("connection reset by peer")
        .with_code(1006)
        .with_component(SzComponent::Engine)
        .with_details("host=db.example.com port=5432");

    // Level 2: first WrapperError around SzError
    let level2 = WrapperError(Box::new(sz));

    // Level 3: second WrapperError
    let level3 = WrapperError(Box::new(level2));

    // Level 4 (outermost): third WrapperError
    let level4 = WrapperError(Box::new(level3));

    // find_in_chain walks all four levels to find the SzError at the bottom.
    let found = SzError::find_in_chain(&level4).unwrap();
    assert_eq!(found.kind(), SzErrorKind::DatabaseConnectionLost);
    assert_eq!(found.code(), Some(1006));
    assert_eq!(found.component(), Some(SzComponent::Engine));
    assert_eq!(found.message(), "connection reset by peer");
    assert_eq!(found.details(), Some("host=db.example.com port=5432"));

    // Verify the hierarchy predicates still work through the chain.
    assert!(found.kind().is_database());
    assert!(found.kind().is_retryable());

    // Verify Display output includes all fields.
    let display = format!("{level4}");
    assert!(display.contains("connection reset by peer"));

    // Walk the std::error::Error source chain manually to confirm 4 levels.
    let src1 = (&level4 as &dyn std::error::Error)
        .source()
        .expect("level4 should have source");
    let src2 = src1.source().expect("level3 should have source");
    let src3 = src2.source().expect("level2 should have source");
    assert!(
        src3.source().is_none(),
        "SzError (no source set) is the leaf"
    );
}

#[test]
fn find_in_chain_four_levels_no_sz_error() {
    // Level 1 (innermost): a plain io::Error
    let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe broken");

    // Level 2: WrapperError around io::Error
    let level2 = WrapperError(Box::new(io_err));

    // Level 3: WrapperError around level 2
    let level3 = WrapperError(Box::new(level2));

    // Level 4 (outermost): WrapperError around level 3
    let level4 = WrapperError(Box::new(level3));

    // No SzError anywhere in the chain, so find_in_chain returns None.
    assert!(SzError::find_in_chain(&level4).is_none());

    // Walk the source chain manually to confirm all 4 levels exist.
    let src1 = (&level4 as &dyn std::error::Error)
        .source()
        .expect("level4 should have source");
    let src2 = src1.source().expect("level3 should have source");
    let src3 = src2.source().expect("level2 should have source");
    assert!(src3.source().is_none(), "io::Error is the leaf");
}

// ---------------------------------------------------------------------------
// wrap: convenience method for non-Senzing errors
// ---------------------------------------------------------------------------

#[test]
fn wrap_sets_kind_to_sdk() {
    let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe broken");
    let err = SzError::wrap(io_err);
    assert_eq!(err.kind(), SzErrorKind::Sdk);
}

#[test]
fn wrap_uses_display_as_message() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let err = SzError::wrap(io_err);
    assert_eq!(err.message(), "file missing");
}

#[test]
fn wrap_preserves_source() {
    let io_err = std::io::Error::other("oops");
    let err = SzError::wrap(io_err);
    let src = std::error::Error::source(&err).expect("source should be set");
    assert!(src.downcast_ref::<std::io::Error>().is_some());
}

#[test]
fn wrap_works_with_map_err() {
    let result: Result<(), std::io::Error> = Err(std::io::Error::other("boom"));
    let sz_result: Result<(), SzError> = result.map_err(SzError::wrap);
    let err = sz_result.unwrap_err();
    assert_eq!(err.kind(), SzErrorKind::Sdk);
    assert_eq!(err.message(), "boom");
}

// ---------------------------------------------------------------------------
// details: supplementary context field
// ---------------------------------------------------------------------------

#[test]
fn details_is_none_by_default() {
    let err = SzError::new("x");
    assert!(err.details().is_none());
}

#[test]
fn with_details_sets_details() {
    let err = SzError::new("msg").with_details("record_id=42");
    assert_eq!(err.details(), Some("record_id=42"));
}

#[test]
fn with_details_called_twice_overwrites() {
    let err = SzError::new("msg")
        .with_details("first")
        .with_details("second");
    assert_eq!(err.details(), Some("second"));
}

#[test]
fn clone_preserves_details() {
    let err = SzError::new("msg").with_details("extra");
    let cloned = err.clone();
    assert_eq!(cloned.details(), Some("extra"));
}

#[test]
fn display_includes_details() {
    let err = SzError::bad_input("invalid")
        .with_code(2)
        .with_details("field=name");
    let s = format!("{err}");
    assert!(s.contains("[field=name]"), "display was: {s}");
}

#[test]
fn display_without_details_unchanged() {
    let err = SzError::bad_input("invalid").with_code(2);
    let s = format!("{err}");
    assert_eq!(s, "bad input (code 2): invalid");
}

#[test]
fn details_with_named_constructor() {
    let err = SzError::bad_input("msg").with_details("extra");
    assert_eq!(err.kind(), SzErrorKind::BadInput);
    assert_eq!(err.details(), Some("extra"));
}

#[test]
fn from_kind_has_no_details() {
    let err = SzError::from(SzErrorKind::Sdk);
    assert!(err.details().is_none());
}
