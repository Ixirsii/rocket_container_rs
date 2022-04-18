//! Advertisement service.

use log::trace;
use reqwest::Client;

use crate::repository::advertisement;
use crate::service::group;
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::types::Result;

/// List all advertisements from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::advertisement::list_advertisements;
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
pub async fn list_advertisements(client: &Client) -> Result<AdvertisementMap> {
    trace!("Listing all advertisements");

    let advertisements: Vec<(u32, Advertisement)> = advertisement::list_advertisements(client)
        .await?
        .into_iter()
        .map(|advertisement| {
            (
                advertisement.container_id().parse().unwrap(),
                Advertisement::from(advertisement),
            )
        })
        .collect();

    Ok(group(advertisements.into_iter()))
}

/// List advertisements for a container from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::advertisement::list_advertisements_by_container;
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
) -> Result<AdvertisementMap> {
    trace!("Listing advertisements by container id {}", container_id);

    let advertisements: Vec<(u32, Advertisement)> =
        advertisement::list_advertisements_by_container(client, container_id)
            .await?
            .into_iter()
            .map(|advertisement| {
                (
                    advertisement.container_id().parse().unwrap(),
                    Advertisement::from(advertisement),
                )
            })
            .collect();

    Ok(group(advertisements.into_iter()))
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::service::types::advertisement::AdvertisementMap;
    use crate::types::Result;

    use super::{list_advertisements, list_advertisements_by_container};

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<AdvertisementMap> = list_advertisements(&client).await;

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
        let result: Result<AdvertisementMap> =
            list_advertisements_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }
}
