#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2d collision detection library supporting simple primitive shapes only,
//! and that is suitable for development on `no_std` targets.
//!
//! # Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! `aabb`: *(enabled by default)* Axis-Aligned Bounding Box (`Aabb`) shape
//! `libm`: Use `libm` for math. This is notably necessary for some features to work on `no_std`

#[cfg(feature = "aabb")]
mod aabb;

#[cfg(feature = "aabb")]
pub use aabb::Aabb;

/// Trait implemented by shapes that can be checked for collisions with shapes of type `S`
pub trait Collides<S = Self> {
    /// Returns true if `self` overlaps `other`
    #[must_use]
    fn collides(&self, other: &S) -> bool;

    /// Returns true if `self` collides with any `others`
    ///
    /// If the `others` iterator is empty, this function returns `false`
    #[must_use]
    fn collides_any<'a>(&self, others: impl IntoIterator<Item = &'a S>) -> bool
    where
        S: 'a,
    {
        others.into_iter().any(|other| self.collides(other))
    }
}
