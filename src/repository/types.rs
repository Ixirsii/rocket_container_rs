//! Repository module type definitions.

use crate::types::{AssetType, VideoType};
use serde::{Deserialize, Serialize};

/* ******************************************* Types ******************************************** */

/// Advertisement data returned from Rocket Advertisement service.`
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisement {
    /// Parent container e.g. show/series identifier.
    container_id: String,
    /// Unique advertisement identifier.
    id: String,
    /// Name of advertisement.
    name: String,
    /// Advertisement playback url.
    url: String,
}

/// [Wrapper] for [Advertisement]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advertisements {
    advertisements: Vec<Advertisement>,
}

/// A reference to an asset associated with a [Video].
///
/// [Video]s can have assets associated with them such as promotional images and advertisements.
/// This is a generic pointer to those assets.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReference {
    /// Unique identifier for referenced asset.
    asset_id: String,
    /// Type of asset.
    asset_type: AssetType,
    /// Unique identifier for referenced video.
    video_id: String,
}

/// Image data returned from Rocket Image service.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Parent container e.g. show/series identifier.
    container_id: String,
    /// Unique image identifier.
    id: String,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

/// [Wrapper] for [Image]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Images {
    images: Vec<Image>,
}

/// Video data returned from Rocket Video service.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
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

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Videos {
    videos: Vec<Video>,
}

/// Trait for objects which wrap other objects.
///
/// Rocket Stream's dependencies return lists wrapped in an object.
/// For example, Rocket Advertisement's `listAdvertisements` API returns
///
/// ```json
/// {
///   advertisements: [
///     {
///       container_id: number,
///       id: number,
///       name: string,
///       url: string
///     }
///   ]
/// }
/// ```
///
/// The `advertisements` field is modeled by [Advertisements] which implements this trait so that
/// it can be unwrapped to [Vec]<[Advertisement]>.
///
/// # Examples
///
/// ```rust
/// struct Advertisements {
///     advertisements: Vec<Advertisement>,
/// }
///
/// impl Wrapper<Advertisement> for Advertisements {
///     fn unwrap(self) -> Vec<Advertisement> {
///         self.advertisements
///     }
/// }
/// ```
pub trait Wrapper<T> {
    fn unwrap(self) -> Vec<T>;
}

/* *************************************** Implementation *************************************** */

impl Wrapper<Advertisement> for Advertisements {
    fn unwrap(self) -> Vec<Advertisement> {
        self.advertisements
    }
}

impl Wrapper<Image> for Images {
    fn unwrap(self) -> Vec<Image> {
        self.images
    }
}

impl Wrapper<Video> for Videos {
    fn unwrap(self) -> Vec<Video> {
        self.videos
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::{Advertisement, Advertisements, AssetReference, Image, Images, Video, Videos};
    use crate::types::{AssetType, VideoType};

    /* ******************************* Deserialization tests ******************************** */

    #[test]
    fn deserialize_advertisement() {
        // Given
        let data: &str = r#"
            {
                "containerId": "0",
                "id": "0",
                "name": "Advertisement",
                "url": "https://advertisement.com"
            }
        "#;

        let expected: Advertisement = Advertisement {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Advertisement".to_string(),
            url: "https://advertisement.com".to_string(),
        };

        // When
        let result: serde_json::Result<Advertisement> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn deserialize_advertisements() {
        // Given
        let data: &str = r#"
            {
                "advertisements": [
                    {
                        "containerId": "0",
                        "id": "0",
                        "name": "Advertisement",
                        "url": "https://advertisement.com"
                    }
                ]
            }
        "#;

        let expected: Advertisements = Advertisements {
            advertisements: Vec::from([Advertisement {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Advertisement".to_string(),
                url: "https://advertisement.com".to_string(),
            }]),
        };

        // When
        let result: serde_json::Result<Advertisements> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

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

        let expected: AssetReference = AssetReference {
            asset_id: 0.to_string(),
            asset_type: AssetType::AD,
            video_id: 0.to_string(),
        };

        // When
        let result: serde_json::Result<AssetReference> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn deserialize_image() {
        // Given
        let data: &str = r#"
            {
                "containerId": "0",
                "id": "0",
                "name": "Image",
                "url": "https://image.com"
            }
        "#;

        let expected: Image = Image {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Image".to_string(),
            url: "https://image.com".to_string(),
        };

        // When
        let result: serde_json::Result<Image> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn deserialize_images() {
        // Given
        let data: &str = r#"
            {
                "images": [
                    {
                        "containerId": "0",
                        "id": "0",
                        "name": "Image",
                        "url": "https://image.com"
                    }
                ]
            }
        "#;

        let expected: Images = Images {
            images: Vec::from([Image {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Image".to_string(),
                url: "https://image.com".to_string(),
            }]),
        };

        // When
        let result: serde_json::Result<Images> = serde_json::from_str(data);

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

        let expected: Video = Video {
            container_id: 0.to_string(),
            description: "A short video clip".to_string(),
            expiration_date: "2022-03-23".to_string(),
            id: 0.to_string(),
            playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
            title: "Video".to_string(),
            r#type: VideoType::CLIP,
        };

        // When
        let result: serde_json::Result<Video> = serde_json::from_str(data);

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

        let expected: Videos = Videos {
            videos: Vec::from([Video {
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
        let result: serde_json::Result<Videos> = serde_json::from_str(data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    /* ******************************** Serialization tests ********************************* */

    #[test]
    fn serialize_advertisement() {
        // Given
        let data: Advertisement = Advertisement {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Advertisement".to_string(),
            url: "https://advertisement.com".to_string(),
        };

        let expected: &str = "\
            {\
                \"containerId\":\"0\",\
                \"id\":\"0\",\
                \"name\":\"Advertisement\",\
                \"url\":\"https://advertisement.com\"\
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
    fn serialize_advertisements() {
        // Given
        let data: Advertisements = Advertisements {
            advertisements: Vec::from([Advertisement {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Advertisement".to_string(),
                url: "https://advertisement.com".to_string(),
            }]),
        };

        let expected: &str = "\
            {\
                \"advertisements\":[\
                    {\
                        \"containerId\":\"0\",\
                        \"id\":\"0\",\
                        \"name\":\"Advertisement\",\
                        \"url\":\"https://advertisement.com\"\
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

    #[test]
    fn serialize_asset_reference() {
        // Given
        let data: AssetReference = AssetReference {
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
    fn serialize_image() {
        // Given
        let data: Image = Image {
            container_id: 0.to_string(),
            id: 0.to_string(),
            name: "Image".to_string(),
            url: "https://image.com".to_string(),
        };

        let expected: &str =
            r#"{"containerId":"0","id":"0","name":"Image","url":"https://image.com"}"#;

        // When
        let result: serde_json::Result<String> = serde_json::to_string(&data);

        // Then
        match result {
            Ok(actual) => assert_eq!(actual, expected),
            Err(err) => panic!("Failed to deserialize with error: {:#?}", err),
        }
    }

    #[test]
    fn serialize_images() {
        // Given
        let data: Images = Images {
            images: Vec::from([Image {
                container_id: 0.to_string(),
                id: 0.to_string(),
                name: "Image".to_string(),
                url: "https://image.com".to_string(),
            }]),
        };

        let expected: &str =
            r#"{"images":[{"containerId":"0","id":"0","name":"Image","url":"https://image.com"}]}"#;

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
        let data: Video = Video {
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
        let data: Videos = Videos {
            videos: Vec::from([Video {
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
