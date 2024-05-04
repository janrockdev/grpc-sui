use proto::admin_server::{Admin, AdminServer};
use proto::model_server::{Model, ModelServer};
use std::time::Instant;
use tonic::metadata::MetadataValue;
use tonic::transport::Server;
use tonic::{Request, Status};

extern crate env_logger;
extern crate log;
use log::info;

pub mod model;
pub mod service_index;

mod proto {
    tonic::include_proto!("model");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("model_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct ModelService {
    state: State,
}

impl ModelService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
    }
}

#[tonic::async_trait]
impl Model for ModelService {
    
    async fn asset(
        &self,
        request: tonic::Request<proto::ModelAssetRequest>,
    ) -> Result<tonic::Response<proto::ModelAssetResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        info!("requested asset object from Atlas MongoDB: {:?}", input);

        let res = match service_index::get_record(input).await {
            Ok(o) => o,
            Err(e) => {
                return Err(tonic::Status::internal(format!(
                    "Error while fetching asset from MongoDB: {:?}",
                    e
                )))
            }
        };

        let response = proto::ModelAssetResponse {
            payload: res.payload.to_string(),
            duration: res.duration,
        };

        Ok(tonic::Response::new(response))
    }

    async fn store(
        &self,
        request: tonic::Request<proto::ModelStoreRequest>,
    ) -> Result<tonic::Response<proto::ModelStoreResponse>, tonic::Status> {
        self.increment_counter().await;
        let start = Instant::now();

        let _input = request.get_ref();

        let response = proto::ModelStoreResponse {
            state: "1".to_string(),
            duration: start.elapsed().as_millis() as u64,
        };

        Ok(tonic::Response::new(response))
    }

    async fn validate(
        &self,
        request: tonic::Request<proto::ModelValidateRequest>,
    ) -> Result<tonic::Response<proto::ModelValidateResponse>, tonic::Status> {
        self.increment_counter().await;
        let start = Instant::now();

        let _input = request.get_ref();

        let response = proto::ModelValidateResponse {
            state: "1".to_string(),
            payload: "1".to_string(),
            duration: start.elapsed().as_millis() as u64,
        };

        Ok(tonic::Response::new(response))
    }

    async fn util(
        &self,
        request: tonic::Request<proto::ModelUtilRequest>,
    ) -> Result<tonic::Response<proto::ModelUtilResponse>, tonic::Status> {
        self.increment_counter().await;
        let start = Instant::now();

        let _input = request.get_ref();

        let response = proto::ModelUtilResponse {
            payload: "1".to_string(),
            duration: start.elapsed().as_millis() as u64,
        };

        Ok(tonic::Response::new(response))
    }
    
}

#[derive(Default, Debug)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_result(
        &self,
        _request: tonic::Request<proto::GetResultRequest>,
    ) -> Result<tonic::Response<proto::ResultResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = proto::ResultResponse { count: *count };

        Ok(tonic::Response::new(response))
    }
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    env_logger::init();

    let addr = "[::1]:50051".parse()?;
    let state = State::default();
    let model = ModelService {
        state: state.clone(),
    };
    let admin = AdminService {
        state: state.clone(),
    };
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    info!("Server is running on: {:?}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .add_service(tonic_web::enable(ModelServer::new(model)))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}