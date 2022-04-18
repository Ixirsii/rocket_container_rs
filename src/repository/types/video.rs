//! Video data transfer object type definitions.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::service::types::video::{AssetReference, Video, VideoBuilder};
use crate::types::{array_to_string, AssetType, VideoType};

/* ************************************* AssetReferenceDto ************************************** */

/// A reference to an asset associated with a [Video].
///
/// [Video]s can have assets associated with them such as promotional images and advertisements.
/// This is a generic pointer to those assets.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use rocket_stream::repository::video::list_asset_references;
/// use rocket_stream::repository::types::video::AssetReferenceDto;
///
/// let client: Client = Client::new();
/// let video_id: u32 = 1404;
/// let advertisements: Vec<AdvertisementDto> = list_asset_references(&client, video_id)
///     .await
///     .unwrap();
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
    /// use rocket_stream::repository::types::video::AssetReferenceDto;
    ///
    /// AssetReferenceDto::new(
    ///     "120".to_string(),
    ///     AssetType::IMAGE,
    ///     "1404".to_string(),
    /// )
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
    /// use reqwest::Client;
    /// use rocket_stream::service::types::video::AssetReference;
    /// use rocket_stream::repository::video::list_asset_references;
    ///
    /// let client: Client = Client::new();
    /// let video_id: u32 = 1404;
    /// let advertisements: Vec<AssetReference> = list_asset_references(&client)
    ///     .await?
    ///     .into_iter()
    ///     .map(AssetReference::from)
    ///     .collect();
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
/// use reqwest::Client;
/// use rocket_stream::repository::video::list_videos;
/// use rocket_stream::repository::types::video::VideoDto;
///
/// let client: Client = Client::new();
/// let advertisements: Vec<AdvertisementDto> = list_videos(&client).await.unwrap();
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
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let expected: VideoDto = VideoDto::new(
    ///     "25".to_string(),
    ///     "Etiam vel augue. Vestibulum rutrum rutrum neque. Aenean auctor gravida sem."
    ///         .to_string(),
    ///     "".to_string(),
    ///     "1301".to_string(),
    ///     "/path/to/test1301.m3u8".to_string(),
    ///     "My Family".to_string(),
    ///     VideoType::CLIP,
    /// );
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

    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Get Video ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_stream::repository::types::video::VideoDto;
    ///
    /// let video: VideoDto = ...;
    /// let video_id: &str = video.id();
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
    /// use reqwest::Client;
    /// use rocket_stream::service::types::video::Video;
    /// use rocket_stream::repository::video::list_videos;
    ///
    /// let client: Client = Client::new();
    /// let videos: Vec<Video> = list_videos(&client)
    ///     .await?
    ///     .into_iter()
    ///     .map(Video::from)
    ///     .collect();
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
/// use reqwest::Client;
/// use rocket_stream::repository::request;
/// use rocket_stream::repository::types::advertisement::{AssetReferenceDto, VideoAssetsDto};
///
/// let advertisements: Vec<AssetReferenceDto> =
///     request::<VideoAssetsDto, ()>(client, VIDEO_ENDPOINT, None)
///         .await?
///         .video_assets;
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAssetsDto {
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
/// use reqwest::Client;
/// use rocket_stream::repository::request;
/// use rocket_stream::repository::types::advertisement::{VideoDto, VideosDto};
///
/// let advertisements: Vec<VideoDto> =
///     request::<VideosDto, ()>(client, VIDEO_ENDPOINT, None)
///         .await?
///         .videos;
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideosDto {
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

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::types::{AssetType, VideoType};

    use super::{AssetReferenceDto, VideoDto, VideosDto};

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
}
