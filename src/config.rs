use crate::SzError;

/// Trait for managing Senzing configuration data sources.
///
/// `SzConfig` provides methods for inspecting and modifying data sources
/// within a Senzing configuration. Instances are created via
/// [`SzConfigManager`](crate::SzConfigManager).
pub trait SzConfig {
    /// Exports the current configuration as a JSON string.
    fn export_config(&self) -> Result<String, SzError>;

    /// Returns a JSON document listing all registered data sources.
    fn get_data_source_registry(&self) -> Result<String, SzError>;

    /// Registers a new data source with the given code.
    /// Returns a JSON document describing the registered data source.
    fn register_data_source(&mut self, data_source_code: &str) -> Result<String, SzError>;

    /// Unregisters (removes) a data source by its code.
    fn unregister_data_source(&mut self, data_source_code: &str) -> Result<(), SzError>;
}
