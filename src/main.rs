#[macro_use]
extern crate tracing;

use crate::{
    config::get_configuration, grpc::helloworld::greeter_server::GreeterServer,
    grpc_impl::MyGreeter, logging::init_subscriber,
};
use color_eyre::eyre::Result;
use tokio::signal;
use tonic::transport::Server;

mod config;
mod grpc;
mod grpc_impl;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Fancy span traces are omitted from the program when compiling in release
    // mode to avoid impacting performance.
    if cfg!(debug_assertions) {
        color_eyre::install()?;
    }

    let configuration = get_configuration()?;

    let subscriber = logging::get_subscriber("info"); // default logging level
    init_subscriber(subscriber);

    let (mut health_report, health_service) = tonic_health::server::health_reporter();
    health_report
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;
    health_report
        .set_service_status(
            "", /* clients may choose not to specify gRPC service on healthcheck */
            tonic_health::ServingStatus::Serving,
        )
        .await;

    let addr = format!("{}:{}", configuration.grpc.address, configuration.grpc.port).parse()?;
    let greeter = MyGreeter::default();
    let grpc = Server::builder()
        .trace_fn(|_| tracing::info_span!("chainsaw-server"))
        .add_service(health_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    // TODO: The gRPC server should be able to start up alongside other APIs,
    // such as extra gRPC services, HTTP endpoints, metrics etc.
    info!(?addr, "Revving up Chainsaw!");
    tokio::spawn(grpc);

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Revving down Chainsaw...");
            Ok(())
        }
        Err(err) => {
            error!(
                err = err.to_string().as_str(),
                "Unable to listen for shutdown signal."
            );
            Err(err.into())
        }
    }
}
