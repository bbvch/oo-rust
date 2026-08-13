#[derive(Default, Clone, PartialEq, Debug)]
pub struct Rectangle {
    top_left_x: f32,
    top_left_y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(top_left: (f32, f32), width: f32, height: f32) -> Self {
        let mut rectangle = Self {
            top_left_x: top_left.0,
            top_left_y: top_left.1,
            width,
            height,
        };
        rectangle.validate_data();
        rectangle
    }

    pub fn get_top_left(&self) -> (f32, f32) {
        (self.top_left_x, self.top_left_y)
    }

    pub fn set_top_left(&mut self, top_left: (f32, f32)) {
        self.top_left_x = top_left.0;
        self.top_left_y = top_left.1;
        self.validate_data();
    }

    pub fn get_bottom_right(&self) -> (f32, f32) {
        (self.top_left_x + self.width, self.top_left_y + self.height)
    }

    pub fn set_bottom_right(&mut self, bottom_right: (f32, f32)) {
        self.width = bottom_right.0 - self.top_left_x;
        self.height = bottom_right.1 - self.top_left_y;
        self.validate_data();
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.validate_data();
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
        self.validate_data();
    }

    fn validate_data(&mut self) {
        self.width = self.width.max(0.0);
        self.height = self.height.max(0.0);
    }
}

impl super::Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    fn center(&self) -> (f32, f32) {
        (
            self.top_left_x + self.width / 2.0,
            self.top_left_y + self.height / 2.0,
        )
    }
}
