//! Image data transfer object type definitions.

use serde::{Deserialize, Serialize};

use crate::controller::types::Image;

/* ****************************************** ImageDto ****************************************** */

/// Image data returned from Rocket Image service.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use rocket_stream::repository::image::list_images;
/// use rocket_stream::repository::types::image::ImageDto;
///
/// let client: Client = Client::new();
//  let images: Vec<ImageDto> = list_images(&client).await.unwrap();
/// ```
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

impl From<ImageDto> for Image {
    /// Get an [`Image`][1] from an [`ImageDto`][2].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_stream::controller::types::Image;
    ///
    /// let images: Vec<Image> = list_images(&client)
    ///     .await?
    ///     .into_iter()
    ///     .map(Image::from)
    ///     .collect();;
    /// ```
    ///
    /// [1]: [crate::types::Image]
    /// [2]: [crate::repository::types::image::ImageDto]
    ///
    fn from(image_dto: ImageDto) -> Self {
        Image::new(image_dto.id.parse().unwrap(), image_dto.name, image_dto.url)
    }
}

/* ***************************************** ImagesDto ****************************************** */

/// [Wrapper] for [Image]s.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use rocket_stream::repository::types::image::{ImageDto, ImagesDto};
///
/// let advertisements: Vec<AdvertisementDto> =
///     request::<AdvertisementsDto, ()>(client, ADVERTISEMENT_ENDPOINT, None)
///         .await?
///         .images;
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImagesDto {
    pub images: Vec<ImageDto>,
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
            Err(err) => panic!("Failed to deserialize with error: {}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }
}
