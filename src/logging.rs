use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

pub fn get_subscriber<S: Into<String>>(env_filter: S) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.into()));
    let log_format = tracing_subscriber::fmt::layer();

    Registry::default()
        .with(env_filter)
        .with(log_format)
        .with(ErrorLayer::default())
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to setup standard library log receiver.");
    set_global_default(subscriber).expect("Failed to set global logging subscriber.");
}
