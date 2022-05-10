//! Repository responsible for calling the Rocket Advertisement dependency and handling failures.
//!
//! Rocket Container's dependencies (Rocket Advertisement, Rocket Image, and Rocket Video) all
//! return lists wrapped in an object. The only "data transformation" that happens at this layer
//! is that the lists are unwrapped and returned directly.

use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use log::{debug, trace};
use serde::{Deserialize, Serialize};

use crate::{
    repository::client::Client,
    service::advertisement::Advertisement,
    types::{array_to_string, Result},
};

/// Endpoint for Rocket Advertisement service.
const ADVERTISEMENT_ENDPOINT: &str =
    "http://ads.rocket-stream.bottlerocketservices.com/advertisements";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/* ************************************** AdvertisementDto ************************************** */

/// Advertisement data returned from Rocket Advertisement service.
///
/// [`AdvertisementDto`]s are meant to be deserialized from network calls and not constructed
/// directly.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::advertisement::{AdvertisementDto, AdvertisementRepository};
///
/// let repository: AdvertisementRepository = AdvertisementRepository::default();
/// let advertisements: Vec<AdvertisementDto> = repository.list_advertisements().await?;
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvertisementDto {
    /// Parent container e.g. show/series identifier.
    container_id: String,
    /// Unique advertisement identifier.
    id: String,
    /// Name of advertisement.
    name: String,
    /// Advertisement playback url.
    url: String,
}

impl AdvertisementDto {
    /// Get container ID.
    pub fn container_id(&self) -> &str {
        &self.container_id
    }
}

impl From<AdvertisementDto> for Advertisement {
    /// Get an [`Advertisement`] from an [`AdvertisementDto`].
    fn from(advertisement_dto: AdvertisementDto) -> Self {
        Advertisement::new(
            advertisement_dto.id.parse().unwrap(),
            advertisement_dto.name,
            advertisement_dto.url,
        )
    }
}

impl Display for AdvertisementDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AdvertisementDto {{ container_id: {}, id: {}, name: {}, url: {} }}",
            self.container_id, self.id, self.name, self.url
        )
    }
}

/* ************************************* AdvertisementsDto ************************************** */

/// Wrapped advertisement data returned from Rocket Advertisement service.
///
/// [`AdvertisementsDto`]s are meant to be deserialized from network calls and not constructed
/// directly.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdvertisementsDto {
    /// List of advertisements.
    pub advertisements: Vec<AdvertisementDto>,
}

impl Display for AdvertisementsDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AdvertisementsDto {{ advertisements: {} }}",
            array_to_string(&self.advertisements)
        )
    }
}

/* ********************************** AdvertisementRepository *********************************** */

/// Advertisement repository.
///
/// [`AdvertisementRepository`] is the repository layer which fetches advertisements from Rocket
/// Advertisement service.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::advertisement::{AdvertisementDto, AdvertisementRepository};
///
/// let repository: AdvertisementRepository = AdvertisementRepository::default();
/// let advertisements: Vec<AdvertisementDto> = repository.list_advertisements().await?;
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
    /// use rocket_container::repository::advertisement::{AdvertisementDto, AdvertisementRepository};
    ///
    /// let repository: AdvertisementRepository = AdvertisementRepository::default();
    /// let advertisements: Vec<AdvertisementDto> = repository.list_advertisements().await?;
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
    /// use rocket_container::repository::advertisement::{AdvertisementDto, AdvertisementRepository};
    ///
    /// let container_id: u32 = 1;
    /// let repository: AdvertisementRepository = AdvertisementRepository::default();
    /// let advertisements: Vec<AdvertisementDto> = repository
    ///     .list_advertisements_by_container(container_id)
    ///     .await?;
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
    use crate::types::Result;

    use super::{AdvertisementDto, AdvertisementRepository, AdvertisementsDto};

    #[test]
    fn deserialize_advertisement() {
        // Given
        let data: &str = r#"
            {
                "containerId": "0",
                "id": "0",
                "name": "Advertisement",
                "url": "https://advertisement.com"
            }
        "#;

        let expected: AdvertisementDto = AdvertisementDto {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Advertisement".to_string(),
            url: "https://advertisement.com".to_string(),
        };

        // When
        let result: serde_json::Result<AdvertisementDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_advertisements() {
        // Given
        let data: &str = r#"
            {
                "advertisements": [
                    {
                        "containerId": "0",
                        "id": "0",
                        "name": "Advertisement",
                        "url": "https://advertisement.com"
                    }
                ]
            }
        "#;

        let expected: AdvertisementsDto = AdvertisementsDto {
            advertisements: Vec::from([AdvertisementDto {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Advertisement".to_string(),
                url: "https://advertisement.com".to_string(),
            }]),
        };

        // When
        let result: serde_json::Result<AdvertisementsDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_advertisement() {
        // Given
        let data: AdvertisementDto = AdvertisementDto {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Advertisement".to_string(),
            url: "https://advertisement.com".to_string(),
        };

        let expected: &str = "\
            {\
                \"containerId\":\"0\",\
                \"id\":\"0\",\
                \"name\":\"Advertisement\",\
                \"url\":\"https://advertisement.com\"\
            }\
        ";

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_advertisements() {
        // Given
        let data: AdvertisementsDto = AdvertisementsDto {
            advertisements: Vec::from([AdvertisementDto {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Advertisement".to_string(),
                url: "https://advertisement.com".to_string(),
            }]),
        };

        let expected: &str = "\
            {\
                \"advertisements\":[\
                    {\
                        \"containerId\":\"0\",\
                        \"id\":\"0\",\
                        \"name\":\"Advertisement\",\
                        \"url\":\"https://advertisement.com\"\
                    }\
                ]\
            }\
        ";

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

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
