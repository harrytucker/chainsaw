//! gRPC health checking service.
//!
//! This module provides tools for microservices to provide the standard gRPC
//! health checking service to gRPC clients.

pub use tonic_health::{
    server::health_reporter as reporter, server::HealthReporter, ServingStatus,
};

/// Uses a [`HealthReporter`] to set the global service status for the Chainsaw
/// server.
///
/// # Example
///
/// ```
/// use chainsaw_demo::health::{self, ServingStatus};
///
/// # async fn example() {
/// let (mut health_report, health_service) = health::reporter();
/// health::set_global_status(health_report.clone(), ServingStatus::Serving).await;
/// # }
/// ```
pub async fn set_global_status(mut reporter: HealthReporter, status: ServingStatus) {
    // The `grpc.healthy.v1` specification allows clients to send an empty
    // string for the service field if they do not care about a specific service.
    //
    // [`tonic_health`] doesn't handle this case by default, so we need to
    // handle this ourselves here.
    reporter
        .set_service_status(
            "", // service name
            status,
        )
        .await;
}
