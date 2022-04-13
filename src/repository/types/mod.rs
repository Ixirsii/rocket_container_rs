//! Repository module type definitions.
//!
//! Container Service's dependencies return data with a container ID (containerId) field, and with
//! the asset ID field as a string. This module defines types which match the return type of the
//! dependencies to make (de)serialization easier, as well as to separate types from domain types
//! in keeping with onion architecture principles.

pub mod advertisement;
pub mod image;
pub mod video;
