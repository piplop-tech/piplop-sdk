//! Piplop SDK
//!
//! Library and CLI for registering video content as IP on Story Protocol.

pub mod schema;
pub mod error;
pub mod client;

pub use schema::*;
pub use error::SdkError;
pub use client::PiplopClient;
