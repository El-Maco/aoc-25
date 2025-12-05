mod utils;

fn largest_joltage(bank: &&str) -> u32 {
    let max_first_digit = bank[0..bank.len() - 1]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .max()
        .unwrap();
    let max_first_digit_pos = bank[0..bank.len() - 1]
        .chars()
        .position(|c| c.to_digit(10).unwrap() == max_first_digit)
        .unwrap();

    let max_second_digit = bank[max_first_digit_pos + 1..bank.len()]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .max()
        .unwrap();

    let mut max_digit_str = String::new();
    max_digit_str.push_str(&max_first_digit.to_string());
    max_digit_str.push_str(&max_second_digit.to_string());

    let result = max_digit_str.parse().unwrap();
    println!("Bank: {}, Largest Joltage: {}", bank, result);
    result
}

fn solve_part1(input: &str) -> u32 {
    let banks = utils::parse_input(input);
    banks.iter().map(largest_joltage).sum()
}

fn solve_part2(input: &str) -> u32 {
    0
}

fn main() {
    let input_text: String = utils::load_input(3);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 357);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert!(result == 0);
    }
}
