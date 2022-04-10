//! Video repository.

use super::types::video::{AssetReferenceDto, VideoAssetsDto, VideoDto, VideosDto};
use super::{get_value, get_wrapped_list};
use crate::types::{AssetType, Result, VideoType};
use log::trace;
use reqwest::Client;

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

pub async fn get_video(client: &Client, video_id: u32) -> Result<VideoDto> {
    trace!("Getting video {}", video_id);

    get_value(client, format!("{}/{}", VIDEO_ENDPOINT, video_id).as_str()).await
}

pub async fn list_asset_references(
    client: &Client,
    video_id: u32,
) -> Result<Vec<AssetReferenceDto>> {
    trace!("Listing asset references for video {}", video_id);

    get_wrapped_list::<AssetReferenceDto, VideoAssetsDto, ()>(
        client,
        format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
        None,
    )
    .await
}

pub async fn list_asset_references_by_type(
    client: &Client,
    video_id: u32,
    asset_type: AssetType,
) -> Result<Vec<AssetReferenceDto>> {
    trace!(
        "Listing asset references for video {} by type {}",
        video_id,
        asset_type
    );

    get_wrapped_list::<AssetReferenceDto, VideoAssetsDto, [(&str, AssetType); 1]>(
        client,
        format!("{}/{}/{}", VIDEO_ENDPOINT, video_id, ASSET_REFERENCES).as_str(),
        Some([(ASSET_TYPE, asset_type)]),
    )
    .await
}

pub async fn list_all_videos(client: &Client) -> Result<Vec<VideoDto>> {
    trace!("Listing all videos");

    get_wrapped_list::<VideoDto, VideosDto, ()>(client, VIDEO_ENDPOINT, None).await
}

pub async fn list_videos_by_container(client: &Client, container_id: u32) -> Result<Vec<VideoDto>> {
    trace!("Listing videos by container {}", container_id);

    get_wrapped_list::<VideoDto, VideosDto, [(&str, u32); 1]>(
        client,
        VIDEO_ENDPOINT,
        Some([(CONTAINER_ID, container_id)]),
    )
    .await
}

pub async fn list_videos_by_type(client: &Client, video_type: VideoType) -> Result<Vec<VideoDto>> {
    trace!("Listing videos by type {:#?}", video_type);

    get_wrapped_list::<VideoDto, VideosDto, [(&str, VideoType); 1]>(
        client,
        VIDEO_ENDPOINT,
        Some([(VIDEO_TYPE, video_type)]),
    )
    .await
}

pub async fn list_videos(
    client: &Client,
    container_id: u32,
    video_type: VideoType,
) -> Result<Vec<VideoDto>> {
    trace!(
        "Listing videos by container {}, type {:#?}",
        container_id,
        video_type
    );

    get_wrapped_list::<VideoDto, VideosDto, [(&str, String); 2]>(
        client,
        VIDEO_ENDPOINT,
        Some([
            (CONTAINER_ID, container_id.to_string()),
            (VIDEO_TYPE, video_type.to_string()),
        ]),
    )
    .await
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use super::get_video;
    use super::{
        list_all_videos, list_asset_references, list_asset_references_by_type, list_videos,
        list_videos_by_container, list_videos_by_type,
    };
    use crate::repository::types::video::{AssetReferenceDto, VideoDto};
    use crate::types::{AssetType, Result, VideoType};
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_video() {
        // Given
        let client: Client = Client::new();
        let video_id: u32 = 1301;
        let expected: VideoDto = VideoDto::new(
            "25".to_string(),
            "Etiam vel augue. Vestibulum rutrum rutrum neque. Aenean auctor gravida sem."
                .to_string(),
            "".to_string(),
            "1301".to_string(),
            "/path/to/test1301.m3u8".to_string(),
            "My Family".to_string(),
            VideoType::CLIP,
        );

        // When
        let result: Result<VideoDto> = get_video(&client, video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references() {
        // Given
        let client: Client = Client::new();
        let video_id: u32 = 1404;
        let expected: Vec<AssetReferenceDto> = vec![AssetReferenceDto::new(
            "120".to_string(),
            AssetType::IMAGE,
            "1404".to_string(),
        )];

        // When
        let result: Result<Vec<AssetReferenceDto>> = list_asset_references(&client, video_id).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_asset_references_by_type() {
        // Given
        let asset_type: AssetType = AssetType::IMAGE;
        let client: Client = Client::new();
        let video_id: u32 = 1404;
        let expected: Vec<AssetReferenceDto> = vec![AssetReferenceDto::new(
            "120".to_string(),
            AssetType::IMAGE,
            "1404".to_string(),
        )];

        // When
        let result: Result<Vec<AssetReferenceDto>> =
            list_asset_references_by_type(&client, video_id, asset_type).await;

        // Then
        match result {
            Ok(actual) => assert_eq!(expected, actual),
            Err(err) => panic!("Failed to list all advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_all_videos() {
        // Given
        let client: Client = Client::new();

        // When
        let result: Result<Vec<VideoDto>> = list_all_videos(&client).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_container() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;

        // When
        let result: Result<Vec<VideoDto>> = list_videos_by_container(&client, container_id).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos_by_type() {
        // Given
        let client: Client = Client::new();
        let video_type: VideoType = VideoType::MOVIE;

        // When
        let result: Result<Vec<VideoDto>> = list_videos_by_type(&client, video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }

    #[tokio::test]
    async fn test_list_videos() {
        // Given
        let client: Client = Client::new();
        let container_id: u32 = 0;
        let video_type: VideoType = VideoType::MOVIE;

        // When
        let result: Result<Vec<VideoDto>> = list_videos(&client, container_id, video_type).await;

        // Then
        match result {
            Ok(actual) => assert!(!actual.is_empty()),
            Err(err) => panic!("Failed to list advertisements with error: {:#?}", err),
        }
    }
}
