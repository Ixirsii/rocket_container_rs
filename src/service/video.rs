//! Advertisement service.

extern crate futures;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use futures::future;
use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    repository::video::{VideoDto, VideoRepository},
    service::group,
    types::{array_to_string, option_to_string, AssetType, Result, VideoType},
};

/* *************************************** AssetReference *************************************** */

/// Asset reference returned from Rocket Container.
///
/// Container service returns a variant of [`AssetReferenceDto`][1] with `id` field as a number and
/// without `container_id` field. [`AssetReferenceDto`][1]s returned from
/// [`video repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// ```
///
/// [1]: [crate::repository::types::video::AssetReferenceDto]
/// [2]: [crate::repository::video]
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReference {
    /// Unique identifier for referenced asset.
    asset_id: u32,
    /// Type of asset.
    asset_type: AssetType,
}

impl AssetReference {
    /// Construct a new AssetReference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(asset_id: u32, asset_type: AssetType) -> Self {
        AssetReference {
            asset_id,
            asset_type,
        }
    }
}

impl Display for AssetReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AssetReference {{ asset_id: {}, asset_type: {} }}",
            self.asset_id, self.asset_type
        )
    }
}

/* ******************************************* Video ******************************************** */

/// Video asset returned from Rocket Container.
///
/// Container service returns a variant of [`VideoDto`][1] with `id` field as a number and
/// without `container_id` field. [`VideoDto`][1]s returned from
/// [`video repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// ```
///
/// [1]: [crate::repository::types::video::VideoDto]
/// [2]: [crate::repository::video]
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    /// Video assets.
    assets: Vec<AssetReference>,
    /// Brief description of the video.
    description: String,
    /// Expiration date for video in ISO-8601 format.
    expiration_date: String,
    /// Unique video identifier.
    id: u32,
    /// URL for video playback.
    playback_url: String,
    /// Video title.
    title: String,
    /// Type of video.
    r#type: VideoType,
}

impl Video {
    /// Construct a new Video.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(
        assets: Vec<AssetReference>,
        description: String,
        expiration_date: String,
        id: u32,
        playback_url: String,
        title: String,
        r#type: VideoType,
    ) -> Self {
        Video {
            assets,
            description,
            expiration_date,
            id,
            playback_url,
            title,
            r#type,
        }
    }

    /// Construct a new [VideoBuilder].
    ///
    /// Alias for [VideoBuilder::new].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn builder(id: u32) -> VideoBuilder {
        VideoBuilder::new(id)
    }

    /// Get a [VideoBuilder] with values initialized from this [Video].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn to_builder(&self) -> VideoBuilder {
        VideoBuilder::new(self.id)
            .assets(self.assets.clone())
            .description(self.description.clone())
            .expiration_date(self.expiration_date.clone())
            .playback_url(self.playback_url.clone())
            .title(self.title.clone())
            .r#type(self.r#type.clone())
    }
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Video {{ id: {}, title: {}, description: {}, expiration_date: {}, playback_url: {}, \
            type: {}, assets: {} }}",
            self.id,
            self.title,
            self.description,
            self.expiration_date,
            self.playback_url,
            self.r#type,
            self.assets.len()
        )
    }
}

/* **************************************** VideoBuilder **************************************** */

/// Builder class for [Video].
///
/// # Examples
///
/// ```rust
/// ```
pub struct VideoBuilder {
    /// See [Video::assets].
    ///
    /// Initialized to empty [Vec].
    assets: Vec<AssetReference>,
    /// See [Video::description].
    ///
    /// Initialized to [None].
    description: Option<String>,
    /// See [Video::expiration_date].
    ///
    /// Initialized to [None].
    expiration_date: Option<String>,
    /// See [Video::id].
    ///
    /// Required value.
    id: u32,
    /// See [Video::playback_url].
    ///
    /// Initialized to [None].
    playback_url: Option<String>,
    /// See [Video::title].
    ///
    /// Initialized to [None].
    title: Option<String>,
    /// See [Video::r#type].
    ///
    /// Initialized to [None].
    r#type: Option<VideoType>,
}

impl VideoBuilder {
    /// Construct a new VideoBuilder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(id: u32) -> Self {
        VideoBuilder {
            assets: Vec::new(),
            description: None,
            expiration_date: None,
            id,
            playback_url: None,
            title: None,
            r#type: None,
        }
    }

    /// Build a [Video].
    ///
    /// Builds a [Video], transferring ownership of the builder's internal state to the returned
    /// [Video].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn build(self) -> Video {
        Video {
            assets: self.assets,
            description: self.description.unwrap(),
            expiration_date: self.expiration_date.unwrap(),
            id: self.id,
            playback_url: self.playback_url.unwrap(),
            title: self.title.unwrap(),
            r#type: self.r#type.unwrap(),
        }
    }

    /// Build a [Video].
    ///
    /// Builds a [Video], cloning the builder's internal state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn build_clone(&self) -> Video {
        Video {
            assets: self.assets.clone(),
            description: self.description.clone().unwrap(),
            expiration_date: self.expiration_date.clone().unwrap(),
            id: self.id,
            playback_url: self.playback_url.clone().unwrap(),
            title: self.title.clone().unwrap(),
            r#type: self.r#type.clone().unwrap(),
        }
    }

    /// Push an asset into `VideoBuilder::assets`.
    ///
    /// Singular form of [VideoBuilder::assets].
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn asset(mut self, asset: AssetReference) -> Self {
        self.assets.push(asset);
        self
    }

    /// Set `VideoBuilder::assets`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn assets(mut self, assets: Vec<AssetReference>) -> Self {
        self.assets = assets;
        self
    }

    /// Set `VideoBuilder::description`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set `VideoBuilder::expiration_date`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn expiration_date(mut self, expiration_date: String) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    /// Set `VideoBuilder::playback_url`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn playback_url(mut self, playback_url: String) -> Self {
        self.playback_url = Some(playback_url);
        self
    }

    /// Set `VideoBuilder::title`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set `VideoBuilder::r#type`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn r#type(mut self, r#type: VideoType) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

impl Display for VideoBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VideoBuilder {{
                id: {},
                title: {},
                description: {},
                expiration_date: {},
                playback_url: {},
                type: {},
                assets: {}
            }}",
            self.id,
            option_to_string(&self.title),
            option_to_string(&self.description),
            option_to_string(&self.expiration_date),
            option_to_string(&self.playback_url),
            option_to_string(&self.r#type),
            array_to_string(&self.assets),
        )
    }
}

/* ****************************************** VideoMap ****************************************** */

/// Type alias for a [`HashMap`] of [`u32`] to [`Vec`]`<`[`Video`]`>`.  
pub type VideoMap = HashMap<u32, Vec<Video>>;

/* **************************************** VideoService **************************************** */

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
        service::video::{AssetReference, Video, VideoMap},
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
