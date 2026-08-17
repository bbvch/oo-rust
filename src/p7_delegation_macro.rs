//! # 7. Macro-generated delegation
//!
//! Pattern 1's objection to `Deref` was never "forwarding is wrong", it was
//! "forwarding should be visible". Macros keep it visible — at the type
//! declaration, where a C++ reader would look for `: public Rectangle` — while
//! deleting the typing.
//!
//! * `ambassador::Delegate` — derive a *trait impl* by forwarding to a field.
//!   `#[delegate(Shape, target = "rect")]` reads exactly like a base clause and
//!   is checked at compile time against the trait's real signature.
//! * `delegate::delegate!` — forward *inherent* methods, with the freedom to
//!   rename, change visibility, or post-process the result.
//!
//! Unlike `Deref`: no method resolution surprises, no `&Square` silently
//! coercing to `&Rectangle`, and the set of inherited methods is enumerable by
//! reading the type.

use crate::common::{banner, Color, Point, ShapeData};
use ambassador::{delegatable_trait, Delegate};
use delegate::delegate;

#[delegatable_trait]
pub trait HasShapeData {
    fn data(&self) -> &ShapeData;
    fn data_mut(&mut self) -> &mut ShapeData;
}

#[delegatable_trait]
pub trait Shape: HasShapeData {
    fn area(&self) -> f64;
    fn describe(&self) -> String;
}

pub struct Rectangle {
    base: ShapeData,
    pub width: f64,
    pub height: f64,
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

impl HasShapeData for Rectangle {
    fn data(&self) -> &ShapeData {
        &self.base
    }
    fn data_mut(&mut self) -> &mut ShapeData {
        &mut self.base
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn describe(&self) -> String {
        format!("{} covers {:.2} units^2", self.data().label(), self.area())
    }
}

/// `class Square : public Rectangle` — the base clause is the `#[delegate]`
/// attribute, and it inherits both traits' implementations wholesale.
#[derive(Delegate)]
#[delegate(HasShapeData, target = "rect")]
#[delegate(Shape, target = "rect")]
pub struct Square {
    rect: Rectangle,
}

impl Square {
    pub fn new(origin: Point, color: Color, side: f64) -> Self {
        let mut rect = Rectangle::new(origin, color, side, side);
        rect.base.name = "Square".to_owned();
        Self { rect }
    }

    // Inherent methods forwarded with `delegate!`, including a renamed one —
    // something a `Deref` impl cannot express.
    delegate! {
        to self.rect {
            #[call(area)]
            pub fn surface(&self) -> f64;
            pub fn describe(&self) -> String;
        }
    }

    pub fn side(&self) -> f64 {
        self.rect.width
    }
}

pub fn demo() {
    banner("7. Macro-generated delegation (ambassador + delegate)");
    let mut sq = Square::new(Point::new(1.0, 1.0), Color::Blue, 3.0);
    sq.data_mut().translate(1.0, 1.0);
    println!("{}", Shape::describe(&sq));
    println!("renamed inherent forward: surface() = {:.2}", sq.surface());

    // Delegation produced a real `Shape` impl, so trait objects work.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(Point::new(0.0, 0.0), Color::Green, 2.0, 5.0)),
        Box::new(Square::new(Point::new(0.0, 0.0), Color::Blue, 4.0)),
    ];
    for s in &shapes {
        println!("{}", s.describe());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_delegation_implements_the_trait_for_real() {
        let s = Square::new(Point::new(0.0, 0.0), Color::Blue, 3.0);
        let as_shape: &dyn Shape = &s;
        assert_eq!(as_shape.area(), 9.0);
    }

    #[test]
    fn delegated_state_access_is_shared_with_the_inner_value() {
        let mut s = Square::new(Point::new(0.0, 0.0), Color::Blue, 2.0);
        s.data_mut().translate(5.0, 5.0);
        assert_eq!(s.rect.data().origin, Point::new(5.0, 5.0));
    }

    #[test]
    fn inherent_forward_can_rename() {
        let s = Square::new(Point::new(0.0, 0.0), Color::Blue, 5.0);
        assert_eq!(s.surface(), 25.0);
        assert_eq!(s.side(), 5.0);
    }
}
