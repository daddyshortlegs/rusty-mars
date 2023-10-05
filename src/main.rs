mod compass;
mod rover;

use crate::rover::Rover;

fn main() {}

fn execute(commands: &str) -> String {
    let mut rover = Rover::new();

    for command in commands.chars() {
        if command == 'L' {
            rover.turn_left();
        } else if command == 'R' {
            rover.turn_right();
        } else {
            rover.forward();
        }
    }

    rover.location()
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
    fn should_move_north_and_wrap_around() {
        let result = execute("MMMMMMMMMM");
        assert_eq!(result, "0:0:N")
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

    #[test]
    fn should_move_east() {
        let result = execute("RM");
        assert_eq!(result, "1:0:E")
    }

    #[test]
    fn should_move_east_twice() {
        let result = execute("RMM");
        assert_eq!(result, "2:0:E")
    }

    #[test]
    fn should_move_east_and_wrap_around() {
        let result = execute("RMMMMMMMMMM");
        assert_eq!(result, "0:0:E")
    }

    #[test]
    fn should_move_south() {
        let result = execute("RRM");
        assert_eq!(result, "0:9:S")
    }

    #[test]
    fn should_move_south_twice() {
        let result = execute("RRMM");
        assert_eq!(result, "0:8:S")
    }

    #[test]
    fn should_move_west() {
        let result = execute("LM");
        assert_eq!(result, "9:0:W")
    }

    #[test]
    fn should_move_west_twice() {
        let result = execute("LMM");
        assert_eq!(result, "8:0:W")
    }

    #[test]
    fn should_move_around_grid() {
        let result = execute("MMRMMLM");
        assert_eq!(result, "2:3:N")
    }

}