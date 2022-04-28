use std::time::Duration;

use crate::GreeterServer;
use crate::MyGreeter;
use tonic_health::server::HealthReporter;

/// Example background task that toggles the service health for the Greeter
/// service to demonstrate how to run other tasks while serving gRPC.
///
/// A tokio::spawn(task) is analogous to a `go func()` invocation.
#[tracing::instrument]
pub async fn modify_server_health(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(3)).await;

        if iter % 2 == 0 {
            reporter.set_serving::<GreeterServer<MyGreeter>>().await;
        } else {
            reporter.set_not_serving::<GreeterServer<MyGreeter>>().await;
        };
    }
}
