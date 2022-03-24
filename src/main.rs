//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

use std::collections::HashMap;

mod repository;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
