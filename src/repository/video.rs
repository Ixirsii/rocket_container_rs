//! Video repository.

use std::fmt::{Display, Formatter};
use std::sync::Arc;

use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    repository::client::Client,
    service::video::{AssetReference, Video, VideoBuilder},
    types::{array_to_string, AssetType, Result, VideoType},
};

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

/* ************************************* AssetReferenceDto ************************************** */

/// A reference to an asset associated with a [Video].
///
/// [Video]s can have assets associated with them such as promotional images and advertisements.
/// This is a generic pointer to those assets.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDto {
    /// Unique identifier for referenced asset.
    asset_id: String,
    /// Type of asset.
    asset_type: AssetType,
    /// Unique identifier for referenced video.
    video_id: String,
}

impl AssetReferenceDto {
    /// Construct a new AssetReference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(asset_id: String, asset_type: AssetType, video_id: String) -> Self {
        AssetReferenceDto {
            asset_id,
            asset_type,
            video_id,
        }
    }
}

impl Display for AssetReferenceDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AssetReferenceDto {{ asset_id: {}, asset_type: {}, video_id: {} }}",
            self.asset_id, self.asset_type, self.video_id
        )
    }
}

impl From<AssetReferenceDto> for AssetReference {
    /// Get an [AssetReference] from an [AssetReferenceDto].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    fn from(asset_reference_dto: AssetReferenceDto) -> Self {
        AssetReference::new(
            asset_reference_dto.asset_id.parse().unwrap(),
            asset_reference_dto.asset_type,
        )
    }
}

/* ***************************************** VideoDto ******************************************* */

/// Video data returned from Rocket Video service.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDto {
    /// Parent container e.g. show/series identifier.
    container_id: String,
    /// Brief description of the video.
    description: String,
    /// Expiration date for video in ISO-8601 format.
    expiration_date: String,
    /// Unique video identifier.
    id: String,
    /// URL for video playback.
    playback_url: String,
    /// Video title.
    title: String,
    /// Type of video.
    r#type: VideoType,
}

impl VideoDto {
    /// Construct a new Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(
        container_id: String,
        description: String,
        expiration_date: String,
        id: String,
        playback_url: String,
        title: String,
        r#type: VideoType,
    ) -> Self {
        VideoDto {
            container_id,
            description,
            expiration_date,
            id,
            playback_url,
            title,
            r#type,
        }
    }

    /// Get container ID.
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Get Video ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Display for VideoDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VideoDto {{ id: {}, title: {}, description: {} }}",
            self.id, self.title, self.description
        )
    }
}

impl From<VideoDto> for VideoBuilder {
    /// Get a [Video] from a [VideoDto].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    fn from(video_dto: VideoDto) -> VideoBuilder {
        Video::builder(video_dto.id.parse().unwrap())
            .description(video_dto.description)
            .expiration_date(video_dto.expiration_date)
            .playback_url(video_dto.playback_url)
            .title(video_dto.title)
            .r#type(video_dto.r#type)
    }
}

/* ************************************** VideoAssetsDto **************************************** */

/// [Wrapper] for [Video]s.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAssetsDto {
    /// List of video asset references.
    pub video_assets: Vec<AssetReferenceDto>,
}

impl Display for VideoAssetsDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VideoAssetsDto {{ video_assets: {} }}",
            array_to_string(&self.video_assets)
        )
    }
}

/* ***************************************** VideosDto ****************************************** */

/// [Wrapper] for [Video]s.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideosDto {
    /// List of videos.
    pub videos: Vec<VideoDto>,
}

impl Display for VideosDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VideosDto {{ videos: {:?} }}",
            array_to_string(&self.videos)
        )
    }
}

/* ************************************** VideoRepository *************************************** */

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
    use crate::types::{AssetType, Result, VideoType};

    use super::{AssetReferenceDto, VideoDto, VideoRepository, VideosDto};

    #[test]
    fn deserialize_asset_reference() {
        // Given
        let data: &str = r#"
            {
                "assetId": "0",
                "assetType": "AD",
                "videoId": "0"
            }
        "#;

        let expected: AssetReferenceDto = AssetReferenceDto {
            asset_id: 0.to_string(),
            asset_type: AssetType::Ad,
            video_id: 0.to_string(),
        };

        // When
        let result: serde_json::Result<AssetReferenceDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_video() {
        // Given
        let data: &str = r#"
            {
                "containerId": "0",
                "description": "A short video clip",
                "expirationDate": "2022-03-23",
                "id": "0",
                "playbackUrl": "https://www.youtube.com/watch?v=00000000000",
                "title": "Video",
                "type": "CLIP"
            }
        "#;

        let expected: VideoDto = VideoDto {
            container_id: 0.to_string(),
            description: "A short video clip".to_string(),
            expiration_date: "2022-03-23".to_string(),
            id: 0.to_string(),
            playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
            title: "Video".to_string(),
            r#type: VideoType::Clip,
        };

        // When
        let result: serde_json::Result<VideoDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_videos() {
        // Given
        let data: &str = r#"
            {
                "videos": [
                    {
                        "containerId": "0",
                        "description": "A short video clip",
                        "expirationDate": "2022-03-23",
                        "id": "0",
                        "playbackUrl": "https://www.youtube.com/watch?v=00000000000",
                        "title": "Video",
                        "type": "CLIP"
                    }
                ]
            }
        "#;

        let expected: VideosDto = VideosDto {
            videos: Vec::from([VideoDto {
                container_id: 0.to_string(),
                description: "A short video clip".to_string(),
                expiration_date: "2022-03-23".to_string(),
                id: 0.to_string(),
                playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
                title: "Video".to_string(),
                r#type: VideoType::Clip,
            }]),
        };

        // When
        let result: serde_json::Result<VideosDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_asset_reference() {
        // Given
        let data: AssetReferenceDto = AssetReferenceDto {
            asset_id: 0.to_string(),
            asset_type: AssetType::Ad,
            video_id: 0.to_string(),
        };

        let expected: &str = r#"{"assetId":"0","assetType":"AD","videoId":"0"}"#;

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_video() {
        // Given
        let data: VideoDto = VideoDto {
            container_id: 0.to_string(),
            description: "A short video clip".to_string(),
            expiration_date: "2022-03-23".to_string(),
            id: 0.to_string(),
            playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
            title: "Video".to_string(),
            r#type: VideoType::Clip,
        };

        let expected: &str = "\
            {\
                \"containerId\":\"0\",\
                \"description\":\"A short video clip\",\
                \"expirationDate\":\"2022-03-23\",\
                \"id\":\"0\",\
                \"playbackUrl\":\"https://www.youtube.com/watch?v=00000000000\",\
                \"title\":\"Video\",\
                \"type\":\"CLIP\"\
            }\
        ";

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_videos() {
        // Given
        let data: VideosDto = VideosDto {
            videos: Vec::from([VideoDto {
                container_id: 0.to_string(),
                description: "A short video clip".to_string(),
                expiration_date: "2022-03-23".to_string(),
                id: 0.to_string(),
                playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
                title: "Video".to_string(),
                r#type: VideoType::Clip,
            }]),
        };

        let expected: &str = "\
            {\
                \"videos\":[\
                    {\
                        \"containerId\":\"0\",\
                        \"description\":\"A short video clip\",\
                        \"expirationDate\":\"2022-03-23\",\
                        \"id\":\"0\",\
                        \"playbackUrl\":\"https://www.youtube.com/watch?v=00000000000\",\
                        \"title\":\"Video\",\
                        \"type\":\"CLIP\"\
                    }\
                ]\
            }\
        ";

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

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
