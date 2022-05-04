//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#[macro_use]
extern crate rocket;

use rocket_container::controller::{
    get_advertisements, get_container, get_images, get_videos, list_containers,
};
use rocket_container::repository::advertisement::AdvertisementRepository;
use rocket_container::repository::client::Client;
use rocket_container::repository::image::ImageRepository;
use rocket_container::repository::video::VideoRepository;
use rocket_container::service::advertisement::AdvertisementService;
use rocket_container::service::image::ImageService;
use rocket_container::service::video::VideoService;
use std::sync::Arc;

#[launch]
pub fn rocket() -> _ {
    let client: Arc<Client> = Arc::new(Client::default());
    let advertisement_service: AdvertisementService =
        AdvertisementService::new(AdvertisementRepository::new(Arc::clone(&client)));
    let image_service: ImageService = ImageService::new(ImageRepository::new(Arc::clone(&client)));
    let video_service: VideoService = VideoService::new(VideoRepository::new(client));

    rocket::build()
        .manage(advertisement_service)
        .manage(image_service)
        .manage(video_service)
        .mount(
            "/",
            routes![
                get_advertisements,
                get_container,
                get_images,
                get_videos,
                list_containers
            ],
        )
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::rocket;

    #[test]
    fn get_advertisements() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers/0/ads").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }
}
