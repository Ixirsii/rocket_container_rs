//! Advertisement service.

use log::trace;
use reqwest::Client;

use crate::controller::types::Advertisement;
use crate::repository::advertisement;
use crate::repository::types::advertisement::AdvertisementDto;
use crate::types::Result;

pub async fn list_advertisements(client: &Client) -> Result<Vec<Advertisement>> {
    trace!("Listing all advertisements");

    let advertisement_dtos: Vec<AdvertisementDto> =
        advertisement::list_advertisements(client).await?;

    let advertisements: Vec<Advertisement> = advertisement_dtos
        .into_iter()
        .map(Advertisement::from)
        .collect();

    Ok(advertisements)
}

pub async fn list_advertisements_by_container(
    client: &Client,
    container_id: u32,
) -> Result<Vec<Advertisement>> {
    trace!("Listing advertisements by container id {}", container_id);

    let advertisement_dtos: Vec<AdvertisementDto> =
        advertisement::list_advertisements_by_container(client, container_id).await?;

    let advertisements: Vec<Advertisement> = advertisement_dtos
        .into_iter()
        .map(Advertisement::from)
        .collect();

    Ok(advertisements)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::controller::types::Advertisement;
    use crate::types::Result;

    use super::{list_advertisements, list_advertisements_by_container};

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<Advertisement>> = list_advertisements(&client).await;

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
        let result: Result<Vec<Advertisement>> =
            list_advertisements_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }
}
