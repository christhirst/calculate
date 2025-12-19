use hyper::Error;
use proto::indicator_server::IndicatorServer;
use tonic::transport::Server;
use tracing::span::Id;
use tracing::{debug, info, warn};

mod config;
mod grpc;
mod pattern;

pub mod proto {
    tonic::include_proto!("calculate");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("indicator_descriptor");
}

//TODO trader:
//1. Gets data candles from exchange
//2. Gets indicators
//3. Compare indicators with different patterns
//4. mix them
//5. Send strategy to trader
//6. Trader is updateable with new strategies and patterns
//7. Surviver strategy

#[derive(Debug, Default)]
pub struct IndicatorService;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    info!("Starting GRPC server...");

    let addr = "[::1]:50051".parse().unwrap();
    //GRPC server
    let calc = IndicatorService::default();
    //GRPC reflection
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .accept_http1(true)
        //.layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(IndicatorServer::new(calc))
        //.add_service(tonic_web::enable(CalculatorServer::new(calc)))
        //.add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await
        .unwrap();
    todo!()
}
