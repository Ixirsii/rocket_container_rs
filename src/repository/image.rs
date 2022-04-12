//! Image repository.

use log::trace;
use reqwest::Client;

use crate::types::Result;

use super::get_wrapped_list;
use super::types::image::{ImageDto, ImagesDto};

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Image service.
const IMAGE_ENDPOINT: &str = "http://images.rocket-stream.bottlerocketservices.com/images";

pub async fn list_images(client: &Client) -> Result<Vec<ImageDto>> {
    trace!("Listing all images");

    get_wrapped_list::<ImageDto, ImagesDto, ()>(client, IMAGE_ENDPOINT, None).await
}

pub async fn list_images_by_container(client: &Client, container_id: u32) -> Result<Vec<ImageDto>> {
    trace!("Listing images for container {}", container_id);

    get_wrapped_list::<ImageDto, ImagesDto, [(&str, u32); 1]>(
        client,
        IMAGE_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::repository::types::image::ImageDto;
    use crate::types::Result;

    use super::{list_images, list_images_by_container};

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<ImageDto>> = list_images(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all images with error {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<ImageDto>> = list_images_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error {:#?}", err),
        }
    }
}
