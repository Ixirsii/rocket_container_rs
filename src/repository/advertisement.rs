//! Advertisement repository.

use std::sync::Arc;

use log::{debug, trace};

use crate::repository::client::Client;
use crate::repository::types::advertisement::{AdvertisementDto, AdvertisementsDto};
use crate::types::Result;

/// Endpoint for Rocket Advertisement service.
const ADVERTISEMENT_ENDPOINT: &str =
    "http://ads.rocket-stream.bottlerocketservices.com/advertisements";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Advertisement repository.
///
/// [`AdvertisementRepository`] is the repository layer which fetches advertisements from Rocket
/// Advertisement service.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct AdvertisementRepository {
    /// Client for making requests.
    client: Arc<Client>,
}

impl<'a> AdvertisementRepository {
    /// Create a new [`AdvertisementRepository`].
    pub fn new(client: Arc<Client>) -> Self {
        AdvertisementRepository { client }
    }

    /// List all advertisements from Rocket Advertisement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_advertisements(&self) -> Result<Vec<AdvertisementDto>> {
        trace!("AdvertisementRepository::list_advertisements");

        let advertisements: Vec<AdvertisementDto> = self
            .client
            .get::<AdvertisementsDto, ()>(ADVERTISEMENT_ENDPOINT, None)
            .await?
            .advertisements;

        debug!("Advertisements: {:#?}", advertisements);

        Ok(advertisements)
    }

    /// List advertisements for a container from Rocket Advertisement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_advertisements_by_container(
        &self,
        container_id: u32,
    ) -> Result<Vec<AdvertisementDto>> {
        trace!(
            "AdvertisementRepository::list_advertisements_by_container {}",
            container_id
        );

        let advertisements: Vec<AdvertisementDto> = self
            .client
            .get::<AdvertisementsDto, [(&str, u32); 1]>(
                ADVERTISEMENT_ENDPOINT,
                Some([(CONTAINER_ID, container_id)]),
            )
            .await?
            .advertisements;

        debug!("Advertisements: {:#?}", advertisements);

        Ok(advertisements)
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{repository::types::advertisement::AdvertisementDto, types::Result};

    use super::AdvertisementRepository;

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let repository = AdvertisementRepository::default();

        // When
        let result: Result<Vec<AdvertisementDto>> = repository.list_advertisements().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_advertisements_by_container() {
        // Given
        let repository = AdvertisementRepository::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<AdvertisementDto>> = repository
            .list_advertisements_by_container(container_id)
            .await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }
}
