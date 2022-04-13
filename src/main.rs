//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

use log::info;
use std::fmt::Error;

mod controller;
mod repository;
mod service;
mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match log4rs::init_file("log4rs.yaml", Default::default()) {
        Ok(_) => info!("Logger initialized"),
        Err(e) => eprintln!("Logger initialization failed: {}", e),
    };

    info!("Hello world!");

    Ok(())
}
