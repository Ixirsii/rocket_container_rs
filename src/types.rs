//! Public crate type definitions.
//!
//! This will definitely change in the future as I learn more about how rust works and develop a
//! better plan of implementation.

use serde::{Deserialize, Serialize};

/* ******************************************* Types ******************************************** */

/// Type of asset an `AssetReference` links too.
///
/// `AssetReference` is a generic container which points to "assets" associated with a video.
/// Those assets can be either advertisements or images, and the type of asset is tracked by the
/// types of this enum.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AssetType {
    /// Advertisement asset.
    AD,
    /// Image asset.
    IMAGE,
}

/// Type of `Video`
///
/// Videos can be either short clips, TV length episodes, or full length movies, and the type of
/// video is tracked by the types of this enum.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VideoType {
    /// A short clip.
    CLIP,
    /// A TV length episode.
    EPISODE,
    /// A full length movie.
    MOVIE,
}

// #[derive(Debug)]
// pub struct Container {
//     ads: Vec<Advertisement>,
//     id: u32,
//     images: Vec<Image>,
//     title: String,
//     videos: Vec<Video>
// }

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
            Ok(asset_type) => assert_eq!(asset_type, AssetType::AD),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Ok(asset_type) => assert_eq!(asset_type, AssetType::IMAGE),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Ok(video_type) => assert_eq!(video_type, VideoType::CLIP),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Ok(video_type) => assert_eq!(video_type, VideoType::EPISODE),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
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
            Ok(video_type) => assert_eq!(video_type, VideoType::MOVIE),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn serialize_asset_type_ad() {
        // Given
        let expected: String = String::from("\"AD\"");

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&AssetType::AD);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(_) => panic!("Result should be Ok"),
        }
    }

    #[test]
    fn serialize_asset_type_image() {
        // Given
        let expected: String = String::from("\"IMAGE\"");

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&AssetType::IMAGE);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn serialize_video_type_clip() {
        // Given
        let expected: String = String::from("\"CLIP\"");

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::CLIP);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn serialize_video_type_episode() {
        // Given
        let expected: String = String::from("\"EPISODE\"");

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::EPISODE);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }

    #[test]
    fn serialize_video_type_movie() {
        // Given
        let expected: String = String::from("\"MOVIE\"");

        // When
        let actual: serde_json::Result<String> = serde_json::to_string(&VideoType::MOVIE);

        // Then
        match actual {
            Ok(json) => assert_eq!(json, expected),
            Err(err) => panic!("Failed to deserialize with error: {:?}", err),
        }
    }
}
