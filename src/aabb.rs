use core::borrow::Borrow;

use crate::{abs, Collides, Penetration};

/// Axis-Aligned Bounding Box
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    x: Range,
    y: Range,
}

impl Aabb {
    /// Create from `min` and `max` points
    ///
    /// In a typical screen-space coordinate system (the x-axis points to the right, and the y-axis points down)
    /// `min` would be the "top-left" and `max` would be the "bottom-right"
    #[must_use]
    pub fn from_min_max(min: impl Into<[f32; 2]>, max: impl Into<[f32; 2]>) -> Self {
        let min = min.into();
        let max = max.into();
        let x = Range::from_min_max(min[0], max[0]);
        let y = Range::from_min_max(min[1], max[1]);
        Self { x, y }
    }

    /// Create from `top_left` and `size` of the rectangle
    #[must_use]
    pub fn from_top_left_and_size(
        top_left: impl Into<[f32; 2]>,
        size: impl Into<[f32; 2]>,
    ) -> Self {
        let [x, y] = top_left.into();
        let [w, h] = size.into();
        let x = Range::from_min_max(x, x + w);
        let y = Range::from_min_max(y, y + h);
        Self { x, y }
    }

    /// Move the shape by the given offset
    pub fn offset(&mut self, offset: impl Into<[f32; 2]>) {
        let [dx, dy] = offset.into();
        self.x.offset(dx);
        self.y.offset(dy);
    }

    /// Returned shape after appliying the offset
    #[must_use]
    pub fn with_offset(mut self, offset: impl Into<[f32; 2]>) -> Self {
        self.offset(offset);
        self
    }

    /// Returns the minimum point of the shape
    ///
    /// In a typical screen-space coordinate system (the x-axis points to the right, and the y-axis points down)
    /// this would be the "top-left"
    #[must_use]
    pub fn min(&self) -> [f32; 2] {
        [self.x.min, self.y.min]
    }

    /// Returns the maximum point of the shape
    ///
    /// In a typical screen-space coordinate system (the x-axis points to the right, and the y-axis points down)
    /// this would be the "bottom-right"
    #[must_use]
    pub fn max(&self) -> [f32; 2] {
        [self.x.max, self.y.max]
    }

    /// Returs the size of the shape
    #[must_use]
    pub fn size(&self) -> [f32; 2] {
        [self.x.max - self.x.min, self.y.max - self.y.min]
    }
}

impl Collides for Aabb {
    fn collides(&self, other: impl Borrow<Aabb>) -> bool {
        let other = other.borrow();
        self.x.collides(other.x) && self.y.collides(other.y)
    }
}

impl Penetration for Aabb {
    fn penetration(&self, other: impl Borrow<Self>) -> Option<[f32; 2]> {
        let other = other.borrow();
        let x = self.x.penetration(other.x)?;
        let y = self.y.penetration(other.y)?;
        Some(if abs(x) < abs(y) { [x, 0.] } else { [0., y] })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    min: f32,
    max: f32,
}

impl Range {
    fn from_min_max(min: f32, max: f32) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    fn penetration(self, other: Self) -> Option<f32> {
        let p1 = Some(other.min - self.max).filter(|p| *p < 0.)?;
        let p2 = Some(other.max - self.min).filter(|p| *p > 0.)?;
        Some(if abs(p1) < abs(p2) { p1 } else { p2 })
    }

    fn offset(&mut self, offset: f32) {
        self.min += offset;
        self.max += offset;
    }
}

impl Collides for Range {
    fn collides(&self, other: impl Borrow<Self>) -> bool {
        let other = other.borrow();
        self.max > other.min && self.min < other.max
    }
}

impl From<Range> for [f32; 2] {
    fn from(Range { min, max }: Range) -> Self {
        [min, max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_should_penetrate_self() {
        let range = Range::from_min_max(0., 1.);
        assert_eq!(range.penetration(range).map(abs), Some(1.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn can_get_min_max_from_aabb() {
        let aabb = Aabb::from_min_max([1., 2.], [3., 4.]);
        assert_eq!(aabb.min(), [1., 2.]);
        assert_eq!(aabb.max(), [3., 4.]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn can_build_from_top_left_and_size() {
        let from_min_max = Aabb::from_min_max([1., 2.], [3., 5.]);
        let from_rect = Aabb::from_top_left_and_size([1., 2.], [2., 3.]);
        assert_eq!(from_min_max, from_rect);
        assert_eq!(from_rect.size(), [2., 3.]);
        assert_eq!(from_min_max.size(), [2., 3.]);
    }

    #[test]
    fn can_build_from_top_left_and_negative_size() {
        let from_min_max = Aabb::from_min_max([1., 2.], [3., 5.]);
        let from_rect = Aabb::from_top_left_and_size([3., 5.], [-2., -3.]);
        assert_eq!(from_min_max, from_rect);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn can_offset_shape() {
        let aabb = Aabb::from_min_max([1., 2.], [3., 4.]).with_offset([5.0, 6.0]);
        assert_eq!(aabb.min(), [6., 8.]);
        assert_eq!(aabb.max(), [8., 10.]);
    }
}
