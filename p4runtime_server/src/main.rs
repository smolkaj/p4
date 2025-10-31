mod p4runtime_p4rs_translator;

use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

use p4runtime_p4rs_translator::P4RuntimeP4rsTranslator;
use p4runtime_prost::p4::config::v1::P4Info;
use p4runtime_prost::p4::v1::ForwardingPipelineConfig;
use p4runtime_prost::p4::v1::{
    CapabilitiesRequest, CapabilitiesResponse,
    GetForwardingPipelineConfigRequest, GetForwardingPipelineConfigResponse,
    ReadRequest, ReadResponse, SetForwardingPipelineConfigRequest,
    SetForwardingPipelineConfigResponse, StreamMessageRequest,
    StreamMessageResponse, WriteRequest, WriteResponse,
};
use p4runtime_tonic::p4::v1::p4_runtime_server::{P4Runtime, P4RuntimeServer};

// Compile the hub P4 program statically.
p4_macro::use_p4!(p4 = "test/src/p4/hub.p4", pipeline_name = "hub",);

pub struct P4RuntimeService {
    translator: Arc<Mutex<Option<P4RuntimeP4rsTranslator>>>,
}

impl Default for P4RuntimeService {
    fn default() -> Self {
        Self {
            translator: Arc::new(Mutex::new(None)),
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
            ..Default::default()
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

        // The macro generates {pipeline_name}_pipeline, so for "hub" it's hub_pipeline.
        // But actually, based on the codegen, it generates main_pipeline by default
        // and {pipeline_name}_pipeline when specified. Let's use main_pipeline for now.
        let pipeline =
            Box::new(main_pipeline::new(0)) as Box<dyn p4rs::Pipeline>;

        let translator = P4RuntimeP4rsTranslator::new(pipeline, p4info)?;

        let mut translator_guard = self.translator.lock().unwrap();
        *translator_guard = Some(translator);

        Ok(Response::new(SetForwardingPipelineConfigResponse {}))
    }

    async fn get_forwarding_pipeline_config(
        &self,
        request: Request<GetForwardingPipelineConfigRequest>,
    ) -> Result<Response<GetForwardingPipelineConfigResponse>, Status> {
        let _req = request.into_inner();

        let translator_guard = self.translator.lock().unwrap();

        let translator = translator_guard.as_ref().ok_or_else(|| {
            Status::failed_precondition("no pipeline configured")
        })?;

        let config = ForwardingPipelineConfig {
            p4info: Some(translator.p4info.clone()),
            p4_device_config: vec![], // For static pipeline, device config is empty.
            ..Default::default()
        };

        let response = GetForwardingPipelineConfigResponse {
            config: Some(config),
            ..Default::default()
        };

        Ok(Response::new(response))
    }

    async fn write(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        let req = request.into_inner();

        let mut translator_guard = self.translator.lock().unwrap();

        let _translator = translator_guard.as_mut().ok_or_else(|| {
            Status::failed_precondition("no pipeline configured")
        })?;

        // Process each update in the write request.
        for _update in req.updates {
            // For MVP, we'll just return unimplemented for actual table writes.
            // TODO: Implement table entry writing.
            return Err(Status::unimplemented(
                "table writes not yet implemented",
            ));
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
