//! Repository layer for making requests to dependencies.

extern crate reqwest;

use std::borrow::Borrow;
use std::cmp::min;
use std::fmt::Debug;
use std::future::Future;
use std::thread;
use std::time::Duration;

use log::{debug, error, trace, warn};
use rand::{thread_rng, Rng};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::types::{Error, ErrorKind, Result};

pub mod advertisement;
pub mod image;
pub mod types;
pub mod video;

/// Maximum number of retries when a service call fails.
const MAX_ATTEMPTS: u32 = 10;

/// Maximum backoff delay when retrying a service call.
const MAX_BACKOFF: u64 = 1_000;

/// Make a GET request.
///
/// Makes a GET request based on the provided request builder and checks the response status code.
///
/// # Returns
///
/// - **200 - OK:** `Ok(response)`
/// - **400 - Bad Request:** `Err(ErrorKind::Permanent)`
/// - **500 - Internal Server Error:** `Err(ErrorKind::Transient)`
/// - **Everything else** - `Err(ErrorKind::Permanent)`
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
///
/// let client = Client::new();
/// let mut request_builder: RequestBuilder = client.get("https://example.com".to_string());
//  let response: Response = get(request_builder).await?;
/// ```
async fn get(request_builder: RequestBuilder) -> Result<Response> {
    match request_builder.send().await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                Ok(response)
            } else if response.status() == StatusCode::NOT_FOUND {
                Err(Error::new(ErrorKind::Permanent, "Resource not found"))
            } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                Err(Error::new(ErrorKind::Transient, "Internal server error"))
            } else {
                Err(Error::new(ErrorKind::Permanent, "Unexpected error"))
            }
        }
        Err(err) => return Err(Error::new(ErrorKind::Permanent, &err.to_string())),
    }
}

/// Get backoff/delay to wait before the next retry attempt.
///
/// Calculates exponential backoff based on the attempt number using the function:
/// `min(2^(attempts - 1) + random_number_millis, MAX_BACKOFF)`.
///
/// # Examples
///
/// ```rust
/// let backoff: u64 = get_backoff(i);
/// thread::sleep(Duration::from_millis(backoff));
/// ```
fn get_backoff(attempt: u32) -> u64 {
    const BASE: u64 = 2;
    let exponential_backoff: u64 = BASE.pow(attempt - 1);
    let random_number_millis: u64 = thread_rng().gen_range(0..100);
    let backoff: u64 = exponential_backoff + random_number_millis;

    min(backoff, MAX_BACKOFF)
}

/// Make a GET request with exponential backoff and retries on request failures.
///
/// Returns the result of calling GET `endpoint`, retrying with exponential backoff on transient
/// errors.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize)]
/// pub struct Advertisement {
///     container_id: u64,
///     id: u64,
///     name: String,
///     url: String,
/// }
///
/// #[derive(Deserialize, Serialize)]
/// struct Advertisements {
///     advertisements: Vec<Advertisement>,
/// }
///
/// pub async fn get_advertisement(advertisement_id: u32) -> Result<Advertisement> {
///     let client = Client::new();
///
///     request::<Advertisement, ()>(
///         &client,
///         format!(
///              "{}/{}",
///              "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///              advertisement_id,
///         ).as_str(),
///         None,
///     )
///     .await
/// }
///
/// pub fn list_advertisements() -> Result<Vec<Advertisement>> {
///     let client = Client::new();
///     let container_id = 0;
///
///     let advertisements: Vec<Advertisement> =
///         request::<Advertisement, Advertisements, [(&str, u32); 1]>(
///             &client,
///             "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///             Some([("containerId", container_id)]),
///         )
///         .await?
///         .advertisements;
///
///    Ok(advertisements)
/// }
/// ```
async fn request<T, Q>(client: &Client, endpoint: &str, query: Option<Q>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
    Q: Debug + Serialize,
{
    trace!("Getting {}?{:#?}", endpoint, query);

    let op = || async {
        let mut request_builder: RequestBuilder = client.get(endpoint);

        if query.is_some() {
            request_builder = request_builder.query(query.borrow());
        }

        debug!("Making GET request {:#?}", request_builder);

        let response: Response = get(request_builder).await?;

        match response.json::<T>().await {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::new(ErrorKind::Permanent, &err.to_string())),
        }
    };

    retry(op).await
}

/// Retry an operation with exponential backoff.
///
/// Takes an operation which returns [`Result`][1]<T, [`Error`][2]>. If the operations returns [Ok]
/// then this function returns the same value. If the operation returns [Err] of
/// [`ErrorKind::Permanent`] then the error is returned. However if the operation returns [Err] of
/// [`ErrorKind::Transient`] then the operation is retried up to [`MAX_ATTEMPTS`] times.
///  
/// # Examples
///
/// ```rust
/// let op = || async {
/// let mut request_builder: RequestBuilder = client.get(endpoint);
///
/// if query.is_some() {
///     request_builder = request_builder.query(query.borrow());
/// }
///
/// let response: Response = get(request_builder).await?;
///
/// match response.json::<T>().await {
///     Ok(result) => Ok(result),
///         Err(err) => Err(Error::new(ErrorKind::Permanent, &err.to_string())),
///     }
/// };
///
/// retry(op).await
/// ```
///
/// [1]: crate::types::Result
/// [2]: crate::types::Error
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
                    error!("Attempt #{} returned with un-retryable error {}", i, err);

                    return Err(err);
                } else {
                    warn!("Attempt #{} returned with retryable error {}", i, err);
                }
            }
        }

        let backoff: u64 = get_backoff(i);
        thread::sleep(Duration::from_millis(backoff));
    }

    return f().await;
}
