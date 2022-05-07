//! Rocket Container controller layer.

use log::{error, trace};
use rocket::{get, serde::json::Json, Responder, State};
use serde::Serialize;

use crate::service::{
    advertisement::Advertisement, advertisement::AdvertisementService, container::Container,
    image::Image, video::Video,
};

/* ************************************** Error Responder *************************************** */

/// Error Responder.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Responder)]
pub enum Error {
    /// 400 - Bad Request.
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),
    /// 404 - Not Found.
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),
    /// 500 - Internal Server Error.
    #[response(status = 500, content_type = "json")]
    InternalServiceError(Json<ErrorResponse>),
}

/* ************************************** Error Response **************************************** */

/// Error Response.
///
/// # Examples
///
/// ```rust
/// ```
#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
    /// Error message.
    pub message: String,
}

/* ************************************** Request Result **************************************** */

/// Controller result.
///
/// An alias for [`std::result::Result`] where Ok is a [`Json`] of `T` and Err is an [`Error`].
///
/// # Examples
///
/// ```rust
/// use rocket_container::{
///     controller::{Error, ErrorResponse, Result},
///     service::types::advertisement::Advertisement
/// };
/// use rocket::serde::json::Json;
///
/// let ok: Result<Advertisement> = Ok(Json(advertisement));
/// let error: Result<Advertisement> = Err(Error::InternalServiceError(Json(ErrorResponse {
///     message: "No advertisements found for this container".to_string(),
/// })));
/// ```
pub type Result<T> = std::result::Result<Json<T>, Error>;

/* ***************************** GET /containers/<container_id>/ads ***************************** */

/// GET /containers/<container_id>/ads.
///
/// Controller for getting all advertisements for a container.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate rocket;
///
/// use rocket_container::controller::get_advertisements;
///
/// #[launch]
/// pub fn rocket() -> _ {
///     rocket::build().mount("/", routes![get_advertisements])
/// }
/// ```
#[get("/containers/<container_id>/ads")]
pub async fn get_advertisements(
    container_id: u32,
    service: &State<AdvertisementService>,
) -> Result<Vec<Advertisement>> {
    trace!("GET /containers/{}/ads", container_id);

    match service
        .inner()
        .list_advertisements_by_container(container_id)
        .await
    {
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

/* ****************************** GET /containers/<container_id> ******************************** */

/// GET /containers/<container_id>.
///
/// Controller for getting a container by ID.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate rocket;
///
/// use rocket_container::controller::get_container;
///
/// #[launch]
/// pub fn rocket() -> _ {
///     rocket::build().mount("/", routes![get_container])
/// }
/// ```
#[get("/containers/<container_id>")]
pub async fn get_container(container_id: u32) -> Result<Container> {
    trace!("GET /containers/{}", container_id);

    todo!("get_container")
}

/* *************************** GET /containers/<container_id>/images **************************** */

/// GET /containers/<container_id>/images.
///
/// Controller for getting all images for a container.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate rocket;
///
/// use rocket_container::controller::get_images;
///
/// #[launch]
/// pub fn rocket() -> _ {
///     rocket::build().mount("/", routes![get_images])
/// }
/// ```
#[get("/containers/<container_id>/images")]
pub async fn get_images(container_id: u32) -> Result<Vec<Image>> {
    trace!("GET /containers/{}/images", container_id);

    todo!("get_images")
}

/* *************************** GET /containers/<container_id>/videos **************************** */

/// GET /containers/<container_id>/videos.
///
/// Controller for getting all videos for a container.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate rocket;
///
/// use rocket_container::controller::get_videos;
///
/// #[launch]
/// pub fn rocket() -> _ {
///     rocket::build().mount("/", routes![get_videos])
/// }
/// ```
#[get("/containers/<container_id>/videos")]
pub async fn get_videos(container_id: u32) -> Result<Vec<Video>> {
    trace!("GET /containers/{}/videos", container_id);

    todo!("get_videos")
}

/* ************************************** GET /containers *************************************** */

/// GET /containers.
///
/// Controller for getting all containers.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate rocket;
///
/// use rocket_container::controller::list_containers;
///
/// #[launch]
/// pub fn rocket() -> _ {
///     rocket::build().mount("/", routes![list_containers])
/// }
/// ```
#[get("/containers")]
pub async fn list_containers() -> Result<Vec<Container>> {
    trace!("GET /containers");

    todo!("list_containers")
}
