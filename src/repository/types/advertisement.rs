//! Advertisement data transfer object type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::service::types::advertisement::Advertisement;
use crate::types::array_to_string;

/* ************************************** AdvertisementDto ************************************** */

/// Advertisement data returned from Rocket Advertisement service.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use rocket_container::repository::advertisement::list_advertisements;
/// use rocket_container::repository::types::advertisement::AdvertisementDto;
///
/// let client: Client = Client::new();
//  let advertisements: Vec<AdvertisementDto> = list_advertisements(&client).await.unwrap();
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
    /// Get an [`Advertisement`][1] from an [`AdvertisementDto`][2].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::controller::types::advertisement::Advertisement;
    /// use rocket_container::repository::advertisement::list_advertisements;
    ///
    /// let advertisements: Vec<Advertisement> = list_advertisements(&client)
    ///     .await?
    ///     .into_iter()
    ///     .map(Advertisement::from)
    ///     .collect();;
    /// ```
    ///
    /// [1]: [crate::types::Advertisement]
    /// [2]: [crate::repository::types::advertisement::AdvertisementDto]
    ///
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

/// [Wrapper] for [Advertisement]s.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use rocket_container::repository::request;
/// use rocket_container::repository::types::advertisement::{AdvertisementDto, AdvertisementsDto};
///
/// let advertisements: Vec<AdvertisementDto> =
///     request::<AdvertisementsDto, ()>(client, ADVERTISEMENT_ENDPOINT, None)
///         .await?
///         .advertisements;
/// ```
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

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{AdvertisementDto, AdvertisementsDto};

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
}
