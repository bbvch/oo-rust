//! # 5. Data-oriented: entities, components, systems
//!
//! The answer that stops asking "how do I inherit?" and asks "what does this
//! entity *have*?". State lives in parallel arrays keyed by a handle; behaviour
//! lives in free functions ("systems") that sweep those arrays.
//!
//! What inheritance concepts map to:
//!
//! | C++                        | here                                        |
//! |----------------------------|---------------------------------------------|
//! | base class members         | a component table every entity has          |
//! | derived class members      | a component table only some entities have   |
//! | virtual call in a loop     | one loop per component table, no dispatch   |
//! | multiple inheritance       | an entity registered in several tables      |
//! | `dynamic_cast`             | a table lookup that returns `Option`        |
//!
//! The payoff is layout, not syntax: the "base" fields of all shapes are
//! contiguous, so the transform system touches only the bytes it needs. This is
//! the shape of `bevy_ecs`, `hecs`, `flecs`; hand-rolled here to keep it legible.

use crate::common::{banner, Color, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(usize);

/// Struct-of-arrays: the "base class" state, one array per field.
#[derive(Default)]
pub struct Transforms {
    pub names: Vec<String>,
    pub origins: Vec<Point>,
    pub colors: Vec<Color>,
}

/// Components only *some* entities carry — the "derived" state.
#[derive(Default)]
pub struct CircleShape {
    pub owners: Vec<Entity>,
    pub radii: Vec<f64>,
}

#[derive(Default)]
pub struct RectShape {
    pub owners: Vec<Entity>,
    pub sizes: Vec<(f64, f64)>,
}

#[derive(Default)]
pub struct World {
    pub transforms: Transforms,
    pub circles: CircleShape,
    pub rects: RectShape,
}

impl World {
    fn spawn(&mut self, name: &str, origin: Point, color: Color) -> Entity {
        self.transforms.names.push(name.to_owned());
        self.transforms.origins.push(origin);
        self.transforms.colors.push(color);
        Entity(self.transforms.names.len() - 1)
    }

    pub fn spawn_circle(&mut self, origin: Point, color: Color, radius: f64) -> Entity {
        let e = self.spawn("Circle", origin, color);
        self.circles.owners.push(e);
        self.circles.radii.push(radius);
        e
    }

    pub fn spawn_rectangle(&mut self, origin: Point, color: Color, w: f64, h: f64) -> Entity {
        let e = self.spawn("Rectangle", origin, color);
        self.rects.owners.push(e);
        self.rects.sizes.push((w, h));
        e
    }

    /// A "square" is not a type at all — it is a rectangle whose data happens
    /// to be square. The Liskov argument evaporates because there is no subtype.
    pub fn spawn_square(&mut self, origin: Point, color: Color, side: f64) -> Entity {
        let e = self.spawn("Square", origin, color);
        self.rects.owners.push(e);
        self.rects.sizes.push((side, side));
        e
    }

    /// System: "inherited" behaviour applied to every entity in one linear pass.
    /// No dispatch, no per-object branch, no pointer chasing.
    pub fn translate_all(&mut self, dx: f64, dy: f64) {
        for p in self.transforms.origins.iter_mut() {
            p.x += dx;
            p.y += dy;
        }
    }

    /// System: one tight loop per component kind, each over contiguous data.
    pub fn areas(&self) -> Vec<(Entity, f64)> {
        let mut out = Vec::with_capacity(self.circles.owners.len() + self.rects.owners.len());
        for (&e, &r) in self.circles.owners.iter().zip(&self.circles.radii) {
            out.push((e, std::f64::consts::PI * r * r));
        }
        for (&e, &(w, h)) in self.rects.owners.iter().zip(&self.rects.sizes) {
            out.push((e, w * h));
        }
        out.sort_by_key(|(e, _)| e.0);
        out
    }

    /// The `dynamic_cast` replacement: ask whether the entity has the component.
    pub fn radius_of(&self, e: Entity) -> Option<f64> {
        let i = self.circles.owners.iter().position(|&o| o == e)?;
        Some(self.circles.radii[i])
    }

    pub fn label(&self, e: Entity) -> String {
        let p = self.transforms.origins[e.0];
        format!(
            "{} ({:?}) at ({:.1}, {:.1})",
            self.transforms.names[e.0], self.transforms.colors[e.0], p.x, p.y
        )
    }
}

pub fn demo() {
    banner("5. Data-oriented (entity / component / system)");
    let mut w = World::default();
    w.spawn_circle(Point::new(0.0, 0.0), Color::Red, 2.0);
    w.spawn_rectangle(Point::new(1.0, 1.0), Color::Green, 3.0, 4.0);
    let sq = w.spawn_square(Point::new(2.0, 2.0), Color::Blue, 3.0);

    w.translate_all(0.5, 0.5); // one pass over all "base" state

    for (e, area) in w.areas() {
        println!("{} covers {:.2} units^2", w.label(e), area);
    }
    println!("radius_of(square) = {:?}  <- no such component", w.radius_of(sq));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn systems_apply_shared_behaviour_to_every_entity() {
        let mut w = World::default();
        w.spawn_circle(Point::new(0.0, 0.0), Color::Red, 1.0);
        w.spawn_square(Point::new(1.0, 1.0), Color::Blue, 2.0);
        w.translate_all(1.0, 1.0);
        assert_eq!(w.transforms.origins[0], Point::new(1.0, 1.0));
        assert_eq!(w.transforms.origins[1], Point::new(2.0, 2.0));
    }

    #[test]
    fn area_system_covers_all_component_tables() {
        let mut w = World::default();
        w.spawn_circle(Point::new(0.0, 0.0), Color::Red, 1.0);
        w.spawn_rectangle(Point::new(0.0, 0.0), Color::Green, 2.0, 3.0);
        let total: f64 = w.areas().iter().map(|(_, a)| a).sum();
        assert!((total - (std::f64::consts::PI + 6.0)).abs() < 1e-9);
    }

    #[test]
    fn component_lookup_replaces_dynamic_cast() {
        let mut w = World::default();
        let c = w.spawn_circle(Point::new(0.0, 0.0), Color::Red, 7.0);
        let r = w.spawn_rectangle(Point::new(0.0, 0.0), Color::Green, 1.0, 1.0);
        assert_eq!(w.radius_of(c), Some(7.0));
        assert_eq!(w.radius_of(r), None);
    }
}
