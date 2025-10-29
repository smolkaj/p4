use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

use p4runtime_prost::p4::v1::{
    CapabilitiesRequest, CapabilitiesResponse,
    GetForwardingPipelineConfigRequest, GetForwardingPipelineConfigResponse,
    ReadRequest, ReadResponse, SetForwardingPipelineConfigRequest,
    SetForwardingPipelineConfigResponse, StreamMessageRequest,
    StreamMessageResponse, WriteRequest, WriteResponse,
};
use p4runtime_tonic::p4::v1::p4_runtime_server::{P4Runtime, P4RuntimeServer};

#[derive(Debug, Default)]
pub struct P4RuntimeService {}

#[tonic::async_trait]
impl P4Runtime for P4RuntimeService {
    async fn write(
        &self,
        _request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn read(
        &self,
        _request: Request<ReadRequest>,
    ) -> Result<Response<Self::ReadStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn set_forwarding_pipeline_config(
        &self,
        _request: Request<SetForwardingPipelineConfigRequest>,
    ) -> Result<Response<SetForwardingPipelineConfigResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_forwarding_pipeline_config(
        &self,
        _request: Request<GetForwardingPipelineConfigRequest>,
    ) -> Result<Response<GetForwardingPipelineConfigResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    type ReadStream = tokio_stream::wrappers::ReceiverStream<
        Result<ReadResponse, Status>,
    >;

    type StreamChannelStream = tokio_stream::wrappers::ReceiverStream<
        Result<StreamMessageResponse, Status>,
    >;

    async fn capabilities(
        &self,
        _request: Request<CapabilitiesRequest>,
    ) -> Result<Response<CapabilitiesResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

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

    Server::builder()
        .add_service(P4RuntimeServer::new(p4runtime_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
