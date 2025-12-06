mod utils;

fn apply_operator(a: u64, b: u64, operator: char) -> u64 {
    match operator {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unsupported operator"),
    }
}

fn solve_part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut problems: Vec<Vec<u64>> = Vec::new();
    for line in &lines[0..lines.len() - 1] {
        let row: Vec<u64> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        row.iter().enumerate().for_each(|(i, &num)| {
            if problems.len() <= i {
                problems.push(Vec::new());
            }
            problems.get_mut(i).unwrap().push(num)
        });
    }
    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    problems
        .iter()
        .enumerate()
        .map(|(i, nums)| {
            nums[1..]
                .iter()
                .fold(nums[0], |acc, &num| apply_operator(acc, num, operators[i]))
        })
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let mut problems: Vec<Vec<String>> = Vec::new();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let mut x: usize = 0;
    for i in 0..width {
        let mut new_number: String = String::new();
        let mut all_empty: bool = true;
        for line in lines[0..lines.len() - 1].iter() {
            let c = line.chars().nth(i).unwrap_or(' ');
            if !c.is_whitespace() {
                new_number.push(c);
                all_empty = false;
            }
        }
        if all_empty {
            x += 1;
            continue;
        }
        if problems.len() <= x {
            problems.push(Vec::new());
        }
        problems.get_mut(x).unwrap().push(new_number);
    }
    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    problems
        .iter()
        .enumerate()
        .map(|(i, nums)| {
            let nums: Vec<u64> = nums.iter().map(|s| s.parse().unwrap()).collect();
            nums[1..]
                .iter()
                .fold(nums[0], |acc, &num| apply_operator(acc, num, operators[i]))
        })
        .sum()
}

fn main() {
    let day = 6;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 4277556);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 3263827);
    }
}
