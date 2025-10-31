mod p4runtime_p4rs_translator;

use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

use p4runtime_p4rs_translator::P4RuntimeP4rsTranslator;
use p4runtime_prost::p4::v1::{
    entity, field_match, table_action, update::Type as UpdateType,
    CapabilitiesRequest, CapabilitiesResponse, ForwardingPipelineConfig,
    GetForwardingPipelineConfigRequest, GetForwardingPipelineConfigResponse,
    ReadRequest, ReadResponse, SetForwardingPipelineConfigRequest,
    SetForwardingPipelineConfigResponse, StreamMessageRequest,
    StreamMessageResponse, Update, WriteRequest, WriteResponse,
};
use p4runtime_tonic::p4::v1::p4_runtime_server::{P4Runtime, P4RuntimeServer};

// Compile the hub P4 program statically.
p4_macro::use_p4!(p4 = "test/src/p4/hub.p4", pipeline_name = "hub");

/// A configured P4 pipeline with its translation layer.
pub struct ConfiguredPipeline {
    /// The p4rs pipeline instance for packet processing and table operations.
    pub pipeline: Box<dyn p4rs::Pipeline>,

    /// Translates between P4Runtime identifiers and p4rs Pipeline identifiers.
    pub translator: P4RuntimeP4rsTranslator,

    /// P4Info describing the pipeline.
    pub p4info: p4runtime_prost::p4::config::v1::P4Info,
}

pub struct P4RuntimeService {
    /// The currently configured pipeline, if any.
    configured_pipeline: Arc<Mutex<Option<ConfiguredPipeline>>>,
}

impl Default for P4RuntimeService {
    fn default() -> Self {
        Self {
            configured_pipeline: Arc::new(Mutex::new(None)),
        }
    }
}

#[tonic::async_trait]
impl P4Runtime for P4RuntimeService {
    async fn capabilities(
        &self,
        _request: Request<CapabilitiesRequest>,
    ) -> Result<Response<CapabilitiesResponse>, Status> {
        Ok(Response::new(CapabilitiesResponse {
            p4runtime_api_version: "1.4.1".to_string(),
            experimental: None,
        }))
    }

    async fn set_forwarding_pipeline_config(
        &self,
        request: Request<SetForwardingPipelineConfigRequest>,
    ) -> Result<Response<SetForwardingPipelineConfigResponse>, Status> {
        let req = request.into_inner();
        let config = req
            .config
            .ok_or_else(|| Status::invalid_argument("missing config"))?;

        // Extract P4Info.
        let p4info = config
            .p4info
            .ok_or_else(|| Status::invalid_argument("missing p4info"))?;

        // For MVP, we'll use the statically compiled hub pipeline.
        // In the future, we could use the p4info to select different pipelines
        // or load dynamically from p4_device_config.
        let pipeline =
            Box::new(main_pipeline::new(0)) as Box<dyn p4rs::Pipeline>;

        let translator =
            P4RuntimeP4rsTranslator::new(pipeline.as_ref(), &p4info)?;

        // Store pipeline, translator, and P4Info together, ensuring they're always in sync.
        let mut guard = self
            .configured_pipeline
            .lock()
            .map_err(|e| Status::internal(format!("mutex poisoned: {}", e)))?;
        *guard = Some(ConfiguredPipeline {
            pipeline,
            translator,
            p4info,
        });

        Ok(Response::new(SetForwardingPipelineConfigResponse {}))
    }

    async fn get_forwarding_pipeline_config(
        &self,
        request: Request<GetForwardingPipelineConfigRequest>,
    ) -> Result<Response<GetForwardingPipelineConfigResponse>, Status> {
        let _req = request.into_inner();

        let guard = self
            .configured_pipeline
            .lock()
            .map_err(|e| Status::internal(format!("mutex poisoned: {}", e)))?;

        let configured = guard.as_ref().ok_or_else(|| {
            Status::failed_precondition("no pipeline configured")
        })?;

        let config = ForwardingPipelineConfig {
            p4info: Some(configured.p4info.clone()),
            p4_device_config: vec![], // For static pipeline, device config is empty.
            cookie: None,
        };

        let response = GetForwardingPipelineConfigResponse {
            config: Some(config),
        };

        Ok(Response::new(response))
    }

    async fn write(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        let req = request.into_inner();

        let mut guard = self
            .configured_pipeline
            .lock()
            .map_err(|e| Status::internal(format!("mutex poisoned: {}", e)))?;
        let configured = guard.as_mut().ok_or_else(|| {
            Status::failed_precondition("no pipeline configured")
        })?;

        // Process each update in the write request.
        for update in req.updates {
            process_update(
                update,
                &configured.translator,
                configured.pipeline.as_mut(),
            )?;
        }

        Ok(Response::new(WriteResponse {}))
    }

    async fn read(
        &self,
        _request: Request<ReadRequest>,
    ) -> Result<Response<Self::ReadStream>, Status> {
        // For MVP, return empty stream.
        // TODO: Implement table reads.
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        drop(tx); // Close sender immediately.

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }

    type ReadStream =
        tokio_stream::wrappers::ReceiverStream<Result<ReadResponse, Status>>;

    type StreamChannelStream = tokio_stream::wrappers::ReceiverStream<
        Result<StreamMessageResponse, Status>,
    >;

