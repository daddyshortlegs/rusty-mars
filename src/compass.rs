pub struct Compass {
    directions: [char; 4],
    index: usize,
}

impl Compass {
    pub fn new() -> Compass {
        Compass{index: 0, directions: ['N', 'E', 'S', 'W']}
    }

    pub fn left(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = 3;
        }
    }

    pub fn right(&mut self) {
        self.index += 1;
        if self.index > 3 {
            self.index = 0;
        }
    }

    pub fn get_direction(&mut self) -> String {
        self.directions[self.index].to_string()
    }
}
