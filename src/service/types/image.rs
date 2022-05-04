//! Image domain type definition.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Image asset returned from Rocket Container.
///
/// Container service returns a variant of [`ImageDto`][1] with `id` field as a number and
/// without `container_id` field. [`ImageDto`][1]s returned from
/// [`image repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// use rocket_container::service::types::image::Image;
///
/// let image = Image::new(
///     1,
///     "Cool video thumbnail".to_string(),
///     "https://video.com/thumbnail.png".to_string(),
/// );
/// ```
///
/// ```rust
/// use rocket_container::controller::types::image::Image;
///
/// let image_dto: ImageDto = ...;
/// let image = Image::from(image_dto);
/// ```
///
/// [1]: [crate::repository::types::image::ImageDto]
/// [2]: [crate::repository::image]
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Unique image identifier.
    id: u32,
    /// Name of image.
    name: String,
    /// Image URL.
    url: String,
}

impl Image {
    /// Construct a new Image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_container::controller::types::image::Image;
    ///
    /// let image = Image::new(
    ///     1,
    ///     "Cool video thumbnail".to_string(),
    ///     "https://video.com/thumbnail.png".to_string(),
    /// );
    /// ```
    pub fn new(id: u32, name: String, url: String) -> Self {
        Image { id, name, url }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image {{ id: {}, name: {}, url: {} }}",
            self.id, self.name, self.url
        )
    }
}

/// Type alias for a [`HashMap`] of [`u32`] to [`Vec`]`<`[`Image`]`>`.
pub type ImageMap = HashMap<u32, Vec<Image>>;
