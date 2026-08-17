//! # 2. Accessor trait + provided methods  — the workhorse replacement for `Deref`
//!
//! Split what C++ inheritance fuses into one mechanism:
//!
//! * `HasShapeData` — access to the shared *state* (C++ protected members).
//!   Each type implements exactly one method: "where do I keep my base?"
//! * `Shape: HasShapeData` — the *behaviour*. Required methods are the pure
//!   virtuals; provided (default) methods are the non-virtual base methods that
//!   every implementor inherits for free.
//!
//! `describe` is the Template Method / NVI pattern: a provided method that calls
//! `self.area()`, so base-defined behaviour dispatches into derived overrides.
//! This is the pattern to reach for first in real code.

use crate::common::{banner, Color, Point, ShapeData};

pub trait HasShapeData {
    fn data(&self) -> &ShapeData;
    fn data_mut(&mut self) -> &mut ShapeData;
}

pub trait Shape: HasShapeData {
    /// Pure virtual.
    fn area(&self) -> f64;

    /// Non-virtual base method, inherited by every implementor.
    fn translate(&mut self, dx: f64, dy: f64) {
        self.data_mut().translate(dx, dy);
    }

    /// Template method: fixed skeleton in the "base", the varying step
    /// (`area`) resolved on the concrete type.
    fn describe(&self) -> String {
        format!("{} covers {:.2} units^2", self.data().label(), self.area())
    }
}

/// Removes the only boilerplate this pattern has left.
macro_rules! impl_has_shape_data {
    ($t:ty, $field:ident) => {
        impl HasShapeData for $t {
            fn data(&self) -> &ShapeData {
                &self.$field
            }
            fn data_mut(&mut self) -> &mut ShapeData {
                &mut self.$field
            }
        }
    };
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

/// Concrete-to-concrete reuse: `Square` embeds a `Rectangle` and forwards
/// `area`, so `Rectangle`'s implementation is genuinely shared, not copied.
pub struct Square {
    rect: Rectangle,
}

impl_has_shape_data!(Circle, base);
impl_has_shape_data!(Rectangle, base);

impl HasShapeData for Square {
    fn data(&self) -> &ShapeData {
        self.rect.data()
    }
    fn data_mut(&mut self) -> &mut ShapeData {
        self.rect.data_mut()
    }
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

impl Square {
    pub fn new(origin: Point, color: Color, side: f64) -> Self {
        let mut rect = Rectangle::new(origin, color, side, side);
        rect.base.name = "Square".to_owned();
        Self { rect }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.rect.area()
    }

    /// Overriding a provided method is the exact analogue of overriding a
    /// virtual with a base implementation.
    fn describe(&self) -> String {
        format!("{} (a perfect square)", self.rect.describe())
    }
}

/// Generic code: static dispatch, monomorphized, no vtable.
pub fn shift_and_report<S: Shape>(shape: &mut S, dx: f64, dy: f64) -> String {
    shape.translate(dx, dy);
    shape.describe()
}

pub fn demo() {
    banner("2. Accessor trait + provided methods");
    let mut shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(Point::new(0.0, 0.0), Color::Red, 2.0)),
        Box::new(Rectangle::new(Point::new(1.0, 1.0), Color::Green, 3.0, 4.0)),
        Box::new(Square::new(Point::new(2.0, 2.0), Color::Blue, 3.0)),
    ];
    for s in shapes.iter_mut() {
        s.translate(0.5, 0.5); // inherited, not overridden
        println!("{}", s.describe());
    }
    let mut c = Circle::new(Point::new(9.0, 9.0), Color::Red, 1.0);
    println!("static dispatch: {}", shift_and_report(&mut c, 1.0, 1.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_method_is_inherited_by_all() {
        let mut c = Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0);
        c.translate(1.0, 2.0);
        assert_eq!(c.data().origin, Point::new(1.0, 2.0));
    }

    #[test]
    fn template_method_dispatches_to_override() {
        let r = Rectangle::new(Point::new(0.0, 0.0), Color::Green, 2.0, 5.0);
        assert!(r.describe().contains("10.00"));
    }

    #[test]
    fn overriding_a_provided_method_can_call_the_parents() {
        let s = Square::new(Point::new(0.0, 0.0), Color::Blue, 3.0);
        assert!(s.describe().contains("9.00"));
        assert!(s.describe().ends_with("(a perfect square)"));
    }

    #[test]
    fn works_through_trait_objects() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle::new(Point::new(0.0, 0.0), Color::Red, 1.0)),
            Box::new(Square::new(Point::new(0.0, 0.0), Color::Blue, 2.0)),
        ];
        let total: f64 = shapes.iter().map(|s| s.area()).sum();
        assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
    }
}
