//! Translator layer to translate between P4Runtime and p4rs Pipeline trait.

use p4rs::Pipeline;
use p4runtime_prost::p4::config::v1::P4Info;
use std::collections::HashMap;
use tonic::Status;

/// Translates between P4Runtime identifiers and p4rs Pipeline identifiers.
pub struct P4RuntimeP4rsTranslator {
    /// Map from P4Runtime table ID to p4rs qualified table name.
    pub table_id_to_name: HashMap<u32, String>,

    /// Map from p4rs qualified table name to P4Runtime table ID.
    pub table_name_to_id: HashMap<String, u32>,

    /// Map from P4Runtime action ID to action name.
    pub action_id_to_name: HashMap<u32, String>,

    /// Map from action name to P4Runtime action ID.
    pub action_name_to_id: HashMap<String, u32>,
}

impl P4RuntimeP4rsTranslator {
    /// Create a new translator from a pipeline and P4Info.
    /// The pipeline is used to get the p4rs table names for matching with P4Info.
    pub fn new(
        pipeline: &dyn Pipeline,
        p4info: &P4Info,
    ) -> Result<Self, Status> {
        let mut table_id_to_name = HashMap::new();
        let mut table_name_to_id = HashMap::new();
        let mut action_id_to_name = HashMap::new();
        let mut action_name_to_id = HashMap::new();

        // Get the p4rs table names from the pipeline.
        let pipeline_table_names: Vec<String> = pipeline
            .get_table_ids()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Build table mappings from P4Info.
        // P4Info table names are fully qualified (e.g., "MyControl.table_name").
        // We need to match them with p4rs qualified names from the pipeline.
        for table in &p4info.tables {
            let preamble = table.preamble.as_ref().ok_or_else(|| {
                Status::invalid_argument(format!(
                    "table missing preamble"
                ))
            })?;

            let table_id = preamble.id;
            let p4info_name = &preamble.name;

            // Find matching p4rs table name using a two-stage matching strategy:
            //
            // 1. Exact match: Try to match the full P4Info name (e.g., "MyControl.table_name")
            //    against p4rs qualified names (e.g., "ingress.MyControl.table_name").
            //
            // 2. Suffix match: If no exact match, extract the table name component (last
            //    segment after '.') and match against p4rs names ending with that component.
            //    This handles cases where P4Info uses control-block-qualified names but
            //    p4rs uses ingress/egress-qualified names.
            //
            // Assumptions:
            // - P4Info names may be fully qualified (control.table) or unqualified (table)
            // - p4rs names are always fully qualified (ingress/egress.control.table)
            // - Table names are unique within a control block
            // - split('.').last() is safe: if there's no '.', it returns the whole string
            let p4info_table_name = p4info_name.split('.').last().unwrap_or(p4info_name);
            
            let p4rs_name = pipeline_table_names
                .iter()
                .find(|&p4rs_name| {
                    // Exact match first.
                    p4rs_name == p4info_name
                })
                .or_else(|| {
                    // Try matching by table name only (last component).
                    pipeline_table_names
                        .iter()
                        .find(|&p4rs_name| {
                            p4rs_name.ends_with(p4info_table_name) ||
                            p4rs_name == p4info_table_name
                        })
                })
                .ok_or_else(|| {
                    Status::invalid_argument(format!(
                        "table '{}' (ID {}) not found in pipeline. Available tables: {:?}",
                        p4info_name,
                        table_id,
                        pipeline_table_names
                    ))
                })?;

            // Store mappings in both directions.
            table_id_to_name.insert(table_id, p4rs_name.clone());
            table_name_to_id.insert(p4rs_name.clone(), table_id);
        }

        // Build action mappings from P4Info.
        // P4Info action names are fully qualified (e.g., "MyControl.action_name").
        // The p4rs action names are just the action name (no qualification).
        for action in &p4info.actions {
            let preamble = action.preamble.as_ref().ok_or_else(|| {
                Status::invalid_argument(format!(
                    "action missing preamble"
                ))
            })?;

            let action_id = preamble.id;
            let p4info_name = &preamble.name;

            // Extract the action name (last component after the dot).
            // P4Info action names are fully qualified (e.g., "MyControl.action_name"),
            // while p4rs action names are unqualified (just "action_name").
            //
            // Assumptions:
            // - P4Info action names may be fully qualified or unqualified
            // - p4rs action names are always unqualified (no control block prefix)
            // - split('.').last() is safe: if there's no '.', it returns the whole string
            let action_name = p4info_name
                .split('.')
                .last()
                .unwrap_or(p4info_name)
                .to_string();

            // Store mappings in both directions.
            action_id_to_name.insert(action_id, action_name.clone());
            action_name_to_id.insert(action_name, action_id);
        }

        Ok(Self {
            table_id_to_name,
            table_name_to_id,
            action_id_to_name,
            action_name_to_id,
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

    /// Map an action name to P4Runtime action ID.
    pub fn map_action_name(&self, name: &str) -> Result<u32, Status> {
        self.action_name_to_id.get(name).copied().ok_or_else(|| {
            Status::not_found(format!("action name {} not found", name))
        })
    }
}
