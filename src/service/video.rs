//! Advertisement service.

extern crate futures;

use futures::future;
use log::trace;

use crate::repository::types::video::VideoDto;
use crate::repository::video::VideoRepository;
use crate::service::group;
use crate::service::types::video::{AssetReference, Video, VideoBuilder, VideoMap};
use crate::types::{AssetType, Result, VideoType};

/// Image service.
///
/// [`VideoService`] is the service layer wrapper for [`VideoRepository`]. It transforms
/// DTO types into domain types.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct VideoService {
    /// Repository layer that the service calls.
    video_repository: VideoRepository,
}

impl<'a> VideoService {
    //// Create a new [`VideoService`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(video_repository: VideoRepository) -> Self {
        Self { video_repository }
    }

    /// Get video by ID from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn get_video(&self, video_id: u32) -> Result<Video> {
        trace!("VideoService::get_video {}", video_id);

        let assets: Vec<AssetReference> = self.list_asset_references(video_id).await?;
        let video: Video = VideoBuilder::from(self.video_repository.get_video(video_id).await?)
            .assets(assets)
            .build();

        Ok(video)
    }

    /// List all assets for a video from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_asset_references(&self, video_id: u32) -> Result<Vec<AssetReference>> {
        trace!("VideoService::list_asset_references {}", video_id);

        let asset_references: Vec<AssetReference> = self
            .video_repository
            .list_asset_references(video_id)
            .await?
            .into_iter()
            .map(AssetReference::from)
            .collect();

        Ok(asset_references)
    }

    /// List all assets for a video, by type, from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_asset_references_by_type(
        &self,
        video_id: u32,
        asset_type: AssetType,
    ) -> Result<Vec<AssetReference>> {
        trace!(
            "VideoService::list_asset_references_by_type ({}, {})",
            video_id,
            asset_type
        );

        let asset_references: Vec<AssetReference> = self
            .video_repository
            .list_asset_references_by_type(video_id, asset_type)
            .await?
            .into_iter()
            .map(AssetReference::from)
            .collect();

        Ok(asset_references)
    }

    /// List all videos from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos(&self) -> Result<VideoMap> {
        trace!("VideoService::list_videos");

        let images: Vec<(u32, Video)> = future::try_join_all(
            self.video_repository
                .list_videos()
                .await?
                .into_iter()
                .map(|video_dto| self.map_video_dto_to_tuple(video_dto)),
        )
        .await
        .unwrap();

        Ok(group(images.into_iter()))
    }

    /// List all videos for a container from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos_by_container(&self, container_id: u32) -> Result<Vec<Video>> {
        trace!("VideoService::list_videos_by_container {}", container_id);

        let images: Vec<Video> = future::try_join_all(
            self.video_repository
                .list_videos_by_container(container_id)
                .await?
                .into_iter()
                .map(|video_dto| self.map_video_dto_to_video(video_dto)),
        )
        .await
        .unwrap();

        Ok(images)
    }

    /// List all videos by type from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos_by_type(&self, video_type: VideoType) -> Result<VideoMap> {
        trace!("VideoService::list_videos_by_type {}", video_type);

        let images: Vec<(u32, Video)> = future::try_join_all(
            self.video_repository
                .list_videos_by_type(video_type)
                .await?
                .into_iter()
                .map(|video_dto| self.map_video_dto_to_tuple(video_dto)),
        )
        .await
        .unwrap();

        Ok(group(images.into_iter()))
    }

    /// List all videos for a container, by type, from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos_by_container_and_type(
        &self,
        container_id: u32,
        video_type: VideoType,
    ) -> Result<VideoMap> {
        trace!(
            "VideoService::list_videos_by_container_and_type ({}, {})",
            container_id,
            video_type
        );

        let images: Vec<(u32, Video)> = future::try_join_all(
            self.video_repository
                .list_videos_by_container_and_type(container_id, video_type)
                .await?
                .into_iter()
                .map(|video_dto| self.map_video_dto_to_tuple(video_dto)),
        )
        .await
        .unwrap();

        Ok(group(images.into_iter()))
    }

    /* ****************************** Private utility function ****************************** */

    async fn map_video_dto_to_video(&self, video_dto: VideoDto) -> Result<Video> {
        let assets: Vec<AssetReference> = self
            .list_asset_references(video_dto.id().parse().unwrap())
            .await?;

        Ok(VideoBuilder::from(video_dto).assets(assets).build())
    }

    async fn map_video_dto_to_tuple(&self, video_dto: VideoDto) -> Result<(u32, Video)> {
        let assets: Vec<AssetReference> = self
            .list_asset_references(video_dto.id().parse().unwrap())
            .await?;

        Ok((
            video_dto.container_id().parse().unwrap(),
            VideoBuilder::from(video_dto).assets(assets).build(),
        ))
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{
        service::types::video::{AssetReference, Video, VideoMap},
        types::{AssetType, Result, VideoType},
    };

    use super::VideoService;

    #[tokio::test]
    async fn test_get_video() {
        // Given
        let service = VideoService::default();
        let video_id: u32 = 1301;
        let expected: Video = Video::new(
            Vec::new(),
            "Etiam vel augue. Vestibulum rutrum rutrum neque. Aenean auctor gravida sem."
                .to_string(),
            "".to_string(),
            1301,
            "/path/to/test1301.m3u8".to_string(),
            "My Family".to_string(),
            VideoType::Clip,
        );

        // When
        let result: Result<Video> = service.get_video(video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to get video with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references() {
        // Given
        let service = VideoService::default();
        let video_id: u32 = 1404;
        let expected: Vec<AssetReference> = vec![AssetReference::new(120, AssetType::Image)];

        // When
        let result: Result<Vec<AssetReference>> = service.list_asset_references(video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references_by_type() {
        // Given
        let service = VideoService::default();
        let asset_type: AssetType = AssetType::Image;
        let video_id: u32 = 1404;
        let expected: Vec<AssetReference> = vec![AssetReference::new(120, AssetType::Image)];

        // When
        let result: Result<Vec<AssetReference>> = service
            .list_asset_references_by_type(video_id, asset_type)
            .await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // Given
        let service = VideoService::default();

        // When
        let result: Result<VideoMap> = service.list_videos().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let service = VideoService::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Video>> = service.list_videos_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_type() {
        // Given
        let service = VideoService::default();
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<VideoMap> = service.list_videos_by_type(video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container_and_type() {
        // Given
        let service = VideoService::default();
        let container_id: u32 = 0;
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<VideoMap> = service
            .list_videos_by_container_and_type(container_id, video_type)
            .await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }
}
