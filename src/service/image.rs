//! Advertisement service.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::{repository::image::ImageRepository, service::group, types::Result};

/* ******************************************* Image ******************************************** */

/// Image asset returned from Rocket Container.
///
/// Container service returns a variant of [`ImageDto`][1] with `id` field as a number and
/// without `container_id` field. [`ImageDto`][1]s returned from
/// [`ImageRepository`] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::image::{Image, ImageService};
///
/// let container_id: u32 = 1;
/// let service: ImageService = ImageService::default();
/// let containers: Vec<Image> = service.list_images_by_container(container_id).await?;
/// ```
///
/// [1]: [crate::repository::image::ImageDto]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Unique image identifier.
    id: u32,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

impl Image {
    /// Construct a new Image.
    pub fn new(id: u32, name: String, url: String) -> Self {
        Image { id, name, url }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image {{ id: {}, name: {}, url: {} }}",
            self.id, self.name, self.url
        )
    }
}

/* ****************************************** ImageMap ****************************************** */

/// Type alias for a [`HashMap`] of [`u32`] to [`Vec`]`<`[`Image`]`>`.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::image::{ImageMap, ImageService};
///
/// let service: ImageService = ImageService::default();
/// let containers: ImageMap = service.list_images().await?;
/// ```
pub type ImageMap = HashMap<u32, Vec<Image>>;

/* **************************************** ImageService **************************************** */

/// Image service.
///
/// [`ImageService`] is the service layer wrapper for [`ImageRepository`]. It transforms
/// DTO types into domain types.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::image::{ImageMap, ImageService};
///
/// let service: ImageService = ImageService::default();
/// let containers: ImageMap = service.list_images().await?;
/// ```
#[derive(Default)]
pub struct ImageService {
    /// Repository layer that the service calls.
    repository: ImageRepository,
}

impl ImageService {
    /// Create a new [`ImageService`].
    pub fn new(repository: ImageRepository) -> Self {
        Self { repository }
    }

    /// List all images from Rocket Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::service::image::{ImageMap, ImageService};
    ///
    /// let service: ImageService = ImageService::default();
    /// let containers: ImageMap = service.list_images().await?;
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
    /// use rocket_container::service::image::{Image, ImageService};
    ///
    /// let container_id: u32 = 1;
    /// let service: ImageService = ImageService::default();
    /// let containers: Vec<Image> = service.list_images_by_container(container_id).await?;
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
    use crate::types::Result;

    use super::{Image, ImageMap, ImageService};

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
