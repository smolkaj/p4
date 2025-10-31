//! Translator layer to translate between P4Runtime and p4rs Pipeline trait.

use p4rs::Pipeline;
use p4runtime_prost::p4::config::v1::P4Info;
use std::collections::HashMap;
use tonic::Status;

/// Translates between P4Runtime identifiers and p4rs Pipeline identifiers.
pub struct P4RuntimeP4rsTranslator {
    /// The underlying p4rs pipeline.
    pub pipeline: Box<dyn Pipeline>,

    /// P4Info describing the pipeline.
    pub p4info: P4Info,

    /// Map from P4Runtime table ID to p4rs qualified table name.
    pub table_id_to_name: HashMap<u32, String>,

    /// Map from p4rs qualified table name to P4Runtime table ID.
    pub table_name_to_id: HashMap<String, u32>,

    /// Map from P4Runtime action ID to action name.
    pub action_id_to_name: HashMap<u32, String>,
}

impl P4RuntimeP4rsTranslator {
    /// Create a new translator from a pipeline and P4Info.
    pub fn new(
        pipeline: Box<dyn Pipeline>,
        p4info: P4Info,
    ) -> Result<Self, Status> {
        let mut table_id_to_name = HashMap::new();
        let mut table_name_to_id = HashMap::new();
        let mut action_id_to_name = HashMap::new();

        // Build table mappings from P4Info.
        // For now, we'll build a simple mapping. In a full implementation,
        // we'd parse P4Info to extract table metadata with annotations.
        // The p4rs qualified names follow the pattern: "control.table" or "ingress.control.table".

        // Build action mappings.
        // Similar to tables, we need to parse P4Info action metadata.

        // For MVP, we'll create a simple fallback: use the pipeline's get_table_ids()
        // and match them heuristically with P4Info table data.
        let _pipeline_table_names: Vec<String> = pipeline
            .get_table_ids()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // TODO: Parse P4Info tables and actions to build proper mappings.
        // For MVP, we'll use empty mappings and handle errors at runtime.

        Ok(Self {
            pipeline,
            p4info,
            table_id_to_name,
            table_name_to_id,
            action_id_to_name,
        })
    }

    /// Map a P4Runtime table ID to p4rs qualified table name.
    pub fn map_table_id(&self, id: u32) -> Result<&str, Status> {
        self.table_id_to_name
            .get(&id)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                Status::not_found(format!("table ID {} not found", id))
            })
    }

    /// Map a p4rs qualified table name to P4Runtime table ID.
    pub fn map_table_name(&self, name: &str) -> Result<u32, Status> {
        self.table_name_to_id.get(name).copied().ok_or_else(|| {
            Status::not_found(format!("table name {} not found", name))
        })
    }

    /// Map a P4Runtime action ID to action name.
    pub fn map_action_id(&self, id: u32) -> Result<&str, Status> {
        self.action_id_to_name
            .get(&id)
            .map(|s| s.as_str())
            .ok_or_else(|| {
                Status::not_found(format!("action ID {} not found", id))
            })
    }
}
