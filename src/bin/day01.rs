mod utils;

const STARTING_DIAL: u32 = 50;

fn parse_row(row: &&str) -> i32 {
    match row.chars().nth(0).unwrap() {
        'L' => -1 * row[1..].parse::<i32>().unwrap(),
        'R' => row[1..].parse::<i32>().unwrap(),
        _ => panic!("Invalid direction: {}", row),
    }
}

fn solve_part1(input: &str) -> u32 {
    let input_rows: Vec<&str> = utils::parse_input(&input);
    let directions: Vec<i32> = input_rows.iter().map(parse_row).collect();
    let mut current_dial = STARTING_DIAL as i32;

    let mut count: u32 = 0;
    directions.iter().for_each(|step| {
        current_dial += step;
        current_dial %= 100;
        if current_dial < 0 {
            current_dial += 100;
        }
        if current_dial >= 100 {
            current_dial -= 100;
        }
        if current_dial == 0 {
            count += 1;
        }
    });
    count
}

fn main() {
    let input_text: String = utils::load_input(1);
    let password = solve_part1(&input_text);
    println!("Part 1: The password is {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row(&"L10"), -10);
        assert_eq!(parse_row(&"R25"), 25);
    }

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 3);
    }
}
