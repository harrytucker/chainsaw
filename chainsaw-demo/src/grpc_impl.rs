use chainsaw_proto::helloworld::v1::{
    greeter_server::Greeter, HelloReply, HelloRequest, UuidGenReply, UuidGenRequest,
};
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
        tracing::info!("Handling request.");

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    #[tracing::instrument]
    async fn uuid_gen(
        &self,
        _request: Request<UuidGenRequest>,
    ) -> Result<Response<UuidGenReply>, Status> {
        tracing::info!("Handling request.");
        Err(Status::unimplemented("uuid_gen not implemented"))
    }
}
