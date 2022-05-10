//! Wrapper for [`reqwest::Client`] which retries failed requests.

extern crate reqwest;

use std::{borrow::Borrow, cmp::min, fmt::Debug, future::Future, thread, time::Duration};

use log::{debug, error, trace, warn};
use rand::{thread_rng, Rng};
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::types::{Error, ErrorKind, Result};

/// Maximum number of retries when a service call fails.
const MAX_ATTEMPTS: u32 = 10;

/// Maximum backoff delay when retrying a service call.
const MAX_BACKOFF: u64 = 1_000;

/// Wrapper for [`reqwest::Client`] which retries failed requests.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::{
///     advertisement::{AdvertisementDto, AdvertisementsDto},
///     client::Client
/// };
///
/// let client: Client = Client::default();
/// let advertisements: Vec<AdvertisementDto> = client
///     .get::<AdvertisementsDto, ()>(ADVERTISEMENT_ENDPOINT, None)
///     .await?
///     .advertisements;
/// ```
#[derive(Default)]
pub struct Client {
    /// Client.
    client: reqwest::Client,
}

impl Client {
    /// Create new [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Make a GET request with exponential backoff and retries on request failures.
    ///
    /// Returns the result of calling GET `endpoint`, retrying with exponential backoff on transient
    /// errors.
    ///
    /// # Returns
    ///
    /// - **200 - OK:**                     `Ok(response)`
    /// - **400 - Bad Request:**            `Err(`[`ErrorKind::Permanent`]`)`
    /// - **500 - Internal Server Error:**  `Err(`[`ErrorKind::Transient`]`)`
    /// - **Everything else:**              `Err(`[`ErrorKind::Permanent`]`)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::repository::{
    ///     advertisement::{AdvertisementDto, AdvertisementsDto},
    ///     client::Client
    /// };
    ///
    /// let client: Client = Client::default();
    /// let advertisements: Vec<AdvertisementDto> = client
    ///     .get::<AdvertisementsDto, ()>(ADVERTISEMENT_ENDPOINT, None)
    ///     .await?
    ///     .advertisements;
    /// ```
    pub async fn get<T, Q>(&self, endpoint: &str, query: Option<Q>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        Q: Debug + Serialize,
    {
        trace!("Getting {}?{:#?}", endpoint, query);

        let op = || async {
            let mut request_builder: RequestBuilder = self.client.get(endpoint);

            if query.is_some() {
                request_builder = request_builder.query(query.borrow());
            }

            debug!("Making GET request {:#?}", request_builder);

            let response: Response = Client::send(request_builder).await?;

            match response.json::<T>().await {
                Ok(result) => Ok(result),
                Err(err) => Err(Error {
                    kind: ErrorKind::Permanent,
                    message: err.to_string(),
                }),
            }
        };

        Client::retry(op).await
    }

    /// Get backoff/delay to wait before the next retry attempt.
    ///
    /// Calculates exponential backoff based on the attempt number using the function:
    /// `min(2^(attempts - 1) + random_number_millis, MAX_BACKOFF)`.
    fn get_backoff(attempt: u32) -> u64 {
        const BASE: u64 = 2;
        let exponential_backoff: u64 = BASE.pow(attempt - 1);
        let random_number_millis: u64 = thread_rng().gen_range(0..100);
        let backoff: u64 = exponential_backoff + random_number_millis;

        min(backoff, MAX_BACKOFF)
    }

    /// Retry an operation with exponential backoff.
    ///
    /// Takes an operation which returns [`Result`][1]<T, [`Error`][2]>. If the operations returns [Ok]
    /// then this function returns the same value. If the operation returns [Err] of
    /// [`ErrorKind::Permanent`] then the error is returned. However if the operation returns [Err] of
    /// [`ErrorKind::Transient`] then the operation is retried up to [`MAX_ATTEMPTS`] times.
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

            let backoff: u64 = Client::get_backoff(i);
            thread::sleep(Duration::from_millis(backoff));
        }

        return f().await;
    }

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
    async fn send(request_builder: RequestBuilder) -> Result<Response> {
        match request_builder.send().await {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    Ok(response)
                } else if response.status() == StatusCode::NOT_FOUND {
                    Err(Error {
                        kind: ErrorKind::Permanent,
                        message: "Resource not found".to_string(),
                    })
                } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    Err(Error {
                        kind: ErrorKind::Transient,
                        message: "Internal server error".to_string(),
                    })
                } else {
                    Err(Error {
                        kind: ErrorKind::Permanent,
                        message: "Unexpected error".to_string(),
                    })
                }
            }
            Err(err) => Err(Error {
                kind: ErrorKind::Permanent,
                message: err.to_string(),
            }),
        }
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::types::Result;

    use super::Client;

    #[derive(Deserialize)]
    struct CatFact {
        fact: String,
        length: usize,
    }

    #[tokio::test]
    async fn test_get() {
        // Given
        let client = Client::new();
        let endpoint: &str = "https://catfact.ninja/fact";

        // When
        let result: Result<CatFact> = client
            .get::<CatFact, [(&str, usize); 1]>(endpoint, Some([("max_length", 140)]))
            .await;

        // Then
        assert!(result.is_ok(), "Result should be Ok");
    }
}
