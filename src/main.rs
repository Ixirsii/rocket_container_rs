//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

mod repository;
mod types;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello world!");

    Ok(())
}
