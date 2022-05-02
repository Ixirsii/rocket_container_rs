//! Video repository.

use log::trace;

use crate::repository::request;
use crate::repository::types::video::{AssetReferenceDto, VideoAssetsDto, VideoDto, VideosDto};
use crate::types::{AssetType, Result, VideoType};

/// Asset reference endpoint suffix.
const ASSET_REFERENCES: &str = "asset-references";

/// Asset type query parameter.
const ASSET_TYPE: &str = "assetType";

/// Container ID query parameter.
const CONTAINER_ID: &str = "containerId";

/// Endpoint for Rocket Advertisement service.
const VIDEO_ENDPOINT: &str = "http://videos.rocket-stream.bottlerocketservices.com/videos";

/// Video type query parameter.
const VIDEO_TYPE: &str = "type";

/// Get video by ID from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::get_video;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match get_video(&client, video_id) {
///         Ok(video) => println!("Got video: {}", video),
///         Err(_) => println!("Failed to get video"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn get_video(video_id: u32) -> Result<VideoDto> {
    trace!("Getting video {}", video_id);

    request::<VideoDto, ()>(format!("{}/{}", VIDEO_ENDPOINT, video_id).as_str(), None).await
}

/// List all assets for a video from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_asset_references;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_asset_references(&client, video_id) {
///         Ok(assets) => println!("Got assets: {}", assets),
///         Err(_) => println!("Failed to get assets"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_asset_references(video_id: u32) -> Result<Vec<AssetReferenceDto>> {
    trace!("Listing asset references for video {}", video_id);

    let asset_references: Vec<AssetReferenceDto> = request::<VideoAssetsDto, ()>(
        format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
        None,
    )
    .await?
    .video_assets;

    Ok(asset_references)
}

/// List all assets for a video, by type, from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_asset_references_by_type;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_asset_references_by_type(&client, video_id) {
///         Ok(assets) => println!("Got assets: {}", assets),
///         Err(_) => println!("Failed to get assets"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_asset_references_by_type(
    video_id: u32,
    asset_type: AssetType,
) -> Result<Vec<AssetReferenceDto>> {
    trace!(
        "Listing asset references for video {} by type {}",
        video_id,
        asset_type
    );

    let asset_references: Vec<AssetReferenceDto> =
        request::<VideoAssetsDto, [(&str, AssetType); 1]>(
            format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
            Some([(ASSET_TYPE, asset_type)]),
        )
        .await?
        .video_assets;

    Ok(asset_references)
}

/// List all videos from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_videos;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_videos(&client, video_id) {
///         Ok(videos) => println!("Got videos: {}", videos),
///         Err(_) => println!("Failed to get video"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_videos() -> Result<Vec<VideoDto>> {
    trace!("Listing all videos");

    let videos: Vec<VideoDto> = request::<VideosDto, ()>(VIDEO_ENDPOINT, None).await?.videos;

    Ok(videos)
}

/// List all videos for a container from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_videos_by_container;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_videos_by_container(&client, video_id) {
///         Ok(videos) => println!("Got videos: {}", videos),
///         Err(_) => println!("Failed to get video"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_videos_by_container(container_id: u32) -> Result<Vec<VideoDto>> {
    trace!("Listing videos by container {}", container_id);

    let videos: Vec<VideoDto> = request::<VideosDto, [(&str, u32); 1]>(
        VIDEO_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await?
    .videos;

    Ok(videos)
}

/// List all videos by type from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_videos_by_type;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_videos_by_type(&client, video_id) {
///         Ok(videos) => println!("Got videos: {}", videos),
///         Err(_) => println!("Failed to get video"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_videos_by_type(video_type: VideoType) -> Result<Vec<VideoDto>> {
    trace!("Listing videos by type {}", video_type);

    let videos: Vec<VideoDto> = request::<VideosDto, [(&str, VideoType); 1]>(
        VIDEO_ENDPOINT,
        Some([(VIDEO_TYPE, video_type)]),
    )
    .await?
    .videos;

    Ok(videos)
}

/// List all videos for a container, by type, from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_container::repository::video::list_videos_by_container_and_type;
/// use reqwest::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), ()> {
///     let client = Client::new();
///     let video_id: u32 = 1;
///
///     match list_videos_by_container_and_type(&client, video_id) {
///         Ok(videos) => println!("Got videos: {}", videos),
///         Err(_) => println!("Failed to get advertisements"),
///     };
///
///     Ok(())
/// }
/// ```
pub async fn list_videos_by_container_and_type(
    container_id: u32,
    video_type: VideoType,
) -> Result<Vec<VideoDto>> {
    trace!(
        "Listing videos by container {}, type {}",
        container_id,
        video_type
    );

    let videos: Vec<VideoDto> = request::<VideosDto, [(&str, String); 2]>(
        VIDEO_ENDPOINT,
        Some([
            (CONTAINER_ID, container_id.to_string()),
            (VIDEO_TYPE, video_type.to_string()),
        ]),
    )
    .await?
    .videos;

    Ok(videos)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use crate::repository::types::video::{AssetReferenceDto, VideoDto};
    use crate::types::{AssetType, Result, VideoType};

    use super::get_video;
    use super::{
        list_asset_references, list_asset_references_by_type, list_videos,
        list_videos_by_container, list_videos_by_container_and_type, list_videos_by_type,
    };

    #[tokio::test]
    async fn test_get_video() {
        // Given
        let video_id: u32 = 1301;
        let expected: VideoDto = VideoDto::new(
            "25".to_string(),
            "Etiam vel augue. Vestibulum rutrum rutrum neque. Aenean auctor gravida sem."
                .to_string(),
            "".to_string(),
            "1301".to_string(),
            "/path/to/test1301.m3u8".to_string(),
            "My Family".to_string(),
            VideoType::Clip,
        );

        // When
        let result: Result<VideoDto> = get_video(video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references() {
        // Given
        let video_id: u32 = 1404;
        let expected: Vec<AssetReferenceDto> = vec![AssetReferenceDto::new(
            "120".to_string(),
            AssetType::Image,
            "1404".to_string(),
        )];

        // When
        let result: Result<Vec<AssetReferenceDto>> = list_asset_references(video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references_by_type() {
        // Given
        let asset_type: AssetType = AssetType::Image;
        let video_id: u32 = 1404;
        let expected: Vec<AssetReferenceDto> = vec![AssetReferenceDto::new(
            "120".to_string(),
            asset_type.clone(),
            "1404".to_string(),
        )];

        // When
        let result: Result<Vec<AssetReferenceDto>> =
            list_asset_references_by_type(video_id, asset_type).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // When
        let result: Result<Vec<VideoDto>> = list_videos().await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<VideoDto>> = list_videos_by_container(container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_type() {
        // Given
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<Vec<VideoDto>> = list_videos_by_type(video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container_and_type() {
        // Given
        let container_id: u32 = 0;
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<Vec<VideoDto>> =
            list_videos_by_container_and_type(container_id, video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }
}
