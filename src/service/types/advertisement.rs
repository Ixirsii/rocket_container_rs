//! Advertisement domain type definition.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Advertisement asset returned from Rocket Container.
///
/// Container service returns a variant of [`AdvertisementDto`][1] with `id` field as a number and
/// without `container_id` field. [`AdvertisementDto`][1]s returned from
/// [`advertisement repository`][2] get converted into this type before being returned from the
/// controller.
///
/// # Examples
///
/// ```rust
/// use rocket_stream::controller::types::advertisement::Advertisement;
///
/// let advertisement = Advertisement::new(
///     1,
///     "Advertisement".to_string(),
///     "https://advertisement.com/video".to_string(),
/// );
/// ```
///
/// ```rust
/// use rocket_stream::controller::types::advertisement::Advertisement;
///
/// let advertisement_dto: AdvertisementDto = ...;
/// let advertisement = Advertisement::from(advertisement_dto);
/// ```
///
/// [1]: [crate::repository::types::advertisement::AdvertisementDto]
/// [2]: [crate::repository::advertisement]
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisement {
    /// Unique advertisement identifier.
    id: u32,
    /// Name of advertisement.
    name: String,
    /// Advertisement playback url.
    url: String,
}

impl Advertisement {
    /// Construct a new Advertisement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket_stream::controller::types::advertisement::Advertisement;
    ///
    /// let advertisement = Advertisement::new(
    ///     1,
    ///     "Advertisement".to_string(),
    ///     "https://advertisement.com/video".to_string(),
    /// );
    /// ```
    pub fn new(id: u32, name: String, url: String) -> Self {
        Advertisement { id, name, url }
    }
}

impl Display for Advertisement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Advertisement {{ id: {}, name: {}, url: {} }}",
            self.id, self.name, self.url
        )
    }
}

pub type AdvertisementMap = HashMap<u32, Vec<Advertisement>>;
