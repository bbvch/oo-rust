#[derive(Default, Clone, PartialEq, Debug)]
pub struct Square {
    top_left_x: f32,
    top_left_y: f32,
    side_length: f32,
}

impl Square {
    pub fn new(top_left: (f32, f32), side_length: f32) -> Self {
        let mut square = Self {
            top_left_x: top_left.0,
            top_left_y: top_left.1,
            side_length,
        };
        square.validate_data();
        square
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
        (
            self.top_left_x + self.side_length,
            self.top_left_y + self.side_length,
        )
    }

    pub fn get_side_length(&self) -> f32 {
        self.side_length
    }

    pub fn set_side_length(&mut self, side_length: f32) {
        self.side_length = side_length;
        self.validate_data();
    }

    fn validate_data(&mut self) {
        self.side_length = self.side_length.max(0.0);
    }
}

impl super::Shape for Square {
    fn area(&self) -> f32 {
        self.side_length * self.side_length
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.side_length
    }

    fn center(&self) -> (f32, f32) {
        (
            self.top_left_x + self.side_length / 2.0,
            self.top_left_y + self.side_length / 2.0,
        )
    }
}
