mod utils;

fn solve_part1(input: &str) -> u32 {
    0
}

fn solve_part2(input: &str) -> u32 {
    0
}

fn main() {
    let input_text: String = utils::load_input(2);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The password is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The password is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"";
    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert!(result == 0);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert!(result == 0);
    }
}
