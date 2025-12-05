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

    ids.iter()
        .filter(|id| ranges.iter().any(|(start, end)| id >= &start && id <= &end))
        .count() as u64
}

fn solve_part2(input: &str) -> u64 {
    let rows = utils::parse_input(input);
    let blank_link_pos = rows.iter().position(|r| r.is_empty()).unwrap();
    let mut ranges: Vec<(u64, u64)> = rows[..blank_link_pos]
        .iter()
        .map(|r| {
            let parts: Vec<&str> = r.split('-').collect();
            (
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            )
        })
        .collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    loop {
        println!("Current ranges: {:?}", ranges);
        let mut i = 0;
        let mut merged = false;
        while i < ranges.len() {
            let mut j = i + 1;
            while j < ranges.len() {
                let (start_i, end_i) = ranges[i];
                let (start_j, end_j) = ranges[j];

                if start_i <= start_j && end_i >= end_j {
                    println!(
                        "Removing range {:?} fully enclosed by {:?}",
                        (start_j, end_j),
                        (start_i, end_i)
                    );
                    ranges.remove(j);
                    merged = true;
                    continue; // with same j (next range)
                }

                if start_i <= start_j && end_i >= start_j || start_i <= end_j && end_i >= end_j {
                    println!(
                        "Merging range {:?} due to {:?} due to overlap",
                        (start_j, end_j),
                        (start_i, end_i)
                    );
                    let new_start = start_i.min(start_j);
                    let new_end = end_i.max(end_j);
                    ranges[i] = (new_start, new_end);
                    ranges.remove(j);
                    merged = true;
                    continue; // with same j (next range)
                }
                // println!("Do nothing");
                j += 1;
            }
            i += 1;
        }
        if !merged {
            break;
        }
    }
    let total_covered: u64 = ranges.iter().map(|(start, end)| end - start + 1).sum();
    println!("Merged ranges: {:?}", ranges);
    total_covered
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
        assert_eq!(result, 14);
    }
}
