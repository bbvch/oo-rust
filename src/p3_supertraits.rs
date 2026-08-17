//! # 3. Supertraits, trait objects, upcasting, downcasting
//!
//! Interface inheritance without any state inheritance: `trait Drawable: Shape`
//! is `class Drawable : public virtual Shape`. Two facilities complete the C++
//! analogy:
//!
//! * **upcast** `&dyn Drawable -> &dyn Shape` — trait upcasting, stable since
//!   Rust 1.86, so no more hand-written `fn as_shape(&self) -> &dyn Shape`.
//! * **downcast** via `dyn Any` — the `dynamic_cast<Derived*>` analogue, with
//!   the same "you probably have a design problem" smell attached.

use crate::common::{banner, Color, Point, ShapeData};
use std::any::Any;

pub trait Shape {
    fn data(&self) -> &ShapeData;
    fn area(&self) -> f64;
}

/// Every `Drawable` is a `Shape`; the vtable of `dyn Drawable` embeds it.
pub trait Drawable: Shape {
    fn draw(&self) -> String;
}

/// Opt-in downcasting: implementors expose themselves as `dyn Any`.
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub struct Circle {
    base: ShapeData,
    pub radius: f64,
}

pub struct Rectangle {
    base: ShapeData,
    pub width: f64,
    pub height: f64,
}

impl Circle {
    pub fn new(origin: Point, color: Color, radius: f64) -> Self {
        Self {
            base: ShapeData::new("Circle", origin, color),
            radius,
        }
    }
}

impl Rectangle {
    pub fn new(origin: Point, color: Color, width: f64, height: f64) -> Self {
        Self {
            base: ShapeData::new("Rectangle", origin, color),
            width,
            height,
        }
    }
}

impl Shape for Circle {
    fn data(&self) -> &ShapeData {
        &self.base
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn data(&self) -> &ShapeData {
        &self.base
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("(o) r={}", self.radius)
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("[] {}x{}", self.width, self.height)
    }
}

impl AsAny for Circle {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AsAny for Rectangle {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Accepts the *base* interface; callers pass the derived one and it upcasts.
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// `dynamic_cast<const Circle*>` in Rust clothing.
pub fn radius_of(shape: &dyn AsAny) -> Option<f64> {
    shape.as_any().downcast_ref::<Circle>().map(|c| c.radius)
}

pub fn demo() {
    banner("3. Supertraits + upcasting + downcasting");
    let c = Circle::new(Point::new(0.0, 0.0), Color::Red, 2.0);
    let r = Rectangle::new(Point::new(0.0, 0.0), Color::Green, 3.0, 4.0);

    let drawables: Vec<&dyn Drawable> = vec![&c, &r];
    for d in &drawables {
        println!("{} -> {} area {:.2}", d.data().label(), d.draw(), d.area());
    }

    // Upcast &dyn Drawable -> &dyn Shape, no helper method needed.
    let as_shapes: Vec<&dyn Shape> = drawables.iter().map(|d| *d as &dyn Shape).collect();
    println!("total area via base interface: {:.2}", total_area(&as_shapes));

    println!("downcast to Circle: {:?}", radius_of(&c));
    println!("downcast to Circle: {:?}", radius_of(&r));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supertrait_methods_are_callable_on_the_subtrait_object() {
        let c = Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0);
        let d: &dyn Drawable = &c;
        assert!((d.area() - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn upcasting_preserves_dispatch() {
        let r = Rectangle::new(Point::new(0.0, 0.0), Color::Green, 2.0, 3.0);
        let d: &dyn Drawable = &r;
        let s: &dyn Shape = d;
        assert_eq!(s.area(), 6.0);
    }

    #[test]
    fn downcast_succeeds_only_for_the_real_type() {
        let c = Circle::new(Point::new(0.0, 0.0), Color::Red, 5.0);
        let r = Rectangle::new(Point::new(0.0, 0.0), Color::Green, 1.0, 1.0);
        assert_eq!(radius_of(&c), Some(5.0));
        assert_eq!(radius_of(&r), None);
    }
}
