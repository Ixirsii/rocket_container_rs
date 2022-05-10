//! Repository layer for making requests to dependencies.
//!
//! The repository layer is responsible for making requests to dependencies. It handles call
//! failures and retries, but does not handle any data transformation or processing.

pub mod advertisement;
pub mod client;
pub mod image;
pub mod video;
