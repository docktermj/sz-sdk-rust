use crate::SzError;

/// Trait for Senzing diagnostic operations.
///
/// `SzDiagnostic` provides methods for checking data store performance,
/// retrieving repository information, and maintenance operations like purging.
pub trait SzDiagnostic {
    /// Runs a performance check on the data store for the given duration.
    /// Returns a JSON document with performance metrics.
    fn check_repository_performance(&self, seconds_to_run: i32) -> Result<String, SzError>;

    /// Destroys and cleans up the diagnostic object.
    fn destroy(&mut self) -> Result<(), SzError>;

    /// Retrieves feature data by internal feature ID.
    /// Returns a JSON document describing the feature.
    fn get_feature(&self, feature_id: i64) -> Result<String, SzError>;

    /// Returns a JSON document with information about the data store.
    fn get_repository_info(&self) -> Result<String, SzError>;

    /// Purges all data from the repository. Use with extreme caution.
    fn purge_repository(&mut self) -> Result<(), SzError>;
}
