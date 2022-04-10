//! Image data transfer object type definitions.

use serde::{Deserialize, Serialize};

use crate::controller::types::Image;

use super::Wrapper;

/* ****************************************** ImageDto ****************************************** */

/// Image data returned from Rocket Image service.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageDto {
    /// Parent container e.g. show/series identifier.
    container_id: String,
    /// Unique image identifier.
    id: String,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

impl ImageDto {
    /// Construct a new Image.
    pub fn new(container_id: String, id: String, name: String, url: String) -> Self {
        ImageDto {
            container_id,
            id,
            name,
            url,
        }
    }
}

impl From<ImageDto> for Image {
    fn from(image_dto: ImageDto) -> Self {
        Image::new(image_dto.id.parse().unwrap(), image_dto.name, image_dto.url)
    }
}

/* ***************************************** ImagesDto ****************************************** */

/// [Wrapper] for [Image]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImagesDto {
    images: Vec<ImageDto>,
}

impl ImagesDto {
    /// Construct a new Images wrapper.
    pub fn new(images: Vec<ImageDto>) -> Self {
        ImagesDto { images }
    }
}

impl Wrapper<ImageDto> for ImagesDto {
    /// Unwrap [Images::images].
    fn unwrap(self) -> Vec<ImageDto> {
        self.images
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{ImageDto, ImagesDto};

    #[test]
    fn deserialize_image() {
        // Given
        let data: &str = r#"
            {
                "containerId": "0",
                "id": "0",
                "name": "Image",
                "url": "https://image.com"
            }
        "#;

        let expected: ImageDto = ImageDto {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Image".to_string(),
            url: "https://image.com".to_string(),
        };

        // When
        let result: serde_json::Result<ImageDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn deserialize_images() {
        // Given
        let data: &str = r#"
            {
                "images": [
                    {
                        "containerId": "0",
                        "id": "0",
                        "name": "Image",
                        "url": "https://image.com"
                    }
                ]
            }
        "#;

        let expected: ImagesDto = ImagesDto {
            images: Vec::from([ImageDto {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Image".to_string(),
                url: "https://image.com".to_string(),
            }]),
        };

        // When
        let result: serde_json::Result<ImagesDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn serialize_image() {
        // Given
        let data: ImageDto = ImageDto {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Image".to_string(),
            url: "https://image.com".to_string(),
        };

        let expected: &str =
            r#"{"containerId":"0","id":"0","name":"Image","url":"https://image.com"}"#;

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn serialize_images() {
        // Given
        let data: ImagesDto = ImagesDto {
            images: Vec::from([ImageDto {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Image".to_string(),
                url: "https://image.com".to_string(),
            }]),
        };

        let expected: &str =
            r#"{"images":[{"containerId":"0","id":"0","name":"Image","url":"https://image.com"}]}"#;

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }
}
