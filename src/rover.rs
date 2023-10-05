use crate::compass::Compass;
use crate::grid::Grid;


pub struct Rover {
    compass: Compass,
    grid: Grid
}

impl Rover {
    pub fn new() -> Rover {
        Rover{compass: Compass::new(), grid: Grid::new()}
    }

    pub fn turn_left(&mut self) {
        self.compass.left();
    }

    pub fn turn_right(&mut self) {
        self.compass.right();
    }

    pub fn forward(&mut self) {
        if self.compass.get_direction() == "E" {
            self.grid.east();
        } else if self.compass.get_direction() == "S" {
            self.grid.south();
        } else if self.compass.get_direction() == "W" {
            self.grid.west();
        } else {
            self.grid.north();
        }
    }

    pub fn location(&mut self) -> String {
        format!("{}:{}", self.grid.position(), self.compass.get_direction())
    }
}
