//! Repository module type definitions.

use crate::types::{AssetType, VideoType};
use serde::{Deserialize, Serialize};

/* ******************************************* Types ******************************************** */

/// Advertisement data returned from Rocket Advertisement service.
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

impl Advertisement {
    /// Constructs a new Advertisement.
    pub fn new(container_id: String, id: String, name: String, url: String) -> Self {
        Advertisement {
            container_id,
            id,
            name,
            url,
        }
    }

    /// Get [Advertisement::container_id].
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Get [Advertisement::id].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get [Advertisement::name].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get [Advertisement::url].
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// [Wrapper] for [Advertisement]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Advertisements {
    advertisements: Vec<Advertisement>,
}

impl Advertisements {
    /// Construct a new Advertisements wrapper.
    pub fn new(advertisements: Vec<Advertisement>) -> Self {
        Advertisements { advertisements }
    }
}

impl Wrapper<Advertisement> for Advertisements {
    /// Unwrap [Advertisements::advertisements].
    fn unwrap(self) -> Vec<Advertisement> {
        self.advertisements
    }
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

impl AssetReference {
    /// Construct a new AssetReference.
    pub fn new(asset_id: String, asset_type: AssetType, video_id: String) -> Self {
        AssetReference {
            asset_id,
            asset_type,
            video_id,
        }
    }

    /// Get [AssetReference::asset_id].
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    /// Get [AssetReference::asset_type].
    pub fn asset_type(&self) -> &AssetType {
        &self.asset_type
    }

    /// Get [AssetReference::video_id].
    pub fn video_id(&self) -> &str {
        &self.video_id
    }
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

impl Image {
    /// Construct a new Image.
    pub fn new(container_id: String, id: String, name: String, url: String) -> Self {
        Image {
            container_id,
            id,
            name,
            url,
        }
    }

    /// Get [Image::container_id].
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Get [Image::id].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get [Image::name].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get [Image::url].
    pub fn url(&self) -> &str {
        &self.url
    }
}

/// [Wrapper] for [Image]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Images {
    images: Vec<Image>,
}

impl Images {
    /// Construct a new Images wrapper.
    pub fn new(images: Vec<Image>) -> Self {
        Images { images }
    }
}

impl Wrapper<Image> for Images {
    /// Unwrap [Images::images].
    fn unwrap(self) -> Vec<Image> {
        self.images
    }
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

impl Video {
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
        Video {
            container_id,
            description,
            expiration_date,
            id,
            playback_url,
            title,
            r#type,
        }
    }

    /// Get [Video::container_id].
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Get [Video::description].
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get [Video::description].
    pub fn expiration_date(&self) -> &str {
        &self.expiration_date
    }

    /// Get [Video::id].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get [Video::playback_url].
    pub fn playback_url(&self) -> &str {
        &self.playback_url
    }

    /// Get [Video::title].
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get [Video::r#type].
    pub fn r#type(&self) -> &VideoType {
        &self.r#type
    }
}

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAssets {
    video_assets: Vec<AssetReference>,
}

impl VideoAssets {
    /// Construct a new VideoAssets wrapper.
    pub fn new(video_assets: Vec<AssetReference>) -> Self {
        VideoAssets { video_assets }
    }
}

impl Wrapper<AssetReference> for VideoAssets {
    /// Unwrap [VideoAssets::video_assets].
    fn unwrap(self) -> Vec<AssetReference> {
        self.video_assets
    }
}

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Videos {
    videos: Vec<Video>,
}

impl Videos {
    /// Construct a new Videos wrapper.
    pub fn new(videos: Vec<Video>) -> Self {
        Videos { videos }
    }
}

impl Wrapper<Video> for Videos {
    /// Unwrap [Videos::videos].
    fn unwrap(self) -> Vec<Video> {
        self.videos
    }
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
