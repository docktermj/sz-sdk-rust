use crate::{SzConfigManager, SzDiagnostic, SzEngine, SzError, SzProduct};

/// Trait for creating Senzing SDK component instances.
///
/// `SzAbstractFactory` provides a factory pattern for obtaining
/// instances of the various Senzing SDK components. Implementations
/// (e.g., core local binding, gRPC remote binding) provide concrete
/// factories that create their respective component implementations.
pub trait SzAbstractFactory {
    /// Closes the factory and releases any held resources.
    fn close(&mut self) -> Result<(), SzError>;

    /// Creates a new config manager instance.
    fn create_config_manager(&self) -> Result<Box<dyn SzConfigManager>, SzError>;

    /// Creates a new diagnostic instance.
    fn create_diagnostic(&self) -> Result<Box<dyn SzDiagnostic>, SzError>;

    /// Creates a new engine instance.
    fn create_engine(&self) -> Result<Box<dyn SzEngine>, SzError>;

    /// Creates a new product instance.
    fn create_product(&self) -> Result<Box<dyn SzProduct>, SzError>;

    /// Re-initializes the Senzing engine with a different configuration.
    fn reinitialize(&mut self, config_id: i64) -> Result<(), SzError>;
}
