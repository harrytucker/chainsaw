pub mod server;

use server::hello_world::greeter_server::GreeterServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5001".parse()?;
    let greeter = server::MyGreeter::default();

    println!("Hello, one!");
    let grpc = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    tokio::try_join!(grpc)?;
    println!("Hello, two!");
    Ok(())
}
