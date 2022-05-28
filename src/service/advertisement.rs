//! Advertisement service.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::repository::advertisement::AdvertisementRepository;
use crate::service::group;
use crate::types::Result;

/* *************************************** Advertisement **************************************** */

/// Advertisement asset returned from Rocket Container.
///
/// Container service returns a variant of [`AdvertisementDto`][1] with `id` field as a number and
/// without `container_id` field. [`AdvertisementDto`][1]s returned from
/// [`advertisement repository`][2] get converted into this type before being returned from the
/// controller.
///
/// [1]: [crate::repository::types::advertisement::AdvertisementDto]
/// [2]: [crate::repository::advertisement]
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisement {
    /// Unique advertisement identifier.
    id: u32,
    /// Name of advertisement.
    name: String,
    /// Advertisement playback url.
    url: String,
}

impl Advertisement {
    /// Construct a new Advertisement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(id: u32, name: String, url: String) -> Self {
        Advertisement { id, name, url }
    }
}

impl Display for Advertisement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Advertisement {{ id: {}, name: {}, url: {} }}",
            self.id, self.name, self.url
        )
    }
}

/* ************************************** AdvertisementMap ************************************** */

/// Type alias for a [`HashMap`] of [`u32`] to [`Vec`]`<`[`Advertisement`]`>`.
pub type AdvertisementMap = HashMap<u32, Vec<Advertisement>>;

/* ************************************ AdvertisementService ************************************ */

/// Advertisement service.
///
/// [`AdvertisementService`] is the service layer wrapper for [`AdvertisementRepository`]. It
/// transforms DTO types into domain types.
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
    use crate::types::Result;

    use super::{Advertisement, AdvertisementMap, AdvertisementService};

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
