//! # 1. Explicit composition + hand-written delegation
//!
//! The literal translation of `class Circle : public Shape`: hold the base as a
//! named field and forward to it by hand. This is what `Deref` is trying to
//! automate — and writing it out is the point, because the call site now says
//! `circle.data.translate(..)` or `circle.translate(..)`, never "somehow a
//! `Circle` is also a `ShapeData`".
//!
//! Cost: O(types x methods) forwarding functions. See `p7_delegation_macro` for
//! the mechanical fix and `p2_accessor_trait` for the structural one.

use crate::common::{banner, Color, Point, ShapeData};

pub struct Circle {
    base: ShapeData,
    radius: f64,
}

impl Circle {
    pub fn new(origin: Point, color: Color, radius: f64) -> Self {
        Self {
            base: ShapeData::new("Circle", origin, color),
            radius,
        }
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // --- forwarding boilerplate: the price of explicitness ---
    pub fn label(&self) -> String {
        self.base.label()
    }
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.base.translate(dx, dy);
    }
}

pub struct Rectangle {
    base: ShapeData,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(origin: Point, color: Color, width: f64, height: f64) -> Self {
        Self {
            base: ShapeData::new("Rectangle", origin, color),
            width,
            height,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn label(&self) -> String {
        self.base.label()
    }
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.base.translate(dx, dy);
    }
}

/// The case that breaks naive `Deref` chains: in C++ this is
/// `class Square : public Rectangle`, i.e. inheritance between two *concrete*
/// types, two levels deep. Composition handles it without ceremony.
pub struct Square {
    rect: Rectangle,
}

impl Square {
    pub fn new(origin: Point, color: Color, side: f64) -> Self {
        let mut rect = Rectangle::new(origin, color, side, side);
        rect.base.name = "Square".to_owned();
        Self { rect }
    }

    pub fn side(&self) -> f64 {
        self.rect.width
    }

    /// Reusing the "parent" implementation is an ordinary call, not a keyword.
    pub fn area(&self) -> f64 {
        self.rect.area()
    }
    pub fn label(&self) -> String {
        self.rect.label()
    }
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.rect.translate(dx, dy);
    }
}

pub fn demo() {
    banner("1. Composition + manual delegation");
    let mut c = Circle::new(Point::new(0.0, 0.0), Color::Red, 2.0);
    let mut s = Square::new(Point::new(1.0, 1.0), Color::Blue, 3.0);
    c.translate(1.0, 0.0);
    s.translate(0.0, 2.0);
    println!("{} area {:.2}", c.label(), c.area());
    println!("{} area {:.2}", s.label(), s.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_reuses_rectangle_area() {
        let s = Square::new(Point::new(0.0, 0.0), Color::Red, 4.0);
        assert_eq!(s.area(), 16.0);
        assert_eq!(s.side(), 4.0);
    }

    #[test]
    fn delegated_translate_moves_base_state() {
        let mut c = Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0);
        c.translate(2.0, 3.0);
        assert_eq!(c.base.origin, Point::new(2.0, 3.0));
    }
}
