//! Advertisement service.

extern crate futures;

use futures::future;
use log::trace;
use reqwest::Client;

use crate::repository::types::video::VideoDto;
use crate::repository::video;
use crate::service::group;
use crate::service::types::video::{AssetReference, Video, VideoBuilder, VideoMap};
use crate::types::{AssetType, Result, VideoType};

/// Get video by ID from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::get_video;
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
pub async fn get_video(client: &Client, video_id: u32) -> Result<Video> {
    trace!("Getting video {}", video_id);

    let assets: Vec<AssetReference> = list_asset_references(client, video_id).await?;
    let video: Video = VideoBuilder::from(video::get_video(client, video_id).await?)
        .assets(assets)
        .build();

    Ok(video)
}

/// List all assets for a video from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_asset_references;
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
pub async fn list_asset_references(client: &Client, video_id: u32) -> Result<Vec<AssetReference>> {
    trace!("Listing asset references for video {}", video_id);

    let asset_references: Vec<AssetReference> = video::list_asset_references(client, video_id)
        .await?
        .into_iter()
        .map(AssetReference::from)
        .collect();

    Ok(asset_references)
}

/// List all assets for a video, by type, from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_asset_references_by_type;
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
    client: &Client,
    video_id: u32,
    asset_type: AssetType,
) -> Result<Vec<AssetReference>> {
    trace!(
        "Listing asset references for video {} by type {}",
        video_id,
        asset_type
    );

    let asset_references: Vec<AssetReference> =
        video::list_asset_references_by_type(client, video_id, asset_type)
            .await?
            .into_iter()
            .map(AssetReference::from)
            .collect();

    Ok(asset_references)
}

/// List all videos from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_videos;
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
pub async fn list_videos(client: &Client) -> Result<VideoMap> {
    trace!("Listing all videos");

    let images: Vec<(u32, Video)> = future::try_join_all(
        video::list_videos(client)
            .await?
            .into_iter()
            .map(|video_dto| map_video(client, video_dto)),
    )
    .await
    .unwrap();

    Ok(group(images.into_iter()))
}

/// List all videos for a container from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_videos_by_container;
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
pub async fn list_videos_by_container(client: &Client, container_id: u32) -> Result<VideoMap> {
    trace!("Listing videos by container id {}", container_id);

    let images: Vec<(u32, Video)> = future::try_join_all(
        video::list_videos_by_container(client, container_id)
            .await?
            .into_iter()
            .map(|video_dto| map_video(client, video_dto)),
    )
    .await
    .unwrap();

    Ok(group(images.into_iter()))
}

/// List all videos by type from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_videos_by_type;
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
pub async fn list_videos_by_type(client: &Client, video_type: VideoType) -> Result<VideoMap> {
    trace!("Listing videos by type {}", video_type);

    let images: Vec<(u32, Video)> = future::try_join_all(
        video::list_videos_by_type(client, video_type)
            .await?
            .into_iter()
            .map(|video_dto| map_video(client, video_dto)),
    )
    .await
    .unwrap();

    Ok(group(images.into_iter()))
}

/// List all videos for a container, by type, from Rocket Video.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::service::video::list_videos_by_container_and_type;
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
    client: &Client,
    container_id: u32,
    video_type: VideoType,
) -> Result<VideoMap> {
    trace!(
        "Listing videos by container {} and type {}",
        container_id,
        video_type
    );

    let images: Vec<(u32, Video)> = future::try_join_all(
        video::list_videos_by_container_and_type(client, container_id, video_type)
            .await?
            .into_iter()
            .map(|video_dto| map_video(client, video_dto)),
    )
    .await
    .unwrap();

    Ok(group(images.into_iter()))
}

/* ********************************** Private utility function ********************************** */

async fn map_video(client: &Client, video_dto: VideoDto) -> Result<(u32, Video)> {
    let assets: Vec<AssetReference> =
        list_asset_references(client, video_dto.id().parse().unwrap()).await?;

    Ok((
        video_dto.container_id().parse().unwrap(),
        VideoBuilder::from(video_dto).assets(assets).build(),
    ))
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::service::types::video::{AssetReference, Video, VideoMap};
    use crate::types::{AssetType, Result, VideoType};

    use super::{
        get_video, list_asset_references, list_asset_references_by_type, list_videos,
        list_videos_by_container, list_videos_by_container_and_type, list_videos_by_type,
    };

    #[tokio::test]
    async fn test_get_video() {
        // Given
        let client: Client = Client::new();
        let video_id: u32 = 1301;
        let expected: Video = Video::new(
            Vec::new(),
            "Etiam vel augue. Vestibulum rutrum rutrum neque. Aenean auctor gravida sem."
                .to_string(),
            "".to_string(),
            1301,
            "/path/to/test1301.m3u8".to_string(),
            "My Family".to_string(),
            VideoType::Clip,
        );

        // When
        let result: Result<Video> = get_video(&client, video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to get video with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references() {
        // Given
        let client: Client = Client::new();
        let video_id: u32 = 1404;
        let expected: Vec<AssetReference> = vec![AssetReference::new(120, AssetType::Image)];

        // When
        let result: Result<Vec<AssetReference>> = list_asset_references(&client, video_id).await;

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
        let client: Client = Client::new();
        let video_id: u32 = 1404;
        let expected: Vec<AssetReference> = vec![AssetReference::new(120, AssetType::Image)];

        // When
        let result: Result<Vec<AssetReference>> =
            list_asset_references_by_type(&client, video_id, asset_type).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list asset references with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<VideoMap> = list_videos(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<VideoMap> = list_videos_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_type() {
        // Given
        let client: Client = Client::new();
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<VideoMap> = list_videos_by_type(&client, video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container_and_type() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;
        let video_type: VideoType = VideoType::Movie;

        // When
        let result: Result<VideoMap> =
            list_videos_by_container_and_type(&client, container_id, video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list videos with error: {}", err),
        }
    }
}
