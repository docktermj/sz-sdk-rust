//! Demonstrates the Abstract Factory pattern for the Senzing SDK.
//!
//! This example shows how to write implementation-agnostic code that works
//! with any concrete factory (core or gRPC). Once the factory is created,
//! all SDK calls are identical regardless of the underlying implementation.
//!
//! ```text
//! // With core (local C library):
//! let factory = SzAbstractFactoryCore::new("demo", settings, 0, 0)?;
//! use_sdk(&factory)?;
//!
//! // With gRPC (remote server):
//! let factory = SzAbstractFactoryGrpc::new("http://localhost:8261")?;
//! use_sdk(&factory)?;
//! ```

use sz_sdk::flags::{SZ_ENTITY_DEFAULT_FLAGS, SZ_NO_FLAGS, SZ_SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS};
use sz_sdk::{SzAbstractFactory, SzError};

/// All code below this point is completely implementation-agnostic.
/// It works identically whether the factory is backed by a local C library
/// or a remote gRPC server.
fn use_sdk(factory: &dyn SzAbstractFactory) -> Result<(), SzError> {
    // --- SzProduct: get version and license info ---
    let product = factory.create_product()?;
    let version = product.get_version()?;
    println!("Senzing version: {version}");

    let license = product.get_license()?;
    println!("License info: {license}");

    // --- SzEngine: add records and query entities ---
    let mut engine = factory.create_engine()?;

    engine.add_record(
        "CUSTOMERS",
        "1001",
        r#"{"NAME_FULL": "Robert Smith", "DATE_OF_BIRTH": "11/12/1978"}"#,
        SZ_NO_FLAGS,
    )?;

    engine.add_record(
        "CUSTOMERS",
        "1002",
        r#"{"NAME_FULL": "Bob J Smith", "DATE_OF_BIRTH": "12/11/1978"}"#,
        SZ_NO_FLAGS,
    )?;

    let entity = engine.get_entity_by_record_id("CUSTOMERS", "1001", SZ_ENTITY_DEFAULT_FLAGS)?;
    println!("Entity: {entity}");

    let search_result = engine.search_by_attributes(
        r#"{"NAME_FULL": "Robert Smith"}"#,
        "",
        SZ_SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS,
    )?;
    println!("Search result: {search_result}");

    // --- SzConfigManager: manage configurations ---
    let config_mgr = factory.create_config_manager()?;
    let default_config_id = config_mgr.get_default_config_id()?;
    println!("Default config ID: {default_config_id}");

    let mut config = config_mgr.create_config_from_config_id(default_config_id)?;
    let registry = config.get_data_source_registry()?;
    println!("Data sources: {registry}");

    config.register_data_source("NEW_SOURCE")?;
    let updated_registry = config.get_data_source_registry()?;
    println!("Updated data sources: {updated_registry}");

    // --- SzDiagnostic: check repository ---
    let diagnostic = factory.create_diagnostic()?;
    let repo_info = diagnostic.get_repository_info()?;
    println!("Repository info: {repo_info}");

    // --- Cleanup ---
    engine.delete_record("CUSTOMERS", "1001", SZ_NO_FLAGS)?;
    engine.delete_record("CUSTOMERS", "1002", SZ_NO_FLAGS)?;

    Ok(())
}

fn main() {
    // In a real application, you would create either a core or gRPC factory
    // based on configuration. For example:
    //
    //   use sz_sdk_core::SzAbstractFactoryCore;
    //   let factory = SzAbstractFactoryCore::new("demo", settings, 0, 0).unwrap();
    //   use_sdk(&factory).unwrap();
    //
    //   -- OR --
    //
    //   use sz_sdk_rust_grpc::SzAbstractFactoryGrpc;
    //   let factory = SzAbstractFactoryGrpc::new("http://localhost:8261").unwrap();
    //   use_sdk(&factory).unwrap();
    //
    // The `use_sdk` function works identically with either factory.

    println!("Abstract Factory pattern demonstration.");
    println!("See source code for usage examples with core and gRPC factories.");
    println!();
    println!("Key point: the `use_sdk` function accepts `&dyn SzAbstractFactory`");
    println!("and works identically regardless of the concrete implementation.");

    // This compiles and proves the pattern works at the type level.
    // To actually run it, you would need either a Senzing installation (core)
    // or a running gRPC server.
    let _ = use_sdk;
}
