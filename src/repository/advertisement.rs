//! Advertisement repository.

use crate::repository::get_wrapped_list;
use crate::repository::types::{Advertisement, Advertisements};
use crate::types::Result;
use log::trace;
use reqwest::Client;

/// Endpoint for Rocket Advertisement service.
const ADVERTISEMENT_ENDPOINT: &str =
    "http://ads.rocket-stream.bottlerocketservices.com/advertisements";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// List all advertisements from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use crate::temp::list_all_advertisements;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///
///     match list_all_advertisements(&client) {
///         Ok(advertisements) => println!("Got advertisements: {:#?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_all_advertisements(client: &Client) -> Result<Vec<Advertisement>> {
    trace!("Listing all advertisements");

    get_wrapped_list::<Advertisement, Advertisements, ()>(client, ADVERTISEMENT_ENDPOINT, None)
        .await
}

/// List advertisements for a container from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use crate::temp::list_advertisements;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let container_id = 0;
///
///     match list_advertisements(&client, container_id) {
///         Ok(advertisements) => println!("Got advertisements: {:$?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_advertisements(client: &Client, container_id: u32) -> Result<Vec<Advertisement>> {
    trace!("Listing advertisements for container {}", container_id);

    get_wrapped_list::<Advertisement, Advertisements, [(&str, u32); 1]>(
        client,
        ADVERTISEMENT_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{list_advertisements, list_all_advertisements};
    use crate::repository::types::Advertisement;
    use crate::types::Result;
    use reqwest::Client;

    #[tokio::test]
    async fn test_list_all_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<Advertisement>> = list_all_advertisements(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Advertisement>> = list_advertisements(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }
}
