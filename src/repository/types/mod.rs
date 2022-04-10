//! Repository module type definitions.

pub mod advertisement;
pub mod image;
pub mod video;

/// Trait for objects which wrap other objects.
///
/// Rocket Stream's dependencies return lists wrapped in an object.
/// For example, Rocket Advertisement's `listAdvertisements` API returns
///
/// ```json
/// {
///   "advertisements": [
///     {
///       "containerId": number,
///       "id": number,
///       "name": string,
///       "url": string
///     }
///   ]
/// }
/// ```
///
/// The `advertisements` field is modeled by [Advertisements] which implements this trait so that
/// it can be unwrapped to [Vec]<[Advertisement]>.
///
/// # Examples
///
/// ```rust
/// struct Advertisements {
///     advertisements: Vec<Advertisement>,
/// }
///
/// impl Wrapper<Advertisement> for Advertisements {
///     fn unwrap(self) -> Vec<Advertisement> {
///         self.advertisements
///     }
/// }
/// ```
pub trait Wrapper<T> {
    fn unwrap(self) -> Vec<T>;
}
