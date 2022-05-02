//! Advertisement service.

use log::trace;

use crate::repository::image;
use crate::service::group;
use crate::service::types::image::{Image, ImageMap};
use crate::types::Result;

/// List all images from Rocket Image.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::image::list_images;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///
///     match list_images(&client) {
///         Ok(images) => println!("Got images: {}", images),
///         Err(_) => println!("Failed to get images"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_images() -> Result<ImageMap> {
    trace!("Listing all images");

    let images: Vec<(u32, Image)> = image::list_images()
        .await?
        .into_iter()
        .map(|image| (image.container_id().parse().unwrap(), Image::from(image)))
        .collect();

    Ok(group(images.into_iter()))
}

/// List images for a container from Rocket Image.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::image::list_images_by_container;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     match list_images_by_container() {
///         Ok(images) => println!("Got images: {}", images),
///         Err(_) => println!("Failed to get images"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_images_by_container(container_id: u32) -> Result<Vec<Image>> {
    trace!("Listing images by container id {}", container_id);

    let images: Vec<Image> = image::list_images_by_container(container_id)
        .await?
        .into_iter()
        .map(Image::from)
        .collect();

    Ok(images)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::service::types::image::{Image, ImageMap};
    use crate::types::Result;

    use super::{list_images, list_images_by_container};

    #[tokio::test]
    async fn test_list_images() {
        // When
        let result: Result<ImageMap> = list_images().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = list_images_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error: {}", err),
        }
    }
}
