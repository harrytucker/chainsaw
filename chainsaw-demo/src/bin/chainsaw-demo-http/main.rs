use crate::config::get_configuration;
use axum::{routing, Router};
use chainsaw_demo::{logging, Result};
use tokio::signal;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration()?;

    let subscriber = logging::new_subscriber(configuration.log.level)?;
    logging::set_global_logger(subscriber)?;

    let http_addr = configuration.http.serve_addr();
    let http_router = Router::new()
        .route("/", routing::get(root))
        .layer(logging::http_trace_layer());
    let http = axum::Server::bind(&http_addr).serve(http_router.into_make_service());

    tracing::info!(?http, "Revving up HTTP Chainsaw!");
    tokio::spawn(http);

    signal::ctrl_c()
        .await
        .expect("Unable to listen for shutdown signal.");
    tracing::info!("Revving down Chainsaw...");
    Ok(())
}

#[tracing::instrument]
async fn root() -> &'static str {
    "Hello, world!\n"
}
