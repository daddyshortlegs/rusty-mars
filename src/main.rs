fn main() {
}

fn execute(commands: &str) -> String {
    if commands == "MM" {
        return "0:2:N".to_string()
    } else if commands == "M" {
        return "0:1:N".to_string()
    }
    "0:0:N".to_string()
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
}