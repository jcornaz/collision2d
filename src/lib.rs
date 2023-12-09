#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple 2D collision detection library supporting simple primitive shapes only,
//! and that is suitable for development on `no_std` targets.
//!
//! # Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! * `aabb`: *(enabled by default)* Axis-Aligned Bounding Box (`Aabb`) shape
//! * `libm`: Use `libm` for math. This is notably necessary for some features to work on `no_std`

#[cfg(feature = "aabb")]
mod aabb;

use core::borrow::Borrow;

#[cfg(feature = "aabb")]
pub use aabb::Aabb;

/// Trait implemented by shapes that can be checked for collisions with shapes of type `S`
pub trait Collides<S = Self> {
    /// Returns true if `self` overlaps `other`
    #[must_use]
    fn collides(&self, other: impl Borrow<S>) -> bool;

    /// Returns true if `self` collides with any `others`
    ///
    /// If the `others` iterator is empty, this function returns `false`
    #[must_use]
    fn collides_any<'a, T>(&self, others: impl IntoIterator<Item = T>) -> bool
    where
        T: Borrow<S> + 'a,
    {
        others.into_iter().any(|other| self.collides(other))
    }
}

/// Trait implemented by shapes that can check for penetration into other shapes of type `S`
pub trait Penetration<S = Self> {
    /// Returns by how much `self` should be moved in order to resolve penetration with `other`
    ///
    /// Returns `None` if the two shapes are not collided
    #[must_use]
    fn penetration(&self, other: &S) -> Option<[f32; 2]>;

    /// Returns the maximum penetration of `self` against the `others` shapes.
    ///
    /// Returns `None` if `self` does not penetrate any of the `others` shape.
    #[must_use]
    fn max_penetration<'a>(&self, others: impl IntoIterator<Item = &'a S>) -> Option<[f32; 2]>
    where
        S: 'a,
    {
        let mut max_magnitude = f32::MIN;
        let mut max = None;
        others
            .into_iter()
            .filter_map(move |other| self.penetration(other))
            .for_each(|p| {
                let magnitude = abs(p[0]) + abs(p[1]);
                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                    max = Some(p);
                }
            });
        max
    }
}

#[cfg(feature = "std")]
fn abs(v: f32) -> f32 {
    v.abs()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn abs(v: f32) -> f32 {
    libm::fabsf(v)
}

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
fn abs(v: f32) -> f32 {
    f32::from_bits(v.to_bits() & 0x7fff_ffff)
}
