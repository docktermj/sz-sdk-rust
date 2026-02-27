use crate::{SzConfig, SzError};

/// Trait for managing Senzing configurations in a persistent store.
///
/// `SzConfigManager` provides methods for creating, registering, and
/// retrieving Senzing configurations. It acts as a factory for
/// [`SzConfig`] instances.
pub trait SzConfigManager {
    /// Creates an `SzConfig` from a previously registered configuration ID.
    fn create_config_from_config_id(&self, config_id: i64) -> Result<Box<dyn SzConfig>, SzError>;

    /// Creates an `SzConfig` from a JSON configuration string.
    fn create_config_from_string(
        &self,
        config_definition: &str,
    ) -> Result<Box<dyn SzConfig>, SzError>;

    /// Creates an `SzConfig` from the default template configuration.
    fn create_config_from_template(&self) -> Result<Box<dyn SzConfig>, SzError>;

    /// Destroys and cleans up the config manager.
    fn destroy(&mut self) -> Result<(), SzError>;

    /// Returns a JSON document listing all registered configurations.
    fn get_config_registry(&self) -> Result<String, SzError>;

    /// Returns the configuration ID of the current default configuration.
    fn get_default_config_id(&self) -> Result<i64, SzError>;

    /// Registers a configuration with the data store and returns its new ID.
    fn register_config(
        &mut self,
        config_definition: &str,
        config_comment: &str,
    ) -> Result<i64, SzError>;

    /// Atomically replaces the default configuration ID, checking that
    /// `current_default_config_id` still matches the stored value.
    fn replace_default_config_id(
        &mut self,
        current_default_config_id: i64,
        new_default_config_id: i64,
    ) -> Result<(), SzError>;

    /// Registers the given configuration and sets it as the default in one step.
    /// Returns the new configuration ID.
    fn set_default_config(
        &mut self,
        config_definition: &str,
        config_comment: &str,
    ) -> Result<i64, SzError>;

    /// Sets the default configuration ID.
    fn set_default_config_id(&mut self, config_id: i64) -> Result<(), SzError>;
}
