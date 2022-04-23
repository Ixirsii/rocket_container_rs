//! A solution for Bottle Rocket Studio's Rocket Stream coding challenge.

#[macro_use]
extern crate rocket;

use crate::routes::{get_advertisements, get_container, get_images, get_videos, list_containers};

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
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
    fn list_containers() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let mut response = client.get("/").dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
    }
}
