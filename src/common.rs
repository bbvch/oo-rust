#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct ShapeData {
    pub name: String,
    pub origin: Point,
    pub color: Color,
}

impl ShapeData {
    pub fn new(name: &str, origin: Point, color: Color) -> Self {
        Self {
            name: name.to_owned(),
            origin,
            color,
        }
    }

    /// A non-virtual base method: behaviour derived classes reuse verbatim.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.origin.x += dx;
        self.origin.y += dy;
    }

    pub fn label(&self) -> String {
        format!(
            "{} ({:?}) at ({:.1}, {:.1})",
            self.name, self.color, self.origin.x, self.origin.y
        )
    }
}

pub fn banner(title: &str) {
    println!("\n=== {title} ===");
}
