//! Image repository.

use log::trace;

use crate::repository::request;
use crate::repository::types::image::{ImageDto, ImagesDto};
use crate::types::Result;

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Image service.
const IMAGE_ENDPOINT: &str = "http://images.rocket-stream.bottlerocketservices.com/images";

/// List all images from Rocket Image.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::repository::image::list_images;
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
pub async fn list_images() -> Result<Vec<ImageDto>> {
    trace!("Listing all images");

    let images: Vec<ImageDto> = request::<ImagesDto, ()>(IMAGE_ENDPOINT, None).await?.images;

    Ok(images)
}

/// List images for a container from Rocket Image.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::repository::image::list_images_by_container;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///
///     match list_images_by_container(&client) {
///         Ok(images) => println!("Got images: {}", images),
///         Err(_) => println!("Failed to get images"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_images_by_container(container_id: u32) -> Result<Vec<ImageDto>> {
    trace!("Listing images for container {}", container_id);

    let images: Vec<ImageDto> = request::<ImagesDto, [(&str, u32); 1]>(
        IMAGE_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await?
    .images;

    Ok(images)
}

#[cfg(test)]
mod test {
    use crate::repository::types::image::ImageDto;
    use crate::types::Result;

    use super::{list_images, list_images_by_container};

    #[tokio::test]
    async fn test_list_images() {
        // When
        let result: Result<Vec<ImageDto>> = list_images().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all images with error {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<ImageDto>> = list_images_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error {}", err),
        }
    }
}
