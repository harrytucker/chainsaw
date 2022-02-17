use chainsaw_middleware::auth::UserIdExtension;
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

        if let Some(user_id) = request.extensions().get::<UserIdExtension>() {
            tracing::info!(?user_id, "user id from jwt sub claim");
        } else {
            tracing::warn!("missing user id from jwt sub claim");
        }

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
