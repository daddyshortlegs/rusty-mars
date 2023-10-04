fn main() {
}

fn execute(commands: &str) -> String {
    if commands == "M" {
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
}