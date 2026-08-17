//! # 4. Struct + enum: a closed hierarchy
//!
//! When the set of "derived classes" is known and closed, model it as data
//! rather than as types. The shared base state stays a plain field; only the
//! variant-specific state goes in the enum. Dispatch is a `match`.
//!
//! This is where "data driven" starts: the shape's kind is a *value*, so it can
//! be stored, sent over a wire, deserialised from config, and matched
//! exhaustively — a new variant makes the compiler point at every site that
//! must handle it. C++ inheritance gives you none of that; it trades it for
//! open extensibility, which is exactly what you lose here.

use crate::common::{banner, Color, Point, ShapeData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeKind {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Square { side: f64 },
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub data: ShapeData,
    pub kind: ShapeKind,
}

impl Shape {
    pub fn circle(origin: Point, color: Color, radius: f64) -> Self {
        Self {
            data: ShapeData::new("Circle", origin, color),
            kind: ShapeKind::Circle { radius },
        }
    }

    pub fn rectangle(origin: Point, color: Color, width: f64, height: f64) -> Self {
        Self {
            data: ShapeData::new("Rectangle", origin, color),
            kind: ShapeKind::Rectangle { width, height },
        }
    }

    pub fn square(origin: Point, color: Color, side: f64) -> Self {
        Self {
            data: ShapeData::new("Square", origin, color),
            kind: ShapeKind::Square { side },
        }
    }

    /// The "virtual" method — one place, exhaustively checked.
    pub fn area(&self) -> f64 {
        match self.kind {
            ShapeKind::Circle { radius } => std::f64::consts::PI * radius * radius,
            ShapeKind::Rectangle { width, height } => width * height,
            // Reusing a sibling's logic is a function call, not a base class.
            ShapeKind::Square { side } => side * side,
        }
    }

    /// Base behaviour: written once, no trait, no dispatch, no boilerplate.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.data.translate(dx, dy);
    }

    pub fn describe(&self) -> String {
        format!("{} covers {:.2} units^2", self.data.label(), self.area())
    }
}

pub fn demo() {
    banner("4. Struct + enum (closed hierarchy)");
    let mut shapes = vec![
        Shape::circle(Point::new(0.0, 0.0), Color::Red, 2.0),
        Shape::rectangle(Point::new(1.0, 1.0), Color::Green, 3.0, 4.0),
        Shape::square(Point::new(2.0, 2.0), Color::Blue, 3.0),
    ];
    for s in shapes.iter_mut() {
        s.translate(0.5, 0.5);
        println!("{}", s.describe());
    }
    // Contiguous storage, no Box, no vtable: shapes is a flat Vec<Shape>.
    println!(
        "sizeof Shape = {} bytes, all {} live in one allocation",
        std::mem::size_of::<Shape>(),
        shapes.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas_match_the_variant() {
        assert_eq!(
            Shape::rectangle(Point::new(0.0, 0.0), Color::Red, 2.0, 5.0).area(),
            10.0
        );
        assert_eq!(
            Shape::square(Point::new(0.0, 0.0), Color::Red, 4.0).area(),
            16.0
        );
    }

    #[test]
    fn base_behaviour_is_shared_without_a_trait() {
        let mut s = Shape::square(Point::new(0.0, 0.0), Color::Blue, 1.0);
        s.translate(3.0, -1.0);
        assert_eq!(s.data.origin, Point::new(3.0, -1.0));
    }

    #[test]
    fn shapes_are_values_no_boxing_needed() {
        let shapes = vec![
            Shape::circle(Point::new(0.0, 0.0), Color::Red, 1.0),
            Shape::square(Point::new(0.0, 0.0), Color::Blue, 2.0),
        ];
        let total: f64 = shapes.iter().map(Shape::area).sum();
        assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
    }
}
