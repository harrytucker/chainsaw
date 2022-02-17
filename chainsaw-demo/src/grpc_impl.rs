use std::sync::Arc;

use chainsaw_proto::helloworld::v1::{
    greeter_server::Greeter, HelloReply, HelloRequest, UuidGenReply, UuidGenRequest,
};
use tonic::{Request, Response, Status};

use crate::usecases::HelloUseCase;

#[derive(Debug)]
pub struct MyGreeter {
    usecase: Arc<Box<dyn HelloUseCase>>,
}

impl MyGreeter {
    pub fn new(usecase: Arc<Box<dyn HelloUseCase>>) -> Self {
        Self { usecase }
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Handling request.");

        let reply = HelloReply {
            message: self.usecase.execute(&request.into_inner().name),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::usecases::MockHelloUseCase;
    use mockall::predicate;

    #[tokio::test]
    async fn test_say_hello() -> Result<(), Status> {
        let mut mock = MockHelloUseCase::new();

        mock.expect_execute()
            .with(predicate::eq("Bob"))
            .times(1)
            .returning(|name| format!("Hello {}", name));

        let request = Request::new(HelloRequest {
            name: "Bob".to_string(),
        });

        let greeter = MyGreeter::new(Arc::new(Box::new(mock)));
        greeter.say_hello(request).await?;

        Ok(())
    }
}
