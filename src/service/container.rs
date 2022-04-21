//! Container service.

use log::trace;
use lru::LruCache;
use reqwest::Client;

use crate::service::advertisement;
use crate::service::image;
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::service::types::container::Container;
use crate::service::types::image::{Image, ImageMap};
use crate::service::types::video::{Video, VideoMap};
use crate::service::video;
use crate::types::{Error, ErrorKind, Result};

pub async fn get_advertisements(
    client: &Client,
    container_id: u32,
    cache: &mut LruCache<u32, Container>,
) -> Result<Vec<Advertisement>> {
    trace!("get_advertisements: {}", container_id);

    let advertisements: Vec<Advertisement> = get_container(client, container_id, cache)
        .await?
        .ads()
        .clone();

    Ok(advertisements)
}

pub async fn get_container(
    client: &Client,
    container_id: u32,
    cache: &mut LruCache<u32, Container>,
) -> Result<Container> {
    trace!("get_container: {}", container_id);

    match cache.get(&container_id) {
        Some(container) => Ok(container.clone()),
        None => {
            let container = fetch_container(client, container_id).await?;
            cache.put(container_id, container.clone());
            Ok(container)
        }
    }
}

pub async fn get_images(
    client: &Client,
    container_id: u32,
    cache: &mut LruCache<u32, Container>,
) -> Result<Vec<Image>> {
    trace!("get_images: {}", container_id);

    let images: Vec<Image> = get_container(client, container_id, cache)
        .await?
        .images()
        .clone();

    Ok(images)
}

pub async fn get_videos(
    client: &Client,
    container_id: u32,
    cache: &mut LruCache<u32, Container>,
) -> Result<Vec<Video>> {
    trace!("get_videos: {}", container_id);

    let videos: Vec<Video> = get_container(client, container_id, cache)
        .await?
        .videos()
        .clone();

    Ok(videos)
}

pub async fn list_containers(
    client: &Client,
    cache: &mut LruCache<u32, Container>,
) -> Result<Vec<Container>> {
    trace!("list_containers");

    let advertisements: AdvertisementMap = advertisement::list_advertisements(client).await?;
    let images: ImageMap = image::list_images(client).await?;
    let videos: VideoMap = video::list_videos(client).await?;

    let containers: Vec<Container> = videos
        .iter()
        .map(|(container_id, videos)| {
            let container: Container =
                build_container(*container_id, &advertisements, &images, videos);

            cache.put(*container_id, container.clone());

            container
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

async fn fetch_container(client: &Client, container_id: u32) -> Result<Container> {
    trace!("fetch_container: {}", container_id);

    let advertisement_map: AdvertisementMap =
        advertisement::list_advertisements_by_container(client, container_id).await?;
    let advertisements: &Vec<Advertisement> = match advertisement_map.get(&container_id) {
        Some(advertisements) => Ok(advertisements),
        None => Err(Error::new(
            ErrorKind::Permanent,
            "Failed to get advertisements",
        )),
    }?;

    let image_map: ImageMap = image::list_images_by_container(client, container_id).await?;
    let images: &Vec<Image> = match image_map.get(&container_id) {
        Some(images) => Ok(images),
        None => Err(Error::new(ErrorKind::Permanent, "Failed to get images")),
    }?;

    let video_map: VideoMap = video::list_videos_by_container(client, container_id).await?;
    let videos: &Vec<Video> = match video_map.get(&container_id) {
        Some(videos) => Ok(videos),
        None => Err(Error::new(ErrorKind::Permanent, "Failed to get videos")),
    }?;

    Ok(Container::from(
        container_id,
        advertisements,
        images,
        videos,
    ))
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use lru::LruCache;
    use reqwest::Client;

    use crate::service::container::{
        get_advertisements, get_container, get_images, get_videos, list_containers,
    };
    use crate::service::types::advertisement::Advertisement;
    use crate::service::types::container::Container;
    use crate::service::types::image::Image;
    use crate::service::types::video::Video;
    use crate::types::Result;

    #[tokio::test]
    async fn test_get_advertisements() {
        // Given
        let cache: &mut LruCache<u32, Container> = &mut LruCache::new(1);
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Advertisement>> =
            get_advertisements(&client, container_id, cache).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_get_container() {
        // Given
        let cache: &mut LruCache<u32, Container> = &mut LruCache::new(1);
        let client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Container> = get_container(&client, container_id, cache).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get container with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_get_images() {
        // Given
        let cache: &mut LruCache<u32, Container> = &mut LruCache::new(1);
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = get_images(&client, container_id, cache).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list images with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let cache: &mut LruCache<u32, Container> = &mut LruCache::new(1);
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Video>> = get_videos(&client, container_id, cache).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_containers() {
        // Given
        let cache: &mut LruCache<u32, Container> = &mut LruCache::new(1);
        let client = Client::new();
        let expected: usize = 31;

        // When
        let result: Result<Vec<Container>> = list_containers(&client, cache).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual.len()),
            Err(err) => panic!("Failed to list containers with error: {}", err),
        }
    }
}
