use tracing::{subscriber::set_global_default, Subscriber};
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

/// Returns a [`tracing`] subscriber to receive structured logging events.
///
/// To set this as the global logger, as well as to receive events from the
/// standard library log facade, call [`set_global_logger`].
pub fn new_subscriber<S: Into<String>>(env_filter: S) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.into()));
    let log_format = tracing_subscriber::fmt::layer();

    Registry::default()
        .with(env_filter)
        .with(log_format)
        .with(ErrorLayer::default())
}

/// Initialises [`LogTracer`] to capture log events with [`tracing`], and sets
/// the given subscriber as the global default subscriber for structured logging
/// events. Also enables [`color_eyre`] error and panic handling hooks for
/// developer happiness.
///
/// Calling this twice will result in a code panic.
pub fn set_global_logger(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to setup standard library log receiver.");
    set_global_default(subscriber).expect("Failed to set global logging subscriber.");
    color_eyre::install().expect(
        "Chainsaw failed to configure color-eyre, did you already call color_eyre::install()?",
    )
}
