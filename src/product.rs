use crate::SzError;

/// Trait for retrieving Senzing product information.
///
/// `SzProduct` provides methods for querying license and version
/// information about the Senzing SDK installation.
pub trait SzProduct {
    /// Destroys and cleans up the product object.
    fn destroy(&mut self) -> Result<(), SzError>;

    /// Returns a JSON document with license information.
    fn get_license(&self) -> Result<String, SzError>;

    /// Returns a JSON document with version and build information.
    fn get_version(&self) -> Result<String, SzError>;
}
