//! Advertisement service.

use log::trace;

use crate::repository::advertisement::AdvertisementRepository;
use crate::service::group;
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::types::Result;

/// Advertisement service.
///
/// [`AdvertisementService`] is the service layer wrapper for [`AdvertisementRepository`]. It
/// transforms DTO types into domain types.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct AdvertisementService {
    /// Repository layer that the service calls.
    repository: AdvertisementRepository,
}

impl AdvertisementService {
    /// Create a new [`AdvertisementService`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(repository: AdvertisementRepository) -> Self {
        Self { repository }
    }

    /// List all advertisements from Rocket Advertisement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_advertisements(&self) -> Result<AdvertisementMap> {
        trace!("AdvertisementService::list_advertisements");

        let advertisements = self
            .repository
            .list_advertisements()
            .await?
            .into_iter()
            .map(|advertisement| {
                (
                    advertisement.container_id().parse().unwrap(),
                    Advertisement::from(advertisement),
                )
            });

        Ok(group(advertisements))
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
    ) -> Result<Vec<Advertisement>> {
        trace!(
            "AdvertisementService::list_advertisements_by_container {}",
            container_id
        );

        let advertisements: Vec<Advertisement> = self
            .repository
            .list_advertisements_by_container(container_id)
            .await?
            .into_iter()
            .map(Advertisement::from)
            .collect();

        Ok(advertisements)
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{
        repository::client::Client,
        service::types::advertisement::{Advertisement, AdvertisementMap},
        types::Result,
    };

    use super::AdvertisementService;

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let service = AdvertisementService::default();

        // When
        let result: Result<AdvertisementMap> = service.list_advertisements().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_advertisements_by_container() {
        // Given
        let service = AdvertisementService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Advertisement>> =
            service.list_advertisements_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }
}
