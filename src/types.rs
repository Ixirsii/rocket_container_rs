//! Public crate type definitions.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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
/// use rocket_container::{repository::video::AssetReferenceDto, types::AssetType};
///
/// let asset_reference: AssetReferenceDto = AssetReferenceDto::new(
///     "120".to_string(),
///     AssetType::Image,
///     "1404".to_string(),
/// );
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetType {
    /// Advertisement asset.
    Ad,
    /// Image asset.
    Image,
}

impl Display for AssetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Ad => write!(f, "AD"),
            AssetType::Image => write!(f, "IMAGE"),
        }
    }
}

/// Internal error type.
///
/// # Examples
///
/// ```rust
/// use reqwest::{RequestBuilder, Response, StatusCode};
/// use rocket_container::types::{Error, ErrorKind};
///
/// async fn send(request_builder: RequestBuilder) -> Result<Response, Error> {
///     match request_builder.send().await {
///         Ok(response) => {
///             if response.status() == StatusCode::OK {
///                 Ok(response)
///             } else if response.status() == StatusCode::NOT_FOUND {
///                 Err(Error {
///                     kind: ErrorKind::Permanent,
///                     message: "Resource not found".to_string()
///                 })
///             } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
///                 Err(Error {
///                     kind: ErrorKind::Transient,
///                     message: "Internal server error".to_string()
///                 })
///             } else {
///                 Err(Error {
///                     kind: ErrorKind::Permanent,
///                     message: "Unexpected error".to_string()
///                 })
///             }
///         }
///         Err(err) => Err(Error { kind: ErrorKind::Permanent, message: err.to_string() }),
///     }
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    /// If the error is permanent or transient.
    pub kind: ErrorKind,
    /// Error message.
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {{ kind: {}, message: {} }}",
            self.kind, self.message,
        )
    }
}

/// Type of [Error] (whether the error is retryable or not).
///
/// # Examples
///
/// ```rust
/// use rocket_container::types::{Error, ErrorKind};
///
/// let error: Error = Error {
///     kind: ErrorKind::Permanent,
///     message: "Unexpected error".to_string()
/// };
/// ```
///
/// ```rust
/// use rocket_container::types::{Error, ErrorKind};
///
/// let error: Error = Error {
///     kind: ErrorKind::Transient,
///     message: "Internal server error".to_string()
/// };
/// ```
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// A permanent, non-retryable error.
    Permanent,
    /// A transient, retryable error.
    Transient,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Permanent => write!(f, "Permanent"),
            ErrorKind::Transient => write!(f, "Transient"),
        }
    }
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

impl Display for VideoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoType::Clip => write!(f, "CLIP"),
            VideoType::Episode => write!(f, "EPISODE"),
            VideoType::Movie => write!(f, "MOVIE"),
        }
    }
}

/* ************************************** Utility function ************************************** */

/// Convert an [Option] to a [String].
///
///# Examples
///
///```rust
/// use rocket_container::types::option_to_string;
///
/// let option: Option<String> = Some("Hello".to_string());
/// let option_str: String = option_to_string(&option);
///```
pub fn option_to_string<T>(option: &Option<T>) -> String
where
    T: Display,
{
    if let Some(value) = option {
        format!("Some({})", value)
    } else {
        "None".to_string()
    }
}

/// Convert an array to a [String].
///
/// # Examples
///
/// ```rust
/// use rocket_container::types::array_to_string;
///
/// let array: [String; 2] = ["Hello".to_string(), "World".to_string()];
/// let array_str: String = array_to_string(&array);
/// ```
pub fn array_to_string<T>(vec: &[T]) -> String
where
    T: Display,
{
    if vec.is_empty() {
        "[]".to_string()
    } else {
        let vec_str: String = vec
            .iter()
            .fold(String::new(), |head, tail| format!("{}, {}", head, tail));

        format!("[ {} ]", vec_str)
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
