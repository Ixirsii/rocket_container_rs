//! Image repository.

use crate::repository::get;
use crate::repository::types::{Image, Images};
use crate::types::Result;
use log::trace;
use reqwest::Client;

/// Endpoint for Rocket Image service.
const IMAGE_ENDPOINT: &str = "http://images.rocket-stream.bottlerocketservices.com/images";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

pub async fn list_all_images(client: &Client) -> Result<Vec<Image>> {
    trace!("Listing all images");

    get::<Image, Images, ()>(client, IMAGE_ENDPOINT, None).await
}

pub async fn list_images(client: &Client, container_id: u32) -> Result<Vec<Image>> {
    trace!("Listing images for container {}", container_id);

    get::<Image, Images, [(&str, u32); 1]>(
        client,
        IMAGE_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

#[cfg(test)]
mod test {
    use super::{list_all_images, list_images};
    use crate::repository::types::Image;
    use crate::types::Result;
    use reqwest::Client;

    #[tokio::test]
    async fn test_list_all_images() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<Image>> = list_all_images(&client).await;

        // Then
        match result {
            Ok(images) => assert!(!images.is_empty()),
            Err(err) => panic!("Failed to list all images with error {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = list_images(&client, container_id).await;

        // Then
        match result {
            Ok(images) => assert!(!images.is_empty()),
            Err(err) => panic!("Failed to list images with error {:#?}", err),
        }
    }
}
