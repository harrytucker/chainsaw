#[macro_use]
extern crate tracing;

use crate::{
    grpc::{hello_service::greeter_server::GreeterServer, MyGreeter},
    logging::init_subscriber,
};
use color_eyre::eyre::Result;
use tonic::transport::Server;

mod grpc;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    // easy to read stack-traces
    color_eyre::install()?;

    let subscriber = logging::get_subscriber("info"); // default logging level
    init_subscriber(subscriber);

    let addr = "0.0.0.0:5001".parse()?;
    let greeter = MyGreeter::default();
    let grpc = Server::builder()
        .trace_fn(|_| tracing::info_span!("chainsaw-server"))
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    // TODO: The gRPC server should be able to start up alongside other APIs,
    // such as extra gRPC services, HTTP endpoints, metrics etc.
    //
    // This should also work with neatly terminating all running Tokio tasks
    // using SIGINT. See the mini_redis server example in the Tonic repo.
    info!(?addr, "gRPC server starting.");
    tokio::try_join!(grpc)?;
    Ok(())
}
