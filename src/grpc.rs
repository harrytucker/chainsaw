pub mod hello_service {
    tonic::include_proto!("helloworld"); // must match proto package name
}

use hello_service::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

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

        let reply = hello_service::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
