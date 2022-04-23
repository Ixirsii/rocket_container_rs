//! Rocket Container endpoints..

use log::error;
use rocket::{get, serde::json::Json};

use crate::controller::types::{Error, ErrorResponse, Result};
use crate::service::advertisement;
use crate::service::types::advertisement::Advertisement;
use crate::service::types::container::Container;
use crate::service::types::image::Image;
use crate::service::types::video::Video;

mod types;

#[get("/containers/<container_id>/ads")]
pub async fn get_advertisements(container_id: u32) -> Result<Vec<Advertisement>> {
    match advertisement::list_advertisements_by_container(container_id).await {
        Ok(advertisements) => Ok(Json(advertisements)),
        Err(error) => {
            error!(
                "Error while listing advertisements by container {} {}",
                container_id, error
            );

            Err(Error::InternalServiceError(Json(ErrorResponse {
                message: "No advertisements found for this container".to_string(),
            })))
        }
    }
}

#[get("/containers/<container_id>")]
pub async fn get_container(container_id: u32) -> Result<Container> {
    todo!("get_container")
}

#[get("/containers/<container_id>/images")]
pub async fn get_images(container_id: u32) -> Result<Vec<Image>> {
    todo!("get_images")
}

#[get("/containers/<container_id>/videos")]
pub async fn get_videos(container_id: u32) -> Result<Vec<Video>> {
    todo!("get_videos")
}

#[get("/containers")]
pub async fn list_containers() -> Result<Vec<Container>> {
    todo!("list_containers")
}
