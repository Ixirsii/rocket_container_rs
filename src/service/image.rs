//! Advertisement service.

use log::trace;
use reqwest::Client;

use crate::controller::types::Image;
use crate::repository::image;
use crate::types::Result;

pub async fn list_images(client: &Client) -> Result<Vec<Image>> {
    trace!("Listing all advertisements");

    let images: Vec<Image> = image::list_images(client)
        .await?
        .into_iter()
        .map(Image::from)
        .collect();

    Ok(images)
}

pub async fn list_images_by_container(client: &Client, container_id: u32) -> Result<Vec<Image>> {
    trace!("Listing advertisements by container id {}", container_id);

    let images: Vec<Image> = image::list_images_by_container(client, container_id)
        .await?
        .into_iter()
        .map(Image::from)
        .collect();

    Ok(images)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::controller::types::Image;
    use crate::types::Result;

    use super::{list_images, list_images_by_container};

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<Image>> = list_images(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_advertisements_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = list_images_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }
}
