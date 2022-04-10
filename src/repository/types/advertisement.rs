//! Advertisement data transfer object type definitions.

use serde::{Deserialize, Serialize};

use crate::controller::types::Advertisement;

use super::Wrapper;

/* ************************************** AdvertisementDto ************************************** */

/// Advertisement data returned from Rocket Advertisement service.
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
    /// Constructs a new Advertisement.
    pub fn new(container_id: String, id: String, name: String, url: String) -> Self {
        AdvertisementDto {
            container_id,
            id,
            name,
            url,
        }
    }
}

impl From<AdvertisementDto> for Advertisement {
    fn from(advertisement_dto: AdvertisementDto) -> Self {
        Advertisement::new(
            advertisement_dto.id.parse().unwrap(),
            advertisement_dto.name,
            advertisement_dto.url,
        )
    }
}

/* ************************************* AdvertisementsDto ************************************** */

/// [Wrapper] for [Advertisement]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdvertisementsDto {
    advertisements: Vec<AdvertisementDto>,
}

impl AdvertisementsDto {
    /// Construct a new Advertisements wrapper.
    pub fn new(advertisements: Vec<AdvertisementDto>) -> Self {
        AdvertisementsDto { advertisements }
    }
}

impl Wrapper<AdvertisementDto> for AdvertisementsDto {
    /// Unwrap [Advertisements::advertisements].
    fn unwrap(self) -> Vec<AdvertisementDto> {
        self.advertisements
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }
}
