//! Video data transfer object type definitions.

use super::Wrapper;
use crate::controller::types::{AssetReference, Video, VideoBuilder};
use crate::types::{AssetType, VideoType};
use serde::{Deserialize, Serialize};

/* ******************************************* Types ******************************************** */

/// A reference to an asset associated with a [Video].
///
/// [Video]s can have assets associated with them such as promotional images and advertisements.
/// This is a generic pointer to those assets.
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
    pub fn new(asset_id: String, asset_type: AssetType, video_id: String) -> Self {
        AssetReferenceDto {
            asset_id,
            asset_type,
            video_id,
        }
    }
}

impl From<AssetReferenceDto> for AssetReference {
    fn from(asset_reference_dto: AssetReferenceDto) -> Self {
        AssetReference::new(
            asset_reference_dto.asset_id.parse().unwrap(),
            asset_reference_dto.asset_type,
        )
    }
}

/// Video data returned from Rocket Video service.
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

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl From<VideoDto> for VideoBuilder {
    fn from(video_dto: VideoDto) -> VideoBuilder {
        Video::builder(video_dto.id.parse().unwrap())
            .description(video_dto.description)
            .expiration_date(video_dto.expiration_date)
            .playback_url(video_dto.playback_url)
            .title(video_dto.title)
            .r#type(video_dto.r#type)
    }
}

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAssetsDto {
    video_assets: Vec<AssetReferenceDto>,
}

impl VideoAssetsDto {
    /// Construct a new VideoAssets wrapper.
    pub fn new(video_assets: Vec<AssetReferenceDto>) -> Self {
        VideoAssetsDto { video_assets }
    }
}

impl Wrapper<AssetReferenceDto> for VideoAssetsDto {
    /// Unwrap [VideoAssets::video_assets].
    fn unwrap(self) -> Vec<AssetReferenceDto> {
        self.video_assets
    }
}

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideosDto {
    videos: Vec<VideoDto>,
}

impl VideosDto {
    /// Construct a new Videos wrapper.
    pub fn new(videos: Vec<VideoDto>) -> Self {
        VideosDto { videos }
    }
}

impl Wrapper<VideoDto> for VideosDto {
    /// Unwrap [Videos::videos].
    fn unwrap(self) -> Vec<VideoDto> {
        self.videos
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{AssetReferenceDto, VideoDto, VideosDto};
    use crate::types::{AssetType, VideoType};

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
            asset_type: AssetType::AD,
            video_id: 0.to_string(),
        };

        // When
        let result: serde_json::Result<AssetReferenceDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
            r#type: VideoType::CLIP,
        };

        // When
        let result: serde_json::Result<VideoDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
                r#type: VideoType::CLIP,
            }]),
        };

        // When
        let result: serde_json::Result<VideosDto> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn serialize_asset_reference() {
        // Given
        let data: AssetReferenceDto = AssetReferenceDto {
            asset_id: 0.to_string(),
            asset_type: AssetType::AD,
            video_id: 0.to_string(),
        };

        let expected: &str = r#"{"assetId":"0","assetType":"AD","videoId":"0"}"#;

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
            r#type: VideoType::CLIP,
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
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
                r#type: VideoType::CLIP,
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
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }
}
