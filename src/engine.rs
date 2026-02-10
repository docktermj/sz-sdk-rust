use crate::SzError;

/// Trait for Senzing entity resolution engine operations.
///
/// `SzEngine` is the primary interface to the Senzing entity resolution
/// engine. It provides methods for adding/deleting records, querying
/// entities, finding relationships, and performing why/how analysis.
///
/// Most methods that return entity or record data return JSON strings.
/// The `flags` parameter on many methods is a bitmask controlling what
/// data is included in the response (see [`crate::flags`]).
pub trait SzEngine {
    /// Adds a record to the Senzing repository.
    /// Returns a JSON "with info" document when the `SZ_WITH_INFO` flag is set,
    /// or an empty string otherwise.
    fn add_record(
        &mut self,
        data_source_code: &str,
        record_id: &str,
        record_definition: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Closes an export handle previously returned by
    /// [`export_json_entity_report`](SzEngine::export_json_entity_report) or
    /// [`export_csv_entity_report`](SzEngine::export_csv_entity_report).
    fn close_export_report(&mut self, export_handle: usize) -> Result<(), SzError>;

    /// Returns the number of records in the redo queue.
    fn count_redo_records(&self) -> Result<i64, SzError>;

    /// Deletes a record from the Senzing repository.
    /// Returns a JSON "with info" document when the `SZ_WITH_INFO` flag is set,
    /// or an empty string otherwise.
    fn delete_record(
        &mut self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Destroys and cleans up the engine object.
    fn destroy(&mut self) -> Result<(), SzError>;

    /// Opens a CSV export of entity data. Returns an export handle for use
    /// with [`fetch_next`](SzEngine::fetch_next).
    fn export_csv_entity_report(
        &mut self,
        csv_column_list: &str,
        flags: i64,
    ) -> Result<usize, SzError>;

    /// Opens a JSON export of entity data. Returns an export handle for use
    /// with [`fetch_next`](SzEngine::fetch_next).
    fn export_json_entity_report(&mut self, flags: i64) -> Result<usize, SzError>;

    /// Reads the next row from an export handle. Returns an empty string
    /// when no more data is available.
    fn fetch_next(&self, export_handle: usize) -> Result<String, SzError>;

    /// Finds interesting entities around the given entity.
    fn find_interesting_entities_by_entity_id(
        &self,
        entity_id: i64,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Finds interesting entities around the entity containing the given record.
    fn find_interesting_entities_by_record_id(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Finds a network of relationships among a set of entities.
    /// `entity_ids` is a JSON document listing the entities.
    fn find_network_by_entity_id(
        &self,
        entity_ids: &str,
        max_degrees: i64,
        build_out_degrees: i64,
        build_out_max_entities: i64,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Finds a network of relationships among a set of records.
    /// `record_keys` is a JSON document listing the records.
    fn find_network_by_record_id(
        &self,
        record_keys: &str,
        max_degrees: i64,
        build_out_degrees: i64,
        build_out_max_entities: i64,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Finds a relationship path between two entities.
    /// `avoid_entity_ids` and `required_data_sources` are JSON documents
    /// (pass empty string for none).
    fn find_path_by_entity_id(
        &self,
        start_entity_id: i64,
        end_entity_id: i64,
        max_degrees: i64,
        avoid_entity_ids: &str,
        required_data_sources: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Finds a relationship path between two records.
    /// `avoid_record_keys` and `required_data_sources` are JSON documents
    /// (pass empty string for none).
    #[allow(clippy::too_many_arguments)]
    fn find_path_by_record_id(
        &self,
        start_data_source_code: &str,
        start_record_id: &str,
        end_data_source_code: &str,
        end_record_id: &str,
        max_degrees: i64,
        avoid_record_keys: &str,
        required_data_sources: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Returns the configuration ID of the currently active configuration.
    fn get_active_config_id(&self) -> Result<i64, SzError>;

    /// Retrieves entity data by resolved entity ID.
    fn get_entity_by_entity_id(&self, entity_id: i64, flags: i64) -> Result<String, SzError>;

    /// Retrieves entity data by data source code and record ID.
    fn get_entity_by_record_id(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Retrieves a stored record.
    fn get_record(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Previews what a record would look like without loading it.
    fn get_record_preview(&self, record_definition: &str, flags: i64) -> Result<String, SzError>;

    /// Retrieves the next record from the redo queue.
    /// Returns an empty string when the queue is empty.
    fn get_redo_record(&self) -> Result<String, SzError>;

    /// Returns a JSON document with engine processing statistics.
    fn get_stats(&self) -> Result<String, SzError>;

    /// Retrieves information about a virtual entity composed of the given records.
    /// `record_keys` is a JSON document listing the records.
    fn get_virtual_entity_by_record_id(
        &self,
        record_keys: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Explains how an entity was constructed from its base records.
    fn how_entity_by_entity_id(&self, entity_id: i64, flags: i64) -> Result<String, SzError>;

    /// Pre-initializes heavy-weight internal resources of the engine.
    fn prime_engine(&self) -> Result<(), SzError>;

    /// Processes a redo record from the redo queue.
    /// Returns a JSON "with info" document when the `SZ_WITH_INFO` flag is set,
    /// or an empty string otherwise.
    fn process_redo_record(&mut self, redo_record: &str, flags: i64) -> Result<String, SzError>;

    /// Re-evaluates an entity by entity ID.
    /// Returns a JSON "with info" document when the `SZ_WITH_INFO` flag is set,
    /// or an empty string otherwise.
    fn reevaluate_entity(&mut self, entity_id: i64, flags: i64) -> Result<String, SzError>;

    /// Re-evaluates a record by data source code and record ID.
    /// Returns a JSON "with info" document when the `SZ_WITH_INFO` flag is set,
    /// or an empty string otherwise.
    fn reevaluate_record(
        &mut self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Searches for entities matching the given attributes.
    /// `search_profile` may be empty to use the default profile.
    fn search_by_attributes(
        &self,
        attributes: &str,
        search_profile: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Determines how two entities are related to each other.
    fn why_entities(&self, entity_id1: i64, entity_id2: i64, flags: i64)
        -> Result<String, SzError>;

    /// Determines why a particular record is included in its resolved entity.
    fn why_record_in_entity(
        &self,
        data_source_code: &str,
        record_id: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Determines how two records are related to each other.
    fn why_records(
        &self,
        data_source_code1: &str,
        record_id1: &str,
        data_source_code2: &str,
        record_id2: &str,
        flags: i64,
    ) -> Result<String, SzError>;

    /// Determines how a search record relates to an existing entity.
    /// `search_profile` may be empty to use the default profile.
    fn why_search(
        &self,
        attributes: &str,
        entity_id: i64,
        search_profile: &str,
        flags: i64,
    ) -> Result<String, SzError>;
}
