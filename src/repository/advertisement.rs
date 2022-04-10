//! Advertisement repository.

use super::get_wrapped_list;
use super::types::advertisement::{AdvertisementDto, AdvertisementsDto};
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
///     match list_advertisements(&client) {
///         Ok(advertisements) => println!("Got advertisements: {:#?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_advertisements(client: &Client) -> Result<Vec<AdvertisementDto>> {
    trace!("Listing all advertisements");

    get_wrapped_list::<AdvertisementDto, AdvertisementsDto, ()>(
        client,
        ADVERTISEMENT_ENDPOINT,
        None,
    )
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
///     match list_advertisements_by_container(&client, container_id) {
///         Ok(advertisements) => println!("Got advertisements: {:$?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_advertisements_by_container(
    client: &Client,
    container_id: u32,
) -> Result<Vec<AdvertisementDto>> {
    trace!("Listing advertisements for container {}", container_id);

    get_wrapped_list::<AdvertisementDto, AdvertisementsDto, [(&str, u32); 1]>(
        client,
        ADVERTISEMENT_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{list_advertisements, list_advertisements_by_container};
    use crate::repository::types::advertisement::AdvertisementDto;
    use crate::types::Result;
    use reqwest::Client;

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<AdvertisementDto>> = list_advertisements(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_advertisements_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<AdvertisementDto>> =
            list_advertisements_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }
}
