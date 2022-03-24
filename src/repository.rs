//! Repository layer which makes calls to dependent services.
//!
//! Rocket Stream depends on 3 services: Rocket Advertisement, Rocket Image, and Rocket Video.
//! The repository layer makes requests to these dependencies (with exponential backoff and retry
//! on failed requests) and deserializes the data so it can be aggregated into containers.

extern crate reqwest;

use crate::types::{AssetType, VideoType};
use backoff::{retry, Error, ExponentialBackoff};
use reqwest::blocking::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

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
    container_id: u32,
    /// Unique advertisement identifier.
    id: u32,
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
    asset_id: u32,
    /// Type of asset.
    asset_type: AssetType,
    /// Unique identifier for referenced video.
    video_id: u32,
}

/// Image data returned from Rocket Image service.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Parent container e.g. show/series identifier.
    container_id: u32,
    /// Unique image identifier.
    id: u32,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

/// Alias for [core::result::Result] where the error type is always [Error]<[reqwest::Error]>.
pub type Result<T> = core::result::Result<T, Error<reqwest::Error>>;

/// Video data returned from Rocket Video service.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    /// Parent container e.g. show/series identifier.
    container_id: u32,
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
/// use reqwest::blocking::Client;
///
/// fn main() {
///     let client = Client::new();
///
///     match list_advertisements(&client) {
///         Ok(advertisements) => println!("Got advertisements: {:?}", advertisements),
///         Err(_) => println!("Failed to get advertisements"),
///     };
/// }
/// ```
pub fn list_advertisements(client: &Client) -> Result<Vec<Advertisement>> {
    get::<Advertisement, Advertisements, ()>(client, ADVERTISEMENT_ENDPOINT, None)
}

/// List advertisements for a container from Rocket Advertisement.
///
/// # Examples
///
/// ```rust
/// use crate::repository::list_advertisements;
/// use reqwest::blocking::Client;
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
pub fn list_advertisement(client: &Client, container_id: u32) -> Result<Vec<Advertisement>> {
    get::<Advertisement, Advertisements, [(&str, u32); 1]>(
        client,
        ADVERTISEMENT_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
}

/// Make a GET request with exponential backoff and retries on request failures.
///
/// # Examples
///
/// ```rust
/// use reqwest::blocking::Client;
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
fn get<T, W, Q>(client: &Client, endpoint: &str, query: Option<Q>) -> Result<Vec<T>>
where
    W: Wrapper<T> + for<'de> Deserialize<'de>,
    Q: Serialize,
{
    let op = || {
        let mut request_builder: RequestBuilder = client.get(endpoint);

        if query.is_some() {
            request_builder = request_builder.query(query.borrow());
        }

        let result: Vec<T> = request_builder
            .send()?
            .json::<W>()
            .map_err(Error::Permanent)?
            .unwrap();

        Ok(result)
    };

    retry(ExponentialBackoff::default(), op)
}

/* ************************************** Implementations *************************************** */

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
    use crate::types::VideoType;

    #[test]
    fn deserialize_advertisement() {
        // Given
        let data = r#"
            {
                "containerId": 0,
                "id": 0,
                "name": "Advertisement",
                "url": "https://advertisement.com"
            }
        "#;

        let expected = Advertisement {
            container_id: 0,
            id: 0,
            name: "Advertisement".to_string(),
            url: "https://advertisement.com".to_string(),
        };

        // When
        let actual: serde_json::Result<Advertisement> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn deserialize_advertisements() {
        // Given
        let data = r#"
            {
                "advertisements": [
                    {
                        "containerId": 0,
                        "id": 0,
                        "name": "Advertisement",
                        "url": "https://advertisement.com"
                    }
                ]
            }
        "#;

        let expected = Advertisements {
            advertisements: Vec::<Advertisement>::from([Advertisement {
                container_id: 0,
                id: 0,
                name: "Advertisement".to_string(),
                url: "https://advertisement.com".to_string(),
            }]),
        };

        // When
        let actual: serde_json::Result<Advertisements> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn deserialize_image() {
        // Given
        let data = r#"
            {
                "containerId": 0,
                "id": 0,
                "name": "Image",
                "url": "https://image.com"
            }
        "#;

        let expected = Image {
            container_id: 0,
            id: 0,
            name: "Image".to_string(),
            url: "https://image.com".to_string(),
        };

        // When
        let actual: serde_json::Result<Image> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn deserialize_images() {
        // Given
        let data = r#"
            {
                "images": [
                    {
                        "containerId": 0,
                        "id": 0,
                        "name": "Image",
                        "url": "https://image.com"
                    }
                ]
            }
        "#;

        let expected = Images {
            images: Vec::<Image>::from([Image {
                container_id: 0,
                id: 0,
                name: "Image".to_string(),
                url: "https://image.com".to_string(),
            }]),
        };

        // When
        let actual: serde_json::Result<Images> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn deserialize_video() {
        // Given
        let data = r#"
            {
                "containerId": 0,
                "description": "A short video clip",
                "expirationDate": "2022-03-23",
                "id": 0,
                "playbackUrl": "https://www.youtube.com/watch?v=00000000000",
                "title": "Video",
                "type": "CLIP"
            }
        "#;

        let expected = Video {
            container_id: 0,
            description: "A short video clip".to_string(),
            expiration_date: "2022-03-23".to_string(),
            id: 0,
            playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
            title: "Video".to_string(),
            r#type: VideoType::CLIP,
        };

        // When
        let actual: serde_json::Result<Video> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn deserialize_videos() {
        // Given
        let data = r#"
            {
                "videos": [
                    {
                        "containerId": 0,
                        "description": "A short video clip",
                        "expirationDate": "2022-03-23",
                        "id": 0,
                        "playbackUrl": "https://www.youtube.com/watch?v=00000000000",
                        "title": "Video",
                        "type": "CLIP"
                    }
                ]
            }
        "#;

        let expected = Videos {
            videos: Vec::<Video>::from([Video {
                container_id: 0,
                description: "A short video clip".to_string(),
                expiration_date: "2022-03-23".to_string(),
                id: 0,
                playback_url: "https://www.youtube.com/watch?v=00000000000".to_string(),
                title: "Video".to_string(),
                r#type: VideoType::CLIP,
            }]),
        };

        // When
        let actual: serde_json::Result<Videos> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(advertisement) => assert_eq!(advertisement, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }
}
