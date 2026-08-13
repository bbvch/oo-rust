#[derive(Default, Clone, PartialEq, Debug)]
pub struct Circle {
    center_x: f32,
    center_y: f32,
    radius: f32,
}

impl Circle {
    pub fn new(center: (f32, f32), radius: f32) -> Self {
        let mut circle = Self {
            center_x: center.0,
            center_y: center.1,
            radius,
        };
        circle.validate_data();
        circle
    }

    pub fn get_center(&self) -> (f32, f32) {
        (self.center_x, self.center_y)
    }

    pub fn set_center(&mut self, center: (f32, f32)) {
        self.center_x = center.0;
        self.center_y = center.1;
        self.validate_data();
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.validate_data();
    }

    fn validate_data(&mut self) {
        self.radius = self.radius.max(0.0);
    }
}

impl super::Shape for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn center(&self) -> (f32, f32) {
        self.get_center()
    }
}
