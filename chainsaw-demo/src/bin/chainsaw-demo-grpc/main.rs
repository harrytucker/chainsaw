#[macro_use]
extern crate tracing;

use std::iter::once;

use chainsaw::{config::get_configuration, Result};
use chainsaw_middleware::auth::ParseJWTGrpcAuth;
use chainsaw_observe::logging;
use chainsaw_proto::helloworld::v1::greeter_server::GreeterServer;
use greeter::MyGreeter;
use hyper::header;
use tokio::signal;
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::auth::RequireAuthorizationLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;

mod greeter;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;

    let subscriber = logging::new_subscriber(configuration.log.level)?;
    logging::set_global_logger(subscriber)?;

    // TODO: Wrapping the function that returns these types in order to set the
    // global serving status doesn't work due to some type-system shenanigans.
    //
    // Health service ergonomics may work better by instead initialising the
    // HealthReporter and HealthServer within a Chainsaw::Server type than
    // handling this in fn main().
    let (mut health_report, health_service) = tonic_health::server::health_reporter();
    health_report
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;

    let addr = configuration.grpc.unwrap().serve_addr();
    let greeter = MyGreeter::default();

    let auth_paths = vec!["/helloworld.v1.Greeter/UUIDGen".to_string()];

    let layer = ServiceBuilder::new()
        .layer(SetSensitiveHeadersLayer::new(once(header::AUTHORIZATION)))
        .layer(RequireAuthorizationLayer::custom(ParseJWTGrpcAuth::new(
            auth_paths,
        )))
        .into_inner();

    let grpc = Server::builder()
        .trace_fn(|_| tracing::info_span!("chainsaw-server"))
        .layer(layer)
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
