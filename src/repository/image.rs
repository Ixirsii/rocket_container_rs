//! Image repository.

use log::trace;
use std::sync::Arc;

use crate::repository::client::Client;
use crate::repository::types::image::{ImageDto, ImagesDto};
use crate::types::Result;

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Image service.
const IMAGE_ENDPOINT: &str = "http://images.rocket-stream.bottlerocketservices.com/images";

/// Image repository.
///
/// [`ImageRepository`] is the repository layer which fetches images from Rocket Image service.
///
/// # Examples
///
/// ```rust
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
    /// ```
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
    use crate::repository::types::image::ImageDto;
    use crate::types::Result;

    use super::ImageRepository;

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
