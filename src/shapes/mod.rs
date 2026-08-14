pub mod circle;
pub mod rectangle;
pub mod square;

pub trait Shape: std::ops::Deref<Target = ShapeData> {
    fn label(&self) -> &str {
        self.get_label()
    }
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn center(&self) -> (f32, f32);
    fn distance_from_origin(&self) -> f32 {
        let (x, y) = self.center();
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ShapeData {
    label: String,
}

impl ShapeData {
    pub fn new(label: String) -> Self {
        ShapeData { label }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}
