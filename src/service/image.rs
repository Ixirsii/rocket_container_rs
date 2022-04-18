//! Advertisement service.

use log::trace;
use reqwest::Client;

use crate::repository::image;
use crate::service::group;
use crate::service::types::image::{Image, ImageMap};
use crate::types::Result;

/// List all images from Rocket Image.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::image::list_images;
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
pub async fn list_images(client: &Client) -> Result<ImageMap> {
    trace!("Listing all images");

    let images: Vec<(u32, Image)> = image::list_images(client)
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
/// use rocket_stream::service::image::list_images_by_container;
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
pub async fn list_images_by_container(client: &Client, container_id: u32) -> Result<ImageMap> {
    trace!("Listing images by container id {}", container_id);

    let images: Vec<(u32, Image)> = image::list_images_by_container(client, container_id)
        .await?
        .into_iter()
        .map(|image| (image.container_id().parse().unwrap(), Image::from(image)))
        .collect();

    Ok(group(images.into_iter()))
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::service::types::image::ImageMap;
    use crate::types::Result;

    use super::{list_images, list_images_by_container};

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<ImageMap> = list_images(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<ImageMap> = list_images_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }
}
