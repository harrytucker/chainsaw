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
//! Chainsaw is split into a few different crates to allow you to pick and
//! choose which bits of functionality you would like for your service:
//!
//! - Chainsaw Middleware
//! - Chainsaw Observe
//! - Chainsaw Proto
//!
//! ## Major Libraries
//!
//! As it currently stands, Chainsaw is more like a microservice 'distribution'
//! for Rust than a set of abstractions atop other libraries. This could change
//! in the future, but for now you may find it useful to know some of the
//! primary libraries in use by Chainsaw when using the library, particularly if
//! you need to search for any example code or documentation for these.
//!
//! - Tokio   - An asynchronous runtime for the Rust programming language.
//! - Tonic   - A gRPC client and server library built on top of the Tokio
//!           stack.
//! - Tracing - A highly-configurable structured logging library.
//! - Serde   - A (fantastic) serialisation library for Rust.

pub mod config;

// Chainsaw re-exports types from other libraries that may be used across many
// modules for ease-of-importing.

/// Generic result type from the [`color_eyre`] library.
pub use color_eyre::eyre::Result;
