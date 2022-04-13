//! Public crate type definitions.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/* ******************************************* Types ******************************************** */

/// Type of asset an `AssetReference` links too.
///
/// `AssetReference` is a generic container which points to "assets" associated with a video.
/// Those assets can be either advertisements or images, and the type of asset is tracked by the
/// types of this enum.
///
/// # Examples
///
/// ```rust
/// AssetReferenceDto::new(
///     "120".to_string(),
///     AssetType::Image,
///     "1404".to_string(),
/// )
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetType {
    /// Advertisement asset.
    Ad,
    /// Image asset.
    Image,
}

/// Internal error type.
///
/// # Examples
///
/// ```rust
/// use reqwest::{RequestBuilder, StatusCode};
///
/// let request_builder: RequestBuilder = ...;
///
/// match request_builder.send().await {
///     Ok(response) => {
///         if response.status() == StatusCode::OK {
///             Ok(response)
///         } else if response.status() == StatusCode::NOT_FOUND {
///             Err(Error::new(ErrorKind::Permanent, "Resource not found"))
///         } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
///             Err(Error::new(ErrorKind::Transient, "Internal server error"))
///         } else {
///             Err(Error::new(ErrorKind::Permanent, "Unexpected error"))
///         }
///     }
///     Err(err) => return Err(Error::new(ErrorKind::Permanent, &err.to_string())),
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

/// Type of [Error] (whether the error is retryable or not).
///
/// # Examples
///
/// ```rust
/// use rocket_stream::types::{Error, ErrorKind};
///
/// Error::new(ErrorKind::Permanent, "Unexpected error")
/// ```
///
/// ```rust
/// use rocket_stream::types::{Error, ErrorKind};
///
/// Error::new(ErrorKind::Transient, "Internal server error")
/// ```
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Permanent,
    Transient,
}

/// Alias for [core::result::Result] where the error type is always [Error].
pub type Result<T> = core::result::Result<T, Error>;

/// Type of `Video`
///
/// Videos can be either short clips, TV length episodes, or full length movies, and the type of
/// video is tracked by the types of this enum.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VideoType {
    /// A short clip.
    Clip,
    /// A TV length episode.
    Episode,
    /// A full length movie.
    Movie,
}

// #[derive(Debug)]
// pub struct Container {
//     ads: Vec<Advertisement>,
//     id: u32,
//     images: Vec<Image>,
//     title: String,
//     videos: Vec<Video>
// }

/* *************************************** Implementation *************************************** */

impl Error {
    /// Construct a new Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_stream::types::{Error, ErrorKind};
    ///
    /// Error::new(ErrorKind::Permanent, "Unexpected error")
    /// ```
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: message.to_string(),
        }
    }
}

impl Display for AssetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Ad => write!(f, "Ad"),
            AssetType::Image => write!(f, "Image"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"(kind: {}, message: {})"#, self.kind, self.message,)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Permanent => write!(f, "Permanent"),
            ErrorKind::Transient => write!(f, "Transient"),
        }
    }
}

impl Display for VideoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoType::Clip => write!(f, "Clip"),
            VideoType::Episode => write!(f, "Episode"),
            VideoType::Movie => write!(f, "Movie"),
        }
    }
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::AssetType;
    use super::VideoType;

    #[test]
    fn deserialize_asset_type_ad() {
        // Given
        let data: &str = r#""AD""#;

        // When
        let actual: serde_json::Result<AssetType> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(asset_type) => assert_eq!(asset_type, AssetType::Ad),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_asset_type_image() {
        // Given
        let data: &str = r#""IMAGE""#;

        // When
        let actual: serde_json::Result<AssetType> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(asset_type) => assert_eq!(asset_type, AssetType::Image),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_video_type_clip() {
        // Given
        let data: &str = r#""CLIP""#;

        // When
        let actual: serde_json::Result<VideoType> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(video_type) => assert_eq!(video_type, VideoType::Clip),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_video_type_episode() {
        // Given
        let data: &str = r#""EPISODE""#;

        // When
        let actual: serde_json::Result<VideoType> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(video_type) => assert_eq!(video_type, VideoType::Episode),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn deserialize_video_type_movie() {
        // Given
        let data: &str = r#""MOVIE""#;

        // When
        let actual: serde_json::Result<VideoType> = serde_json::from_str(data);

        // Then
        match actual {
            Ok(video_type) => assert_eq!(video_type, VideoType::Movie),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_asset_type_ad() {
        // Given
        let expected: String = String::from(r#""AD""#);

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&AssetType::Ad);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_asset_type_image() {
        // Given
        let expected: String = String::from(r#""IMAGE""#);

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&AssetType::Image);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_video_type_clip() {
        // Given
        let expected: String = String::from(r#""CLIP""#);

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::Clip);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_video_type_episode() {
        // Given
        let expected: String = String::from(r#""EPISODE""#);

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::Episode);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }

    #[test]
    fn serialize_video_type_movie() {
        // Given
        let expected: String = String::from(r#""MOVIE""#);

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::Movie);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {}", err),
        }
    }
}
