//! Repository layer which makes calls to dependent services.
//!
//! Rocket Stream depends on 3 services: Rocket Advertisement, Rocket Image, and Rocket Video.
//! The repository layer makes requests to these dependencies (with exponential backoff and retry
//! on failed requests) and deserializes the data so it can be aggregated into containers.

extern crate reqwest;

use crate::types::{AssetType, Error, ErrorKind, VideoType};
use rand::{thread_rng, Rng};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::min;
use std::future::Future;
use std::thread;
use std::time::Duration;

/* ***************************************** Constants ****************************************** */

/// Endpoint for Rocket Advertisement service.
const ADVERTISEMENT_ENDPOINT: &str =
    "http://ads.rocket-stream.bottlerocketservices.com/advertisements";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Maximum number of retries when a service call fails.
const MAX_ATTEMPTS: u32 = 10;

/// Maximum backoff delay when retrying a service call.
const MAX_BACKOFF: u64 = 1_000;

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

/// Alias for [core::result::Result] where the error type is always [Error]<[reqwest::Error]>.
pub type Result<T> = core::result::Result<T, Error>;

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
trait Wrapper<T> {
    fn unwrap(self) -> Vec<T>;
}

/// [Wrapper] for [Advertisement]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Advertisements {
    advertisements: Vec<Advertisement>,
}

/// [Wrapper] for [Image]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Images {
    images: Vec<Image>,
}

/// [Wrapper] for [Video]s.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Videos {
    videos: Vec<Video>,
}

/* ***************************************** Functions ****************************************** */

/// List all advertisements from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use crate::repository::list_advertisements;
/// use reqwest::Client;
///
/// fn main() {
///     let client = Client::new();
///
///     match list_all_advertisements(&client) {
///         Ok(advertisements) => println!("Got advertisements: {:?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
/// }
/// ```
pub async fn list_all_advertisements(client: &Client) -> Result<Vec<Advertisement>> {
    get::<Advertisement, Advertisements, ()>(client, ADVERTISEMENT_ENDPOINT, None).await
}

/// List advertisements for a container from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use crate::repository::list_advertisements;
/// use reqwest::Client;
///
/// fn main() {
///     let client = Client::new();
///     let container_id = 0;
///
///     match list_advertisements(&client, container_id) {
///         Ok(advertisements) => println!("Got advertisements: {:?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
/// }
/// ```
pub async fn list_advertisements(client: &Client, container_id: u32) -> Result<Vec<Advertisement>> {
    get::<Advertisement, Advertisements, [(&str, u32); 1]>(
        client,
        ADVERTISEMENT_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

/// Make a GET request with exponential backoff and retries on request failures.
///
/// # Examples
///
/// ```rust
/// use reqwest::Client;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// pub struct Advertisement {
///     container_id: u64,
///     id: u64,
///     name: String,
///     url: String,
/// }
///
/// #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
/// struct Advertisements {
///     advertisements: Vec<Advertisement>,
/// }
///
/// pub fn list_advertisements() -> Result<Vec<Advertisement>> {
///     let client = Client::new();
///
///     get::<Advertisement, Advertisements, ()>(
///         &client,
///         "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///         None,
///     )
/// }
///
/// pub fn list_advertisements() -> Result<Vec<Advertisement>> {
///     let client = Client::new();
///     let container_id = 0;
///
///     get::<Advertisement, Advertisements, [(&str, u32); 1]>(
///         &client,
///         "http://ads.rocket-stream.bottlerocketservices.com/advertisements",
///         Some([("containerId", container_id)]),
///     )
/// }
/// ```
async fn get<T, W, Q>(client: &Client, endpoint: &str, query: Option<Q>) -> Result<Vec<T>>
where
    W: Wrapper<T> + for<'de> Deserialize<'de>,
    Q: Serialize,
{
    let op = || async {
        let mut request_builder: RequestBuilder = client.get(endpoint);

        if query.is_some() {
            request_builder = request_builder.query(query.borrow());
        }

        let response: Response = match request_builder.send().await {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    response
                } else if response.status() == StatusCode::NOT_FOUND {
                    return Err(Error::new(ErrorKind::Permanent, "Resource not found"));
                } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(Error::new(ErrorKind::Transient, "Internal server error"));
                } else {
                    return Err(Error::new(ErrorKind::Permanent, "Unexpected error"));
                }
            }
            Err(err) => return Err(Error::new(ErrorKind::Permanent, &err.to_string())),
        };

        match response.json::<W>().await {
            Ok(result) => Ok(result.unwrap()),
            Err(err) => Err(Error::new(ErrorKind::Permanent, &err.to_string())),
        }
    };

    retry(op).await
}

/// Get backoff/delay to wait before the next retry attempt.
fn get_backoff(attempt: u32) -> u64 {
    const BASE: u64 = 2;
    let exponential_backoff: u64 = BASE.pow(attempt);
    let random_number_milliseconds: u64 = thread_rng().gen_range(0..100);
    let backoff: u64 = exponential_backoff + random_number_milliseconds;

    min(backoff, MAX_BACKOFF)
}

/// Calls a function and retries if the function fails.
async fn retry<I, F, Fut>(mut f: F) -> Result<I>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<I>>,
{
    for i in 1..MAX_ATTEMPTS {
        match f().await {
            Ok(data) => return Ok(data),
            Err(err) => {
                if err.kind == ErrorKind::Permanent {
                    return Err(err);
                }
            }
        }

        let backoff: u64 = get_backoff(i);
        thread::sleep(Duration::from_millis(backoff));
    }

    return f().await;
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
    use super::{
        list_advertisements, list_all_advertisements, Advertisement, Advertisements,
        AssetReference, Image, Images, Result, Video, Videos,
    };
    use crate::types::{AssetType, VideoType};
    use reqwest::Client;

    /* ******************************** list_advertisements ********************************* */

    #[tokio::test]
    async fn test_list_advertisements() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<Advertisement>> = list_advertisements(&client, container_id).await;

        // Then
        match result {
            Ok(advertisements) => assert!(!advertisements.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_all_advertisements() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<Advertisement>> = list_all_advertisements(&client).await;

        // Then
        match result {
            Ok(advertisements) => assert!(!advertisements.is_empty()),
            Err(err) => panic!("Failed to list all advertisements with error: {:?}", err),
        }
    }

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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }
}
