//! Structured logging facilities.
//!
//! Structured logging gives us the ability to more easily programmatically
//! interact with logs produced by applications. The most widely used crate for
//! structured logging is the [`tracing`] crate, so this module provides some
//! opinionated configurations to allow you to use [`tracing`] in your
//! microservice.

mod logging;

pub use logging::{new_subscriber, set_global_logger};
