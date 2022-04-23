//! Container service.

use log::trace;

use crate::service::advertisement::{list_advertisements, list_advertisements_by_container};
use crate::service::image::{list_images, list_images_by_container};
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::service::types::container::Container;
use crate::service::types::image::{Image, ImageMap};
use crate::service::types::video::{Video, VideoMap};
use crate::service::video::{list_videos, list_videos_by_container};
use crate::types::Result;

pub async fn get_container(container_id: u32) -> Result<Container> {
    trace!("get_container: {}", container_id);

    let advertisements: Vec<Advertisement> = list_advertisements_by_container(container_id).await?;
    let images: Vec<Image> = list_images_by_container(container_id).await?;
    let videos: Vec<Video> = list_videos_by_container(container_id).await?;

    Ok(Container::from(
        container_id,
        &advertisements,
        &images,
        &videos,
    ))
}

pub async fn list_containers() -> Result<Vec<Container>> {
    trace!("list_containers");

    let advertisements: AdvertisementMap = list_advertisements().await?;
    let images: ImageMap = list_images().await?;
    let videos: VideoMap = list_videos().await?;

    let containers: Vec<Container> = videos
        .iter()
        .map(|(container_id, videos)| {
            build_container(*container_id, &advertisements, &images, videos)
        })
        .collect();

    Ok(containers)
}

/* ****************************** Private utility function ****************************** */

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
    use crate::service::container::{get_container, list_containers};
    use crate::service::types::container::Container;
    use crate::types::Result;

    #[tokio::test]
    async fn test_get_container() {
        // Given
        let container_id: u32 = 0;

        // When
        let result: Result<Container> = get_container(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get container with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_containers() {
        // Given
        let expected: usize = 31;

        // When
        let result: Result<Vec<Container>> = list_containers().await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual.len()),
            Err(err) => panic!("Failed to list containers with error: {}", err),
        }
    }
}
