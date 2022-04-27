//! Structured logging gives us the ability to more easily programmatically
//! interact with logs produced by applications. The most widely used crate for
//! structured logging is the [`tracing`] crate, so this module provides some
//! opinionated configurations to allow you to use [`tracing`] in your
//! microservice.

use color_eyre::Result;

use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{subscriber::set_global_default, Level, Subscriber};
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::filter_fn, prelude::*, EnvFilter};

/// Tokio Console requires that the [`tokio`] and `runtime` targets be logged at
/// the TRACE level. This constant is used to add a directive to [`EnvFilter`]
/// to do this.
const TOKIO_CONSOLE_FILTERS: &str = "tokio=trace";
const RUNTIME_CONSOLE_FILTERS: &str = "runtime=trace";

/// Returns a [`tracing`] subscriber to receive structured logging events.
///
/// To set this as the global logger, as well as to receive events from the
/// standard library log facade, call [`set_global_logger`].
pub fn new_subscriber<L: Into<Level>>(log_level: L) -> Result<impl Subscriber + Send + Sync> {
    // Filters tracing events based on the RUST_LOG environment variable, or
    // `env_filter` if RUST_LOG is not set.
    let log_level = log_level.into();
    let env_filter = EnvFilter::from(log_level.as_str())
        .add_directive(TOKIO_CONSOLE_FILTERS.parse()?)
        .add_directive(RUNTIME_CONSOLE_FILTERS.parse()?);

    // Enable support for capturing span traces when errors occur, used for
    // error reports with the `color-eyre` crate.
    let span_errors = ErrorLayer::default();

    // Enables Tokio Console support for debugging, filters out TRACE level
    // events by default in order avoid burying application logs in Tokio TRACE
    // events.
    let tokio_console = console_subscriber::spawn();
    let tokio_filter = filter_fn(|metadata| metadata.level() != &Level::TRACE);
    let log_format = tracing_subscriber::fmt::layer().with_filter(tokio_filter);

    Ok(tracing_subscriber::registry()
        .with(tokio_console)
        .with(env_filter)
        .with(log_format)
        .with(span_errors))
}

/// Initialises [`LogTracer`] to capture log events with [`tracing`], and sets
/// the given subscriber as the global default subscriber for structured logging
/// events. Also enables [`color_eyre`] error and panic handling hooks for
/// developer happiness.
///
/// Calling this twice will result in a code panic.
pub fn set_global_logger(subscriber: impl Subscriber + Send + Sync) -> Result<()> {
    color_eyre::install()?;
    LogTracer::init()?;
    set_global_default(subscriber)?;

    Ok(())
}

pub fn http_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
