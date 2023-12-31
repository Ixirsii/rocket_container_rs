//! A solution for Bottle Rocket Studio's Rocket Container coding challenge.

#[macro_use]
extern crate rocket;

use std::sync::Arc;

use rocket_container::{
    controller::{get_advertisements, get_container, get_images, get_videos, list_containers},
    repository::{
        advertisement::AdvertisementRepository, client::Client, image::ImageRepository,
        video::VideoRepository,
    },
    service::{
        advertisement::AdvertisementService, container::ContainerService, image::ImageService,
        video::VideoService,
    },
};

/// Main function for a Rocket application.
#[launch]
pub fn rocket() -> _ {
    let container_service: ContainerService = get_container_service();

    rocket::build().manage(container_service).mount(
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

fn get_container_service() -> ContainerService {
    let client: Arc<Client> = Arc::new(Client::default());
    let advertisement_service: AdvertisementService =
        AdvertisementService::new(AdvertisementRepository::new(client.clone()));
    let image_service: ImageService = ImageService::new(ImageRepository::new(client.clone()));
    let video_service: VideoService = VideoService::new(VideoRepository::new(client));

    ContainerService::new(advertisement_service, image_service, video_service)
}

/* ******************************************* Tests ******************************************** */

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::rocket;

    #[test]
    fn list_container() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_container() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers/0").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_advertisements() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers/0/ads").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_images() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers/0/images").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_videos() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let response = client.get("/containers/0/videos").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }
}
