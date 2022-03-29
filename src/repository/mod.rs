//! Repository layer for making requests to dependencies.

mod advertisement;
mod image;
mod types;

extern crate reqwest;

use crate::types::{Error, ErrorKind, Result};
use log::{debug, error, trace, warn};
use rand::{thread_rng, Rng};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::min;
use std::fmt::Debug;
use std::future::Future;
use std::thread;
use std::time::Duration;
use types::Wrapper;

/// Maximum number of retries when a service call fails.
const MAX_ATTEMPTS: u32 = 10;

/// Maximum backoff delay when retrying a service call.
const MAX_BACKOFF: u64 = 1_000;

/// Make a GET request with exponential backoff and retries on request failures.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// pub struct Advertisement {
///     container_id: u64,
///     id: u64,
///     name: String,
///     url: String,
/// }
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct Advertisements {
///     advertisements: Vec<Advertisement>,
/// }
///
/// pub fn list_advertisements() -> Result<Vec<Advertisement>> {
///     let client = Client::new();
///
///     get::<Advertisement, Advertisements, ()>(
///         &client,
///         "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///         None,
///     )
/// }
///
/// pub fn list_advertisements() -> Result<Vec<Advertisement>> {
///     let client = Client::new();
///     let container_id = 0;
///
///     get::<Advertisement, Advertisements, [(&str, u32); 1]>(
///         &client,
///         "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///         Some([("containerId", container_id)]),
///     )
/// }
/// ```
async fn get<T, W, Q>(client: &Client, endpoint: &str, query: Option<Q>) -> Result<Vec<T>>
where
    W: Wrapper<T> + for<'de> Deserialize<'de>,
    Q: Debug + Serialize,
{
    trace!("Getting {} ? {:#?}", endpoint, query.as_ref().unwrap());

    let op = || async {
        let mut request_builder: RequestBuilder = client.get(endpoint);

        if query.is_some() {
            request_builder = request_builder.query(query.borrow());
        }

        debug!("Making GET request {:#?}", request_builder);

        let response: Response = match request_builder.send().await {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    response
                } else if response.status() == StatusCode::NOT_FOUND {
                    return Err(Error::new(ErrorKind::Permanent, "Resource not found"));
                } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(Error::new(ErrorKind::Transient, "Internal server error"));
                } else {
                    return Err(Error::new(ErrorKind::Permanent, "Unexpected error"));
                }
            }
            Err(err) => return Err(Error::new(ErrorKind::Permanent, &err.to_string())),
        };

        match response.json::<W>().await {
            Ok(result) => Ok(result.unwrap()),
            Err(err) => Err(Error::new(ErrorKind::Permanent, &err.to_string())),
        }
    };

    retry(op).await
}

/// Get backoff/delay to wait before the next retry attempt.
fn get_backoff(attempt: u32) -> u64 {
    const BASE: u64 = 2;
    let exponential_backoff: u64 = BASE.pow(attempt);
    let random_number_milliseconds: u64 = thread_rng().gen_range(0..100);
    let backoff: u64 = exponential_backoff + random_number_milliseconds;

    min(backoff, MAX_BACKOFF)
}

/// Calls a function and retries if the function fails.
async fn retry<I, F, Fut>(mut f: F) -> Result<I>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<I>>,
{
    for i in 1..MAX_ATTEMPTS {
        trace!("Attempt #{}", i);

        match f().await {
            Ok(data) => return Ok(data),
            Err(err) => {
                if err.kind == ErrorKind::Permanent {
                    error!("Attempt #{} returned with un-retryable error {:#?}", i, err);

                    return Err(err);
                } else {
                    warn!("Attempt #{} returned with retryable error {:#?}", i, err);
                }
            }
        }

        let backoff: u64 = get_backoff(i);
        thread::sleep(Duration::from_millis(backoff));
    }

    return f().await;
}
