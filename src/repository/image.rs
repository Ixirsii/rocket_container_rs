//! Repository responsible for calling the Rocket Image dependency and handling failures.
//!
//! Rocket Container's dependencies (Rocket Advertisement, Rocket Image, and Rocket Video) all
//! return lists wrapped in an object. The only "data transformation" that happens at this layer
//! is that the lists are unwrapped and returned directly.
//!
//! # Types
//!
//! - [`ImageDto`]: Data Transfer Object for images returned from Rocket Image.
//! - [`ImagesDto`]: Rocket Image returns a list of images wrapped in an object. [`ImagesDto`]
//!   models the wrapper object and contains only a list of [`ImageDto`]s.
//! - [`ImageRepository`]: Wrapper around [`Client`] which calls Rocket Image service.

use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    repository::client::Client,
    service::image::Image,
    types::{array_to_string, Result},
};

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Image service.
const IMAGE_ENDPOINT: &str = "http://images.rocket-stream.bottlerocketservices.com/images";

/* ****************************************** ImageDto ****************************************** */

/// Image data returned from Rocket Image service.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::image::{ImageDto, ImageRepository};
///
/// let repository = ImageRepository::default();
/// let images: Vec<ImageDto> = repository.get_images().await?;
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

impl ImageDto {
    /// Get container ID.
    pub fn container_id(&self) -> &str {
        &self.container_id
    }
}

impl From<ImageDto> for Image {
    /// Get an [`Image`] from an [`ImageDto`].
    fn from(image_dto: ImageDto) -> Self {
        Image::new(image_dto.id.parse().unwrap(), image_dto.name, image_dto.url)
    }
}

impl Display for ImageDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ container_id: {}, id: {}, name: {}, url: {} }}",
            self.container_id, self.id, self.name, self.url
        )
    }
}

/* ***************************************** ImagesDto ****************************************** */

/// Wrapped image data returned from Rocket Image service.
///
/// [`ImagesDto`]s are meant to be deserialized from network calls and not constructed directly.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImagesDto {
    /// List of images.
    pub images: Vec<ImageDto>,
}

impl Display for ImagesDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ images: {} }}", array_to_string(&self.images))
    }
}

/* ************************************** ImageRepository *************************************** */

/// Image repository.
///
/// [`ImageRepository`] is the repository layer which fetches images from Rocket Image service.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::image::{ImageDto, ImageRepository};
///
/// let repository: ImageRepository = ImageRepository::default();
/// let images: Vec<ImageDto> = repository.list_images().await?;
/// ```
#[derive(Default)]
pub struct ImageRepository {
    /// Client for making requests.
    client: Arc<Client>,
}

impl ImageRepository {
    /// Create new [`ImageRepository`].
    pub fn new(client: Arc<Client>) -> Self {
        ImageRepository { client }
    }

    /// List all images from Rocket Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::repository::image::{ImageDto, ImageRepository};
    ///
    /// let repository: ImageRepository = ImageRepository::default();
    /// let images: Vec<ImageDto> = repository.list_images().await?;
    /// ```
    pub async fn list_images(&self) -> Result<Vec<ImageDto>> {
        trace!("Listing all images");

        let images: Vec<ImageDto> = self
            .client
            .get::<ImagesDto, ()>(IMAGE_ENDPOINT, None)
            .await?
            .images;

        Ok(images)
    }

    /// List images for a container from Rocket Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::repository::image::{ImageDto, ImageRepository};
    ///
    /// let container_id: u32 = 1;
    /// let repository: ImageRepository = ImageRepository::default();
    /// let images: Vec<ImageDto> = repository.list_images_by_container(container_id).await?;
    /// ```
    pub async fn list_images_by_container(&self, container_id: u32) -> Result<Vec<ImageDto>> {
        trace!("Listing images for container {}", container_id);

        let images: Vec<ImageDto> = self
            .client
            .get::<ImagesDto, [(&str, u32); 1]>(
                IMAGE_ENDPOINT,
                Some([(CONTAINER_ID, container_id)]),
            )
            .await?
            .images;

        Ok(images)
    }
}

#[cfg(test)]
mod test {
    use crate::types::Result;

    use super::{ImageDto, ImageRepository, ImagesDto};

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

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let repository = ImageRepository::default();

        // When
        let result: Result<Vec<ImageDto>> = repository.list_images().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all images with error {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let repository = ImageRepository::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<ImageDto>> = repository.list_images_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error {}", err),
        }
    }
}
