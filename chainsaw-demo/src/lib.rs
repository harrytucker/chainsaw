//! An opinionated set of libraries and tools for creating speedy, fully-fuelled
//! microservices in Rust.
//!
//! # Chainsaw - Don't Miss the Forest for the Trees.
//!
//! Creating microservices is no easy task, there are many different facilities
//! you may want to integrate with: metrics, health checks, structured logging,
//! and more. The aim of this library is to provide a higher-level of
//! abstraction for writing speedy microservices in Rust, without needing to get
//! stuck into the weeds regarding library choices and tooling.
//!
//! ## Available Tooling
//!
//! Currently Chainsaw only provides tooling for writing gRPC-based
//! microservices. In the future, Chainsaw will also provide abstractions for
//! handling REST-based APIs, integrated into the same server if possible.
//!
//! ## Major Libraries
//!
//! As it currently stands, Chainsaw is more like a microservice 'distribution'
//! for Rust than a set of abstractions atop other libraries. This could change
//! in the future, but for now you may find it useful to know some of the
//! primary libraries in use by Chainsaw when using the library, particularly if
//! you need to search for any example code or documentation for these.
//!
//! - [Tokio](tokio) - An asynchronous runtime for the Rust programming language.
//! - [Tonic](tonic) - A gRPC client and server library built on top of the Tokio
//!                    stack.
//! - [Tracing](tracing) - A highly-configurable structured logging library.
//! - [Serde](serde) - A (fantastic) serialisation library for Rust.

pub mod health;
pub mod logging;

pub mod server {}

pub mod metrics {}

// Chainsaw re-exports types from other libraries that may be used across many
// modules for ease-of-importing.

/// Generic result type from the [`color_eyre`] library.
pub use color_eyre::eyre::Result;