    async fn stream_channel(
        &self,
        _request: Request<tonic::Streaming<StreamMessageRequest>>,
    ) -> Result<Response<Self::StreamChannelStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

/// Process a single P4Runtime update.
fn process_update(
    update: Update,
    translator: &P4RuntimeP4rsTranslator,
    pipeline: &mut dyn p4rs::Pipeline,
) -> Result<(), Status> {
    // Extract update type first, before moving update.entity.
    let update_type = update.r#type();

    let entity = update
        .entity
        .ok_or_else(|| Status::invalid_argument("update missing entity"))?;

    // Currently only handle TableEntry entities.
    let entry = match entity.entity {
        Some(entity::Entity::TableEntry(entry)) => entry,
        _ => {
            return Err(Status::unimplemented(format!(
                "entity type not yet supported: {:?}",
                entity.entity
            )));
        }
    };

    // Extract table ID and map to p4rs table name.
    let table_id = entry.table_id;
    let table_name = translator.map_table_id(table_id)?.to_string();

    // Extract match fields and build keyset_data according to p4rs format.
    // Currently only exact matches are supported.
    let mut keyset_data = Vec::new();
    for field in &entry.r#match {
        match &field.field_match_type {
            Some(field_match::FieldMatchType::Exact(exact)) => {
                // Exact: just the value bytes.
                keyset_data.extend_from_slice(&exact.value);
            }
            Some(field_match::FieldMatchType::Ternary(_ternary)) => {
                return Err(Status::unimplemented(
                    "ternary match type not yet supported",
                ));
            }
            Some(field_match::FieldMatchType::Lpm(_lpm)) => {
                return Err(Status::unimplemented(
                    "LPM match type not yet supported",
                ));
            }
            Some(field_match::FieldMatchType::Optional(_optional)) => {
                return Err(Status::unimplemented(
                    "optional match type not yet supported",
                ));
            }
            Some(field_match::FieldMatchType::Range(_range)) => {
                return Err(Status::unimplemented(
                    "range match type not yet supported",
                ));
            }
            Some(field_match::FieldMatchType::Other(_)) => {
                return Err(Status::unimplemented(
                    "other match types not yet supported",
                ));
            }
            None => {
                return Err(Status::invalid_argument(format!(
                    "match field missing field_match_type: {}",
                    field.field_id
                )));
            }
        }
    }

    // Extract all needed values before borrowing translator mutably.
    let (action_name, parameter_data, priority) = match update_type {
        UpdateType::Insert | UpdateType::Modify => {
            // Extract action and parameters.
            let action = entry.action.ok_or_else(|| {
                Status::invalid_argument("table entry missing action")
            })?;

            // Extract action ID and parameters based on action type.
            let (action_id, params) = match action.r#type {
                Some(table_action::Type::Action(action)) => {
                    let action_id = action.action_id;
                    let params = action
                        .params
                        .iter()
                        .flat_map(|p| p.value.clone())
                        .collect::<Vec<u8>>();
                    (action_id, params)
                }
                Some(table_action::Type::ActionProfileMemberId(_member_id)) => {
                    return Err(Status::unimplemented(
                        "action profile members not yet supported",
                    ));
                }
                Some(table_action::Type::ActionProfileGroupId(_group_id)) => {
                    return Err(Status::unimplemented(
                        "action profile groups not yet supported",
                    ));
                }
                Some(table_action::Type::ActionProfileActionSet(
                    _action_set,
                )) => {
                    return Err(Status::unimplemented(
                        "action profile action sets not yet supported",
                    ));
                }
                None => {
                    return Err(Status::invalid_argument(
                        "action missing type",
                    ));
                }
            };

            // Map action ID to p4rs action name.
            let action_name = translator.map_action_id(action_id)?.to_string();

            // Extract priority and convert from i32 to u32.
            let priority = entry.priority.try_into().map_err(|_| {
                Status::invalid_argument(format!(
                    "invalid priority: {}",
                    entry.priority
                ))
            })?;

            (action_name, params, priority)
        }
        UpdateType::Delete => {
            // For DELETE, we don't need action/parameters/priority.
            // Return empty strings and 0 for unused values.
            (String::new(), Vec::new(), 0)
        }
        UpdateType::Unspecified => {
            return Err(Status::invalid_argument("update type unspecified"));
        }
    };

    // Call pipeline methods directly.
    match update_type {
        UpdateType::Insert | UpdateType::Modify => {
            // Add or modify the table entry.
            pipeline.add_table_entry(
                &table_name,
                &action_name,
                &keyset_data,
                &parameter_data,
                priority,
            );
        }
        UpdateType::Delete => {
            // Delete the table entry.
            pipeline.remove_table_entry(&table_name, &keyset_data);
        }
        UpdateType::Unspecified => {
            return Err(Status::invalid_argument("update type unspecified"));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:9559".parse()?;

    let p4runtime_service = P4RuntimeService::default();
    let reflection_service = ReflectionBuilder::configure()
        .register_file_descriptor_set(p4runtime_prost::file_descriptor_set())
        .build_v1()?;

    println!("P4Runtime server starting on {}", addr);
    println!("Using statically compiled hub.p4 pipeline");

    Server::builder()
        .add_service(P4RuntimeServer::new(p4runtime_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
