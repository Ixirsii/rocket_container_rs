//! Container service.

use reqwest::Client;

use crate::service::advertisement;
use crate::service::image;
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::service::types::container::Container;
use crate::service::types::image::{Image, ImageMap};
use crate::service::types::video::{Video, VideoMap};
use crate::service::video;
use crate::types::Result;

pub async fn get_advertisements(client: &Client, container_id: u32) -> Vec<Advertisement> {
    todo!("list_advertisements")
}

pub async fn get_container(client: &Client, container_id: u32) -> Container {
    todo!("get_container")
}

pub async fn get_images(client: &Client, container_id: u32) -> Vec<Image> {
    todo!("list_images")
}

pub async fn get_videos(client: &Client, container_id: u32) -> Vec<Video> {
    todo!("list_videos")
}

pub async fn list_containers(client: &Client) -> Result<Vec<Container>> {
    let advertisements: AdvertisementMap = advertisement::list_advertisements(client).await?;
    let images: ImageMap = image::list_images(client).await?;
    let videos: VideoMap = video::list_videos(client).await?;

    let containers: Vec<Container> = videos
        .iter()
        .map(|(container_id, videos)| {
            build_container(*container_id, &advertisements, &images, videos)
        })
        .collect();

    Ok(containers)
}

/* ********************************** Private utility function ********************************** */

fn build_container(
    container_id: u32,
    advertisements: &AdvertisementMap,
    images: &ImageMap,
    videos: &Vec<Video>,
) -> Container {
    let advertisements_default: &Vec<Advertisement> = &Vec::new();
    let images_default: &Vec<Image> = &Vec::new();

    let advertisements: &Vec<Advertisement> = match advertisements.get(&container_id) {
        Some(advertisements) => advertisements,
        None => advertisements_default,
    };
    let images: &Vec<Image> = match images.get(&container_id) {
        Some(images) => images,
        None => images_default,
    };

    Container::from(container_id, advertisements, images, videos)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::service::types::container::Container;
    use crate::types::Result;

    use super::list_containers;

    #[tokio::test]
    async fn test_list_containers() {
        // Given
        let client = Client::new();
        let expected: usize = 31;

        // When
        let result: Result<Vec<Container>> = list_containers(&client).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual.len()),
            Err(err) => panic!("Failed to list containers with error: {}", err),
        }
    }
}
