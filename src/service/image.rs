//! Advertisement service.

use log::trace;

use crate::repository::image::ImageRepository;
use crate::service::group;
use crate::service::types::image::{Image, ImageMap};
use crate::types::Result;

/// Image service.
///
/// [`ImageService`] is the service layer wrapper for [`ImageRepository`]. It transforms
/// DTO types into domain types.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct ImageService {
    /// Repository layer that the service calls.
    repository: ImageRepository,
}

impl ImageService {
    /// Create a new [`ImageService`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(repository: ImageRepository) -> Self {
        Self { repository }
    }

    /// List all images from Rocket Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_images(&self) -> Result<ImageMap> {
        trace!("ImageService::list_images");

        let images = self
            .repository
            .list_images()
            .await?
            .into_iter()
            .map(|image| (image.container_id().parse().unwrap(), Image::from(image)));

        Ok(group(images))
    }

    /// List images for a container from Rocket Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_images_by_container(&self, container_id: u32) -> Result<Vec<Image>> {
        trace!("ImageService::list_images_by_container {}", container_id);

        let images: Vec<Image> = self
            .repository
            .list_images_by_container(container_id)
            .await?
            .into_iter()
            .map(Image::from)
            .collect();

        Ok(images)
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{
        service::types::image::{Image, ImageMap},
        types::Result,
    };

    use super::ImageService;

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let service = ImageService::default();

        // When
        let result: Result<ImageMap> = service.list_images().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let service = ImageService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = service.list_images_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error: {}", err),
        }
    }
}
