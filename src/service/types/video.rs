//! Video domain type definition.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::types::{array_to_string, option_to_string, AssetType, VideoType};

/// Asset reference returned from container service.
///
/// Container service returns a variant of [`AssetReferenceDto`][1] with `id` field as a number and
/// without `container_id` field. [`AssetReferenceDto`][1]s returned from
/// [`video repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::types::video::AssetReference;
///
/// let asset_reference = AssetReference::new(1, AssetType::Video);
/// ```
///
/// ```rust
/// use rocket_stream::service::types::video::AssetReference;
///
/// let asset_reference_dto: AssetReferenceDto = ...;
/// let asset_reference = AssetReference::from(advertisement_dto);
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
    /// use rocket_stream::service::types::video::AssetReference;
    ///
    /// let asset_reference = AssetReference::new(1, AssetType::Video);
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

/// Video asset returned from container service.
///
/// Container service returns a variant of [`VideoDto`][1] with `id` field as a number and
/// without `container_id` field. [`VideoDto`][1]s returned from
/// [`video repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::types::VideoType;
/// use rocket_stream::service::types::video::Video;
///
/// let video = Video::new(
///     Vec::new(),
///     "A Rust tutorial!".to_string(),
///     "2023-04-18".to_string(),
///     1,
///     "https://video.com/video.mp4".to_string(),
///     "How to implement Rocket Stream in Rust".to_string(),
///     VideoType::EPISODE,
/// );
/// ```
///
/// ```rust
/// use rocket_stream::types::VideoType;
/// use rocket_stream::service::types::video::Video;
///
/// let assets: Vec<AssetReference> = ...;
/// let video = Video::builder(1)
///     .assets(assets)
///     .description("A Rust tutorial!".to_string())
///     .expiration_date("2023-04-18".to_string())
///     .playback_url("https://video.com/video.mp4".to_string())
///     .title("How to implement Rocket Stream in Rust".to_string())
///     .r#type(VideoType::EPISODE)
///     .build();
/// ```
///
/// ```rust
/// use rocket_stream::service::types::video::Video;
/// use rocket_stream::repository::types::video::VideoDto;
///
/// let video_dto: VideoDto = ...;
/// let assets: Vec<AssetReference> = ...;
/// let video = Video::from(video_dto);
///     .assets(assets)
///     .build();
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
    /// use rocket_stream::types::VideoType;
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let video = Video::new(
    ///     Vec::new(),
    ///     "A Rust tutorial!".to_string(),
    ///     "2023-04-18".to_string(),
    ///     1,
    ///     "https://video.com/video.mp4".to_string(),
    ///     "How to implement Rocket Stream in Rust".to_string(),
    ///     VideoType::EPISODE,
    /// );
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
    /// use rocket_stream::types::VideoType;
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let assets: Vec<AssetReference> = ...;
    /// let video = Video::builder(1)
    ///     .assets(assets)
    ///     .description("A Rust tutorial!".to_string())
    ///     .expiration_date("2023-04-18".to_string())
    ///     .playback_url("https://video.com/video.mp4".to_string())
    ///     .title("How to implement Rocket Stream in Rust".to_string())
    ///     .r#type(VideoType::EPISODE)
    ///     .build();
    /// ```
    pub fn builder(id: u32) -> VideoBuilder {
        VideoBuilder::new(id)
    }

    /// Get a [VideoBuilder] with values initialized from this [Video].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let video_without_assets: Video = ...;
    /// let asset: AssetReference = ...;
    /// let video_with_assets = video_without_assets.to_builder()
    ///     .asset(asset)
    ///     .build();
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

/// Builder class for [Video].
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::types::video::Video;
/// use rocket_stream::repository::types::video::VideoDto;
///
/// let video_dto: VideoDto = ...;
/// let assets: Vec<AssetReference> = ...;
/// let video = Video::from(video_dto);
///     .assets(assets)
///     .build();
/// ```
///
/// ```rust
/// use rocket_stream::types::VideoType;
/// use rocket_stream::service::types::video::Video;
///
/// let assets: Vec<AssetReference> = ...;
/// let video = Video::builder(1)
///     .assets(assets)
///     .description("A Rust tutorial!".to_string())
///     .expiration_date("2023-04-18".to_string())
///     .playback_url("https://video.com/video.mp4".to_string())
///     .title("How to implement Rocket Stream in Rust".to_string())
///     .r#type(VideoType::EPISODE)
///     .build();
/// ```
///
/// ```rust
/// use rocket_stream::types::VideoType;
/// use rocket_stream::service::types::video::Video;
///
/// let assets: Vec<AssetReference> = ...;
/// let video = VideoBuilder::new(1)
///     .assets(assets)
///     .description("A Rust tutorial!".to_string())
///     .expiration_date("2023-04-18".to_string())
///     .playback_url("https://video.com/video.mp4".to_string())
///     .title("How to implement Rocket Stream in Rust".to_string())
///     .r#type(VideoType::EPISODE)
///     .build();
/// ```
///
/// ```rust
/// use rocket_stream::service::types::video::Video;
///
/// let video_without_assets: Video = ...;
/// let asset: AssetReference = ...;
/// let video_with_assets = video_without_assets.to_builder()
///     .asset(asset)
///     .build();
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
    /// use rocket_stream::types::VideoType;
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let assets: Vec<AssetReference> = ...;
    /// let video = VideoBuilder::new(1)
    ///     .assets(assets)
    ///     .description("A Rust tutorial!".to_string())
    ///     .expiration_date("2023-04-18".to_string())
    ///     .playback_url("https://video.com/video.mp4".to_string())
    ///     .title("How to implement Rocket Stream in Rust".to_string())
    ///     .r#type(VideoType::EPISODE)
    ///     .build();
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
    /// use rocket_stream::types::VideoType;
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let assets: Vec<AssetReference> = ...;
    /// let video = VideoBuilder::new(1)
    ///     .assets(assets)
    ///     .description("A Rust tutorial!".to_string())
    ///     .expiration_date("2023-04-18".to_string())
    ///     .playback_url("https://video.com/video.mp4".to_string())
    ///     .title("How to implement Rocket Stream in Rust".to_string())
    ///     .r#type(VideoType::EPISODE)
    ///     .build();
    /// ```
    pub fn build(mut self) -> Video {
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
    /// use rocket_stream::types::VideoType;
    /// use rocket_stream::service::types::video::Video;
    ///
    /// let assets: Vec<AssetReference> = ...;
    /// let video_builder = VideoBuilder::new(1)
    ///     .assets(assets)
    ///     .description("A Rust tutorial!".to_string())
    ///     .expiration_date("2023-04-18".to_string())
    ///     .playback_url("https://video.com/video.mp4".to_string())
    ///     .title("How to implement Rocket Stream in Rust".to_string())
    ///     .r#type(VideoType::EPISODE);
    ///
    /// let video1: Video = video_builder.build_clone();
    ///
    /// video_builder.asset(asset);
    ///
    /// let video2: Video = video_builder.build_clone();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let asset: AssetReference = ...;
    /// let video = Video::from(video_dto)
    ///     .asset(asset)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let assets: Vec<AssetReference> = ...;
    /// let video = Video::from(video_dto)
    ///     .assets(assets)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let description: String = ...;
    /// let video = Video::from(video_dto)
    ///     .description(description)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let expiration_date: String = ...;
    /// let video = Video::from(video_dto)
    ///     .expiration_date(expiration_date)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let playback_url: String = ...;
    /// let video = Video::from(video_dto)
    ///     .playback_url(playback_url)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let title: String = ...;
    /// let video = Video::from(video_dto)
    ///     .title(title)
    ///     .build();
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
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video_dto: VideoDto = ...;
    /// let r#type: VideoType = ...;
    /// let video = Video::from(video_dto)
    ///     .r#type(r#type)
    ///     .build();
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
