//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

use log::info;

mod repository;
mod types;

#[tokio::main]
async fn main() -> Result<(), ()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Hello world!");

    Ok(())
}
