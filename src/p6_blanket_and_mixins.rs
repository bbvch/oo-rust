//! # 6. Blanket impls and generic mixins
//!
//! Two mechanisms C++ has no direct equivalent for, both of which do work that
//! inheritance is usually drafted into:
//!
//! * **Blanket impl** — `impl<T: HasShapeData> Positioned for T` gives an
//!   entire *category* of types a body of behaviour with zero per-type code.
//!   That is implementation inheritance, granted by capability rather than by
//!   lineage: any type that gains the bound later gets the methods retroactively.
//!
//! * **Generic mixin wrapper** — `Bordered<S>` adds state *and* behaviour to any
//!   `S: Shape`, and is itself a `Shape`. Stacking wrappers composes vertically
//!   the way a class hierarchy does, but each layer is independently testable
//!   and the stacking order is chosen at the use site, not at declaration.
//!
//! Both are statically dispatched and monomorphized — the CRTP niche, minus the
//! CRTP.

use crate::common::{banner, Color, Point, ShapeData};

pub trait HasShapeData {
    fn data(&self) -> &ShapeData;
    fn data_mut(&mut self) -> &mut ShapeData;
}

pub trait Shape: HasShapeData {
    fn area(&self) -> f64;
}

/// Nobody implements this trait. Everyone gets it.
pub trait Positioned {
    fn position(&self) -> Point;
    fn translate(&mut self, dx: f64, dy: f64);
    fn distance_to_origin(&self) -> f64;
}

impl<T: HasShapeData> Positioned for T {
    fn position(&self) -> Point {
        self.data().origin
    }
    fn translate(&mut self, dx: f64, dy: f64) {
        self.data_mut().translate(dx, dy);
    }
    fn distance_to_origin(&self) -> f64 {
        let p = self.position();
        (p.x * p.x + p.y * p.y).sqrt()
    }
}

pub struct Circle {
    base: ShapeData,
    pub radius: f64,
}

impl Circle {
    pub fn new(origin: Point, color: Color, radius: f64) -> Self {
        Self {
            base: ShapeData::new("Circle", origin, color),
            radius,
        }
    }
}

impl HasShapeData for Circle {
    fn data(&self) -> &ShapeData {
        &self.base
    }
    fn data_mut(&mut self) -> &mut ShapeData {
        &mut self.base
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// Mixin: extends any shape with a border, and remains a shape itself.
pub struct Bordered<S: Shape> {
    inner: S,
    pub width: f64,
}

impl<S: Shape> Bordered<S> {
    pub fn new(inner: S, width: f64) -> Self {
        Self { inner, width }
    }

    /// Reaching the wrapped value is explicit, unlike a `Deref` chain.
    pub fn inner(&self) -> &S {
        &self.inner
    }
}

impl<S: Shape> HasShapeData for Bordered<S> {
    fn data(&self) -> &ShapeData {
        self.inner.data()
    }
    fn data_mut(&mut self) -> &mut ShapeData {
        self.inner.data_mut()
    }
}

impl<S: Shape> Shape for Bordered<S> {
    /// Overriding by wrapping: call "super", then adjust.
    fn area(&self) -> f64 {
        self.inner.area() + self.width
    }
}

/// A second mixin, to show the layers stack in whatever order you choose.
pub struct Scaled<S: Shape> {
    inner: S,
    pub factor: f64,
}

impl<S: Shape> Scaled<S> {
    pub fn new(inner: S, factor: f64) -> Self {
        Self { inner, factor }
    }
}

impl<S: Shape> HasShapeData for Scaled<S> {
    fn data(&self) -> &ShapeData {
        self.inner.data()
    }
    fn data_mut(&mut self) -> &mut ShapeData {
        self.inner.data_mut()
    }
}

impl<S: Shape> Shape for Scaled<S> {
    fn area(&self) -> f64 {
        self.inner.area() * self.factor
    }
}

pub fn demo() {
    banner("6. Blanket impls + generic mixins");
    let mut c = Circle::new(Point::new(3.0, 4.0), Color::Red, 1.0);
    // `translate` / `distance_to_origin` were never implemented for Circle.
    c.translate(0.0, 0.0);
    println!(
        "{} distance from origin {:.2}",
        c.data().label(),
        c.distance_to_origin()
    );

    let stacked = Scaled::new(Bordered::new(c, 10.0), 2.0);
    println!(
        "Scaled<Bordered<Circle>> area = (pi + 10) * 2 = {:.2}",
        stacked.area()
    );
    // The blanket impl reaches through both wrapper layers.
    println!("...and it is still Positioned: {:?}", stacked.position());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blanket_impl_grants_behaviour_without_per_type_code() {
        let c = Circle::new(Point::new(3.0, 4.0), Color::Red, 1.0);
        assert_eq!(c.distance_to_origin(), 5.0);
    }

    #[test]
    fn mixins_stack_and_order_matters() {
        let c = || Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0);
        let pi = std::f64::consts::PI;
        let a = Scaled::new(Bordered::new(c(), 10.0), 2.0).area();
        let b = Bordered::new(Scaled::new(c(), 2.0), 10.0).area();
        assert!((a - (pi + 10.0) * 2.0).abs() < 1e-9);
        assert!((b - (pi * 2.0 + 10.0)).abs() < 1e-9);
        assert!((a - b).abs() > 1e-9);
    }

    #[test]
    fn wrappers_forward_base_state() {
        let mut w = Bordered::new(Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0), 1.0);
        w.translate(2.0, 2.0);
        assert_eq!(w.inner().data().origin, Point::new(2.0, 2.0));
    }
}
