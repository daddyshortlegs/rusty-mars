use crate::compass::Compass;

pub struct Rover {
    x: i32,
    y: i32,
    compass: Compass
}

impl Rover {
    pub fn new() -> Rover {
        Rover{x: 0, y: 0, compass: Compass::new()}
    }

    pub fn turn_left(&mut self) {
        self.compass.left();
    }

    pub fn turn_right(&mut self) {
        self.compass.right();
    }

    pub fn forward(&mut self) {
        if self.compass.get_direction() == "E" {
            self.x += 1;
            if self.x > 9 {
                self.x = 0;
            }
        } else if self.compass.get_direction() == "S" {
            if self.y > 0 {
                self.y -= 1;
            } else {
                self.y = 9;
            }
        } else if self.compass.get_direction() == "W" {
            if self.x > 0 {
                self.x -= 1;
            } else {
                self.x = 9;
            }
        } else {
            self.y += 1;
            if self.y > 9 {
                self.y = 0;
            }
        }
    }

    pub fn location(&mut self) -> String {
        format!("{}:{}:{}", self.x, self.y, self.compass.get_direction())
    }
}
