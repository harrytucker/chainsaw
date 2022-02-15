#[macro_use]
extern crate tracing;

use crate::{
    config::get_configuration,
    grpc_impl::MyGreeter,
};
use chainsaw_proto::helloworld::greeter_server::GreeterServer;
use chainsaw_demo::{
    health::{self, ServingStatus},
    logging, Result,
};
use tokio::signal;
use tonic::transport::Server;

mod config;
mod grpc_impl;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;

    let subscriber = logging::new_subscriber(configuration.log.level);
    logging::set_global_logger(subscriber);

    // TODO: Wrapping the function that returns these types in order to set the
    // global serving status doesn't work due to some type-system shenanigans.
    //
    // Health service ergonomics may work better by instead initialising the
    // HealthReporter and HealthServer within a Chainsaw::Server type than
    // handling this in fn main().
    let (mut health_report, health_service) = health::reporter();
    chainsaw_demo::health::set_global_status(health_report.clone(), ServingStatus::Serving).await;
    health_report
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;

    let addr = configuration.grpc.serve_addr();
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
