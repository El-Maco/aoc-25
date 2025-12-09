mod utils;

fn solve_part1(input: &str) -> u64 {
    let rows = input
        .lines()
        .map(|r| {
            let mut split = r.split(',');
            let x: usize = split.next().unwrap().parse().unwrap();
            let y: usize = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut largest_area = 0;
    rows.iter().for_each(|&(first_x, first_y)| {
        rows.iter().for_each(|&(second_x, second_y)| {
            if first_y != second_y && first_x != second_x {
                let area = (((first_x as isize - second_x as isize).abs() + 1)
                    * ((first_y as isize - second_y as isize).abs() + 1))
                    as u64;
                if area > largest_area {
                    largest_area = area;
                }
            }
        });
    });
    largest_area
}

fn solve_part2(input: &str) -> u64 {
    0
}

fn main() {
    let day = 9;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 50);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 0);
    }
}
