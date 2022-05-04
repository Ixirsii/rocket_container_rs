//! Container service.

use log::trace;

use crate::service::advertisement::AdvertisementService;
use crate::service::image::ImageService;
use crate::service::types::advertisement::{Advertisement, AdvertisementMap};
use crate::service::types::container::Container;
use crate::service::types::image::{Image, ImageMap};
use crate::service::types::video::{Video, VideoMap};
use crate::service::video::VideoService;
use crate::types::Result;

/// Container service.
///
/// Container service aggregates data from [`AdvertisementService`], [`ImageService`], and
/// [`VideoService`] into containers by container ID.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct ContainerService {
    /// Advertisement service.
    advertisement_service: AdvertisementService,
    /// Image service.
    image_service: ImageService,
    /// Video service.
    video_service: VideoService,
}

impl ContainerService {
    /// Create a new container service.
    pub fn new(
        advertisement_service: AdvertisementService,
        image_service: ImageService,
        video_service: VideoService,
    ) -> Self {
        Self {
            advertisement_service,
            image_service,
            video_service,
        }
    }

    /// Get container by ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn get_container(&self, container_id: u32) -> Result<Container> {
        trace!("get_container: {}", container_id);

        let advertisements: Vec<Advertisement> = self
            .advertisement_service
            .list_advertisements_by_container(container_id)
            .await?;
        let images: Vec<Image> = self
            .image_service
            .list_images_by_container(container_id)
            .await?;
        let videos: Vec<Video> = self
            .video_service
            .list_videos_by_container(container_id)
            .await?;

        Ok(Container::from(
            container_id,
            &advertisements,
            &images,
            &videos,
        ))
    }

    /// Get all containers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_containers(&self) -> Result<Vec<Container>> {
        trace!("list_containers");

        let advertisements: AdvertisementMap =
            self.advertisement_service.list_advertisements().await?;
        let images: ImageMap = self.image_service.list_images().await?;
        let videos: VideoMap = self.video_service.list_videos().await?;

        let containers: Vec<Container> = videos
            .iter()
            .map(|(container_id, videos)| {
                self.build_container(*container_id, &advertisements, &images, videos)
            })
            .collect();

        Ok(containers)
    }

    /* ****************************** Private utility function ****************************** */

    /// Build a container from data maps.
    ///
    /// Gets advertisements for a container from an [`AdvertisementMap`], images from an
    /// [`ImageMap`], and combines them with a list of videos to make a container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    fn build_container(
        &self,
        container_id: u32,
        advertisements: &AdvertisementMap,
        images: &ImageMap,
        videos: &[Video],
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
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{
        service::{container::ContainerService, types::container::Container},
        types::Result,
    };

    #[tokio::test]
    async fn test_get_container() {
        // Given
        let under_test = ContainerService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Container> = under_test.get_container(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get container with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_containers() {
        // Given
        let under_test = ContainerService::default();
        let expected: usize = 31;

        // When
        let result: Result<Vec<Container>> = under_test.list_containers().await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual.len()),
            Err(err) => panic!("Failed to list containers with error: {}", err),
        }
    }
}
