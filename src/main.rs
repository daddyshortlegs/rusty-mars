fn main() {
}

fn execute(commands: &str) -> String {
    let mut y = 0;

    for command in commands.chars() {
        y+=1
    }

    format!("0:{}:N", y)
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