pub mod circle;
pub mod rectangle;
pub mod square;

pub trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn center(&self) -> (f32, f32);
    fn distance_from_origin(&self) -> f32 {
        let (x, y) = self.center();
        (x.powi(2) + y.powi(2)).sqrt()
    }
}
