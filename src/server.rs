#[macro_use]
extern crate tracing;

// TODO: Potentially abstract this stuff with my own types for the purposes of
// server initialisation.
use tonic::{transport::Server, Request, Response, Status};

// FIXME: Spantraces currently aren't reporting correctly, despite having an
// ErrorLayer setup.
use color_eyre::eyre::Result;

// TODO: Move gRPC related code to its own module
use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

// TODO: Split logging setup into its own module
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // Must match proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        info!("Handling request.");

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // easy to read stack-traces
    color_eyre::install()?;

    // structured logging setup
    let log_format = tracing_subscriber::fmt::layer();
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("INFO"));

    let subscriber = tracing_subscriber::Registry::default()
        .with(log_format)
        .with(env_filter)
        .with(ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber)?;

    let addr = "0.0.0.0:5001".parse()?;
    let greeter = MyGreeter::default();
    let grpc = Server::builder()
        .trace_fn(|_| tracing::info_span!("helloworld-server"))
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
