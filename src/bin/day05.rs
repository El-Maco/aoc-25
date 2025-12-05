mod utils;

fn solve_part1(input: &str) -> u64 {
    let rows = utils::parse_input(input);
    let blank_link_pos = rows.iter().position(|r| r.is_empty()).unwrap();
    let ranges: Vec<(u64, u64)> = rows[..blank_link_pos]
        .iter()
        .map(|r| {
            let parts: Vec<&str> = r.split('-').collect();
            (
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            )
        })
        .collect();
    let ids: Vec<u64> = rows[blank_link_pos + 1..]
        .iter()
        .map(|r| r.parse::<u64>().unwrap())
        .collect();

    ids.iter().filter(|id| {
        ranges.iter().any(|(start, end)| id >= &start && id <= &end)
    }).count() as u64
}

fn solve_part2(input: &str) -> u64 {
    0
}

fn main() {
    let day = 5;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
111
2222";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 0);
    }
}
