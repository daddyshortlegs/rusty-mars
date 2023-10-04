pub struct Compass {
    direction_index: usize,
    directions: [String; 4],
}

impl Compass {
    pub fn new() -> Compass {
        Compass{direction_index: 0, directions: ["N".to_string(), "E".to_string(), "S".to_string(), "W".to_string()]}
    }

    pub fn left(&mut self) {
        if self.direction_index > 0 {
            self.direction_index -= 1;
        } else {
            self.direction_index = 3;
        }
    }

    pub fn right(&mut self) {
        self.direction_index += 1;
        if self.direction_index > 3 {
            self.direction_index = 0;
        }
    }

    pub fn get_direction(&mut self) -> String {
        self.directions[self.direction_index].to_string()
    }
}
