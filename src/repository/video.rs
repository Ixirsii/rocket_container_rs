//! Video repository.

use log::trace;
use std::sync::Arc;

use crate::repository::client::Client;
use crate::repository::types::video::{AssetReferenceDto, VideoAssetsDto, VideoDto, VideosDto};
use crate::types::{AssetType, Result, VideoType};

/// Asset reference endpoint suffix.
const ASSET_REFERENCES: &str = "asset-references";

/// Asset type query parameter.
const ASSET_TYPE: &str = "assetType";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Advertisement service.
const VIDEO_ENDPOINT: &str = "http://videos.rocket-stream.bottlerocketservices.com/videos";

/// Video type query parameter.
const VIDEO_TYPE: &str = "type";

/// Video repository.
///
/// [`VideoRepository`] is the repository layer which fetches videos from Rocket Video service.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Default)]
pub struct VideoRepository {
    /// Client for making requests.
    client: Arc<Client>,
}

impl VideoRepository {
    /// Create new [`VideoRepository`].
    pub fn new(client: Arc<Client>) -> Self {
        VideoRepository { client }
    }

    /// Get video by ID from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn get_video(&self, video_id: u32) -> Result<VideoDto> {
        trace!("VideoRepository::get_video {}", video_id);

        self.client
            .get::<VideoDto, ()>(format!("{}/{}", VIDEO_ENDPOINT, video_id).as_str(), None)
            .await
    }

    /// List all assets for a video from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_asset_references(&self, video_id: u32) -> Result<Vec<AssetReferenceDto>> {
        trace!("VideoRepository::list_asset_references {}", video_id);

        let asset_references: Vec<AssetReferenceDto> = self
            .client
            .get::<VideoAssetsDto, ()>(
                format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
                None,
            )
            .await?
            .video_assets;

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
    ) -> Result<Vec<AssetReferenceDto>> {
        trace!(
            "VideoRepository::list_asset_references_by_type ({}, {})",
            video_id,
            asset_type
        );

        let asset_references: Vec<AssetReferenceDto> = self
            .client
            .get::<VideoAssetsDto, [(&str, AssetType); 1]>(
                format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
                Some([(ASSET_TYPE, asset_type)]),
            )
            .await?
            .video_assets;

        Ok(asset_references)
    }

    /// List all videos from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos(&self) -> Result<Vec<VideoDto>> {
        trace!("VideoRepository::list_videos");

        let videos: Vec<VideoDto> = self
            .client
            .get::<VideosDto, ()>(VIDEO_ENDPOINT, None)
            .await?
            .videos;

        Ok(videos)
    }

    /// List all videos for a container from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos_by_container(&self, container_id: u32) -> Result<Vec<VideoDto>> {
        trace!("VideoRepository::list_videos_by_container {}", container_id);

        let videos: Vec<VideoDto> = self
            .client
            .get::<VideosDto, [(&str, u32); 1]>(
                VIDEO_ENDPOINT,
                Some([(CONTAINER_ID, container_id)]),
            )
            .await?
            .videos;

        Ok(videos)
    }

    /// List all videos by type from Rocket Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub async fn list_videos_by_type(&self, video_type: VideoType) -> Result<Vec<VideoDto>> {
        trace!("VideoRepository::list_videos_by_type {}", video_type);

        let videos: Vec<VideoDto> = self
            .client
            .get::<VideosDto, [(&str, VideoType); 1]>(
                VIDEO_ENDPOINT,
                Some([(VIDEO_TYPE, video_type)]),
            )
            .await?
            .videos;

        Ok(videos)
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
    ) -> Result<Vec<VideoDto>> {
        trace!(
            "VideoRepository::list_videos_by_container_and_type ({}, {})",
            container_id,
            video_type
        );

        let videos: Vec<VideoDto> = self
            .client
            .get::<VideosDto, [(&str, String); 2]>(
                VIDEO_ENDPOINT,
                Some([
                    (CONTAINER_ID, container_id.to_string()),
                    (VIDEO_TYPE, video_type.to_string()),
                ]),
            )
            .await?
            .videos;

        Ok(videos)
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::{
        repository::types::video::{AssetReferenceDto, VideoDto},
        types::{AssetType, Result, VideoType},
    };

    use super::VideoRepository;

    #[tokio::test]
    async fn test_get_video() {
        // Given
        let repository = VideoRepository::default();
        let video_id: u32 = 1301;

        // When
        let result: Result<VideoDto> = repository.get_video(video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(video_id.to_string(), actual.id()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references() {
        // Given
        let repository = VideoRepository::default();
        let video_id: u32 = 1404;

        // When
        let result: Result<Vec<AssetReferenceDto>> =
            repository.list_asset_references(video_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references_by_type() {
        // Given
        let repository = VideoRepository::default();
        let asset_type: AssetType = AssetType::Image;
        let video_id: u32 = 1404;

        // When
        let result: Result<Vec<AssetReferenceDto>> = repository
            .list_asset_references_by_type(video_id, asset_type)
            .await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // When
        let repository = VideoRepository::default();
        let result: Result<Vec<VideoDto>> = repository.list_videos().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let repository = VideoRepository::default();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<VideoDto>> = repository.list_videos_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_type() {
        // Given
        let repository = VideoRepository::default();
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<Vec<VideoDto>> = repository.list_videos_by_type(video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container_and_type() {
        // Given
        let repository = VideoRepository::default();
        let container_id: u32 = 0;
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<Vec<VideoDto>> = repository
            .list_videos_by_container_and_type(container_id, video_type)
            .await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }
}
