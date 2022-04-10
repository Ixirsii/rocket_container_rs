//! Controller layer type definitions.

use crate::types::{AssetType, VideoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisement {
    /// Unique advertisement identifier.
    id: u32,
    /// Name of advertisement.
    name: String,
    /// Advertisement playback url.
    url: String,
}

impl Advertisement {
    /// Construct a new Advertisement.
    pub fn new(id: u32, name: String, url: String) -> Self {
        Advertisement { id, name, url }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReference {
    /// Unique identifier for referenced asset.
    asset_id: u32,
    /// Type of asset.
    asset_type: AssetType,
}

impl AssetReference {
    /// Construct a new AsssetReference.
    pub fn new(asset_id: u32, asset_type: AssetType) -> Self {
        AssetReference {
            asset_id,
            asset_type,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Unique image identifier.
    id: u32,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

impl Image {
    /// Construct a new Image.
    pub fn new(id: u32, name: String, url: String) -> Self {
        Image { id, name, url }
    }
}

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

    pub fn builder(id: u32) -> VideoBuilder {
        VideoBuilder::new(id)
    }

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

/// Builder class for [Video].
pub struct VideoBuilder {
    /// See [Video::assets].
    /// Initialized to empty [Vec].
    assets: Vec<AssetReference>,
    /// See [Video::description].
    /// Initialized to [None].
    description: Option<String>,
    /// See [Video::expiration_date].
    /// Initialized to [None].
    expiration_date: Option<String>,
    /// See [Video::id].
    /// Required value.
    id: u32,
    /// See [Video::playback_url].
    /// Initialized to [None].
    playback_url: Option<String>,
    /// See [Video::title].
    /// Initialized to [None].
    title: Option<String>,
    /// See [Video::r#type].
    /// Initialized to [None].
    r#type: Option<VideoType>,
}

impl VideoBuilder {
    /// Construct a new VideoBuilder.
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

    ///
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

    pub fn asset(mut self, asset: AssetReference) -> Self {
        self.assets.push(asset);
        self
    }

    pub fn assets(mut self, assets: Vec<AssetReference>) -> Self {
        self.assets = assets;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn expiration_date(mut self, expiration_date: String) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    pub fn playback_url(mut self, playback_url: String) -> Self {
        self.playback_url = Some(playback_url);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn r#type(mut self, r#type: VideoType) -> Self {
        self.r#type = Some(r#type);
        self
    }
}
