//! Container service.

use std::fmt::{Display, Formatter};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    service::{
        advertisement::{Advertisement, AdvertisementMap, AdvertisementService},
        image::{Image, ImageMap, ImageService},
        video::{Video, VideoMap, VideoService},
    },
    types::Result,
};

/* ***************************************** Container ****************************************** */

/// Container asset returned from Rocket Container.
///
/// A Container is an aggregation of advertisements, images, and videos. Rocket Container gets
/// this data from its dependencies, Rocket Advertisement, Rocket Image, and Rocket Video, and
/// aggregates them into containers for Rocket Stream.
///
/// # Examples
///
/// ```rust
/// todo!("Example");
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Container {
    ads: Vec<Advertisement>,
    id: u32,
    images: Vec<Image>,
    title: String,
    videos: Vec<Video>,
}

impl Container {
    /// Construct a new Container.
    pub fn new(
        ads: Vec<Advertisement>,
        id: u32,
        images: Vec<Image>,
        title: String,
        videos: Vec<Video>,
    ) -> Self {
        Container {
            ads,
            id,
            images,
            title,
            videos,
        }
    }

    /// Create a container from a list of advertisements, images, and videos.
    pub fn from(
        container_id: u32,
        advertisements: &[Advertisement],
        images: &[Image],
        videos: &[Video],
    ) -> Self {
        let title_ads: String = match advertisements.is_empty() {
            false => "_ads".to_string(),
            true => String::new(),
        };
        let title_images: String = match images.is_empty() {
            false => "_images".to_string(),
            true => String::new(),
        };
        let title: String = format!(
            "container-{}{}${}_videos",
            container_id, title_ads, title_images
        );

        Container::new(
            advertisements.to_vec(),
            container_id,
            images.to_vec(),
            title,
            videos.to_vec(),
        )
    }

    /// Get advertisements.
    pub fn ads(&self) -> &Vec<Advertisement> {
        &self.ads
    }

    /// Get container ID.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get images.
    pub fn images(&self) -> &Vec<Image> {
        &self.images
    }

    /// Get title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get videos.
    pub fn videos(&self) -> &Vec<Video> {
        &self.videos
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Container {{ id: {}, title: {} }}", self.id, self.title)
    }
}

/* ************************************** ContainerService ************************************** */

/// Container service.
///
/// Container service aggregates data from [`AdvertisementService`], [`ImageService`], and
/// [`VideoService`] into containers by container ID.
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

    /// List all advertisements for a container.
    pub async fn list_advertisements(&self, container_id: u32) -> Result<Vec<Advertisement>> {
        self.advertisement_service
            .list_advertisements_by_container(container_id)
            .await
    }

    /// Get all containers.
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

    /// List all images for a container.
    pub async fn list_images(&self, container_id: u32) -> Result<Vec<Image>> {
        self.image_service
            .list_images_by_container(container_id)
            .await
    }

    /// List all videos for a container.
    pub async fn list_videos(&self, container_id: u32) -> Result<Vec<Video>> {
        self.video_service
            .list_videos_by_container(container_id)
            .await
    }

    /* ****************************** Private utility function ****************************** */

    /// Build a container from data maps.
    ///
    /// Gets advertisements for a container from an [`AdvertisementMap`], images from an
    /// [`ImageMap`], and combines them with a list of videos to make a container.
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
    use crate::service::advertisement::Advertisement;
    use crate::service::image::Image;
    use crate::service::video::Video;
    use crate::types::Result;

    use super::{Container, ContainerService};

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
    async fn test_list_advertisements() {
        // Given
        let under_test = ContainerService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Advertisement>> = under_test.list_advertisements(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get advertisements with error: {}", err),
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

    #[tokio::test]
    async fn test_list_images() {
        // Given
        let under_test = ContainerService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Image>> = under_test.list_images(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get images with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // Given
        let under_test = ContainerService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Video>> = under_test.list_videos(container_id).await;

        // Then
        match result {
            Ok(_) => (),
            Err(err) => panic!("Failed to get videos with error: {}", err),
        }
    }
}
