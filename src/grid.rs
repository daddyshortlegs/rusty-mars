struct Coordinate {
    x: i32,
    y: i32
}

pub struct Grid {
    width: i32,
    height: i32,
    coordinate: Coordinate,
}

impl Grid{
    pub fn new() -> Grid {
        Grid { width: 10, height: 10, coordinate:Coordinate { x: 0, y: 0 } }
    }

    pub fn east(&mut self) {
        self.coordinate.x += 1;
        if self.coordinate.x >= self.width {
            self.coordinate.x = 0;
        }
    }

    pub fn south(&mut self) {
        if self.coordinate.y > 0 {
            self.coordinate.y -= 1;
        } else {
            self.coordinate.y = self.height - 1;
        }
    }

    pub fn west(&mut self) {
        if self.coordinate.x > 0 {
            self.coordinate.x -= 1;
        } else {
            self.coordinate.x = self.height - 1;
        }
    }

    pub fn north(&mut self) {
        self.coordinate.y += 1;
        if self.coordinate.y >= self.height {
            self.coordinate.y = 0;
        }
    }

    pub fn position(&mut self) -> String {
        format!("{}:{}", self.coordinate.x, self.coordinate.y)
    }

}
