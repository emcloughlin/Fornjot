use crate::{
    kernel::geometry::{Curve, Surface},
    math::Point,
};

use super::handle::{Handle, Storage};

/// API to access a shape's geometry
pub struct Geometry;

impl Geometry {
    /// Add a point to the shape
    pub fn add_point(&mut self, point: Point<3>) -> Handle<Point<3>> {
        Storage::new(point).handle()
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        Storage::new(curve).handle()
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        Storage::new(surface).handle()
    }
}