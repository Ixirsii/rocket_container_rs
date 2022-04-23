//! Container service.

use std::rc::Rc;

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

/// Cache type for container data.
///
/// Caches a [Rc]<[Container]> for a given [Container] ID.
pub type Cache = LruCache<u32, Rc<Container>>;

/// Container service.
pub struct ContainerService {
    /// Client for making HTTP requests.
    client: Client,
    /// Cache for container data.
    cache: Cache,
}

impl ContainerService {
    pub async fn get_container(&mut self, container_id: u32) -> Result<Rc<Container>> {
        trace!("get_container: {}", container_id);

        match self.cache.get(&container_id) {
            Some(container) => Ok(container.clone()),
            None => {
                let container: Container = self.fetch_container(container_id).await?;
                let container = Rc::new(container);

                self.cache.put(container_id, container.clone());

                Ok(container)
            }
        }
    }

    pub async fn list_containers(&mut self) -> Result<Vec<Rc<Container>>> {
        trace!("list_containers");

        let advertisements: AdvertisementMap =
            advertisement::list_advertisements(&self.client).await?;
        let images: ImageMap = image::list_images(&self.client).await?;
        let videos: VideoMap = video::list_videos(&self.client).await?;

        let containers: Vec<Rc<Container>> = videos
            .iter()
            .map(|(container_id, videos)| {
                let container: Container =
                    self.build_container(*container_id, &advertisements, &images, videos);
                let container: Rc<Container> = Rc::new(container);

                self.cache.put(*container_id, container.clone());

                container
            })
            .collect();

        Ok(containers)
    }

    /* ****************************** Private utility function ****************************** */

    fn build_container(
        &self,
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

    async fn fetch_container(&self, container_id: u32) -> Result<Container> {
        trace!("fetch_container: {}", container_id);

        let advertisement_map: AdvertisementMap =
            advertisement::list_advertisements_by_container(&self.client, container_id).await?;
        let advertisements: &Vec<Advertisement> = match advertisement_map.get(&container_id) {
            Some(advertisements) => Ok(advertisements),
            None => Err(Error::new(
                ErrorKind::Permanent,
                "Failed to get advertisements",
            )),
        }?;

        let image_map: ImageMap =
            image::list_images_by_container(&self.client, container_id).await?;
        let images: &Vec<Image> = match image_map.get(&container_id) {
            Some(images) => Ok(images),
            None => Err(Error::new(ErrorKind::Permanent, "Failed to get images")),
        }?;

        let video_map: VideoMap =
            video::list_videos_by_container(&self.client, container_id).await?;
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
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use lru::LruCache;
    use reqwest::Client;

    use crate::service::container::{Cache, ContainerService};
    use crate::service::types::container::Container;
    use crate::types::Result;

    #[tokio::test]
    async fn test_get_container() {
        // Given
        let container_service: &mut ContainerService = &mut ContainerService {
            cache: LruCache::new(1),
            client: Client::new(),
        };
        let container_id: u32 = 0;

        // When
        let result: Result<Rc<Container>> = container_service.get_container(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get container with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_containers() {
        // Given
        let container_service: &mut ContainerService = &mut ContainerService {
            cache: LruCache::new(31),
            client: Client::new(),
        };
        let expected: usize = 31;

        // When
        let result: Result<Vec<Rc<Container>>> = container_service.list_containers().await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual.len()),
            Err(err) => panic!("Failed to list containers with error: {}", err),
        }
    }
}
