use chainsaw_middleware::auth::UserIdExtension;
use chainsaw_proto::helloworld::v1::{
    greeter_server::Greeter, HelloReply, HelloRequest, UuidGenReply, UuidGenRequest,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    /// gRPC endpoint that responds with a greeting for the supplied name.
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

    /// gRPC endpoint that responds with a randomly generated UUID.
    #[tracing::instrument]
    async fn uuid_gen(
        &self,
        _request: Request<UuidGenRequest>,
    ) -> Result<Response<UuidGenReply>, Status> {
        tracing::info!("Handling request.");
        Err(Status::unimplemented("uuid_gen not implemented"))
    }
}

#[cfg(test)]
mod test {
    use tonic::IntoRequest;

    use super::*;
    use crate::Result;

    #[tokio::test]
    async fn greeter_happy_path() -> Result<()> {
        let greeter = MyGreeter::default();
        let request = HelloRequest::default();

        let resp = greeter.say_hello(request.into_request()).await?;
        assert!(resp.into_inner().message == "Hello !");
        Ok(())
    }
}
