fn main() {}


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

    fn get_direction(&mut self) -> String {
        self.directions[self.direction_index].to_string()
    }
}


fn execute(commands: &str) -> String {
    let mut y = 0;

    let mut compass = Compass::new();

    for command in commands.chars() {
        if command == 'L' {
            compass.left();
        } else if command == 'R' {
            compass.right();
        } else {
            y += 1;
        }
    }

    format!("0:{}:{}", y, compass.get_direction())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_stay_at_origin() {
        let result = execute("");
        assert_eq!(result, "0:0:N")
    }

    #[test]
    fn should_move() {
        let result = execute("M");
        assert_eq!(result, "0:1:N")
    }

    #[test]
    fn should_move_north_twice() {
        let result = execute("MM");
        assert_eq!(result, "0:2:N")
    }

    #[test]
    fn should_move_north_thrice() {
        let result = execute("MMM");
        assert_eq!(result, "0:3:N")
    }

    #[test]
    fn should_rotate_left() {
        let result = execute("L");
        assert_eq!(result, "0:0:W")
    }

    #[test]
    fn should_rotate_left_twice() {
        let result = execute("LL");
        assert_eq!(result, "0:0:S")
    }

    #[test]
    fn should_rotate_left_thrice() {
        let result = execute("LLL");
        assert_eq!(result, "0:0:E")
    }

    #[test]
    fn should_rotate_left_back_to_start() {
        let result = execute("LLLL");
        assert_eq!(result, "0:0:N")
    }

    #[test]
    fn should_rotate_right() {
        let result = execute("R");
        assert_eq!(result, "0:0:E")
    }

    #[test]
    fn should_rotate_right_twice() {
        let result = execute("RR");
        assert_eq!(result, "0:0:S")
    }

    #[test]
    fn should_rotate_right_thrice() {
        let result = execute("RRR");
        assert_eq!(result, "0:0:W")
    }

    #[test]
    fn should_rotate_right_back_to_start() {
        let result = execute("RRRR");
        assert_eq!(result, "0:0:N")
    }
}