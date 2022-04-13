//! Advertisement repository.

use log::trace;
use reqwest::Client;

use crate::types::Result;

use crate::repository::request;
use crate::repository::types::advertisement::{AdvertisementDto, AdvertisementsDto};

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
/// use rocket_stream::repository::advertisement::list_advertisements;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///
///     match list_advertisements(&client) {
///         Ok(advertisements) => println!("Got advertisements: {}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_advertisements(client: &Client) -> Result<Vec<AdvertisementDto>> {
    trace!("Listing all advertisements");

    let advertisements: Vec<AdvertisementDto> =
        request::<AdvertisementsDto, ()>(client, ADVERTISEMENT_ENDPOINT, None)
            .await?
            .advertisements;

    Ok(advertisements)
}

/// List advertisements for a container from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::repository::advertisement::list_advertisements_by_container;
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

    let advertisements: Vec<AdvertisementDto> = request::<AdvertisementsDto, [(&str, u32); 1]>(
        client,
        ADVERTISEMENT_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await?
    .advertisements;

    Ok(advertisements)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::repository::types::advertisement::AdvertisementDto;
    use crate::types::Result;

    use super::{list_advertisements, list_advertisements_by_container};

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<AdvertisementDto>> = list_advertisements(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {}", err),
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
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }
}
