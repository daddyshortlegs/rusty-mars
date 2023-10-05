use crate::compass::Compass;

struct Coordinate {
    x: i32,
    y: i32
}

pub struct Rover {
    coordinate: Coordinate,
    compass: Compass
}

impl Rover {
    pub fn new() -> Rover {
        Rover{coordinate:Coordinate { x: 0, y: 0 }, compass: Compass::new()}
    }

    pub fn turn_left(&mut self) {
        self.compass.left();
    }

    pub fn turn_right(&mut self) {
        self.compass.right();
    }

    pub fn forward(&mut self) {
        if self.compass.get_direction() == "E" {
            self.coordinate.x += 1;
            if self.coordinate.x > 9 {
                self.coordinate.x = 0;
            }
        } else if self.compass.get_direction() == "S" {
            if self.coordinate.y > 0 {
                self.coordinate.y -= 1;
            } else {
                self.coordinate.y = 9;
            }
        } else if self.compass.get_direction() == "W" {
            if self.coordinate.x > 0 {
                self.coordinate.x -= 1;
            } else {
                self.coordinate.x = 9;
            }
        } else {
            self.coordinate.y += 1;
            if self.coordinate.y > 9 {
                self.coordinate.y = 0;
            }
        }
    }

    pub fn location(&mut self) -> String {
        format!("{}:{}:{}", self.coordinate.x, self.coordinate.y, self.compass.get_direction())
    }
}
