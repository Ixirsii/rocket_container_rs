//! Container domain type definition.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::service::types::advertisement::Advertisement;
use crate::service::types::image::Image;
use crate::service::types::video::Video;

/// Container asset returned from Rocket Container.
///
/// A Container is an aggregation of advertisements, images, and videos. Rocket Container gets
/// this data from its dependencies, Rocket Advertisement, Rocket Image, and Rocket Video, and
/// aggregates them into containers for Rocket Stream.
///
/// # Examples
///
/// ```rust
/// todo!("Example");
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Container {
    ads: Vec<Advertisement>,
    id: u32,
    images: Vec<Image>,
    title: String,
    videos: Vec<Video>,
}

impl Container {
    /// Construct a new Container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// todo!("Example");
    /// ```
    pub fn new(
        ads: Vec<Advertisement>,
        id: u32,
        images: Vec<Image>,
        title: String,
        videos: Vec<Video>,
    ) -> Self {
        Container {
            ads,
            id,
            images,
            title,
            videos,
        }
    }

    pub fn from(
        container_id: u32,
        advertisements: &[Advertisement],
        images: &[Image],
        videos: &[Video],
    ) -> Self {
        let title_ads: String = match advertisements.is_empty() {
            false => "_ads".to_string(),
            true => String::new(),
        };
        let title_images: String = match images.is_empty() {
            false => "_images".to_string(),
            true => String::new(),
        };
        let title: String = format!(
            "container-{}{}${}_videos",
            container_id, title_ads, title_images
        );

        Container::new(
            advertisements.to_vec(),
            container_id,
            images.to_vec(),
            title,
            videos.to_vec(),
        )
    }
}

impl Display for Container {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Container {{ id: {}, title: {} }}", self.id, self.title)
    }
}
