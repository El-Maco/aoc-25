use std::collections::{HashMap, HashSet};
mod utils;

fn solve_part1(input: &str) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    let header = rows[0];
    let rows: Vec<&str> = rows[1..].to_vec();

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(header.find('S').unwrap());
    let mut split_count = 0;

    rows.iter().for_each(|row| {
        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut to_add: HashSet<usize> = HashSet::new();
        row.chars().enumerate().for_each(|(i, c)| {
            beams.iter().for_each(|&b| {
                if i == b {
                    match c {
                        '^' => {
                            to_remove.insert(b);
                            split_count += 1;
                            if b > 0 {
                                to_add.insert(b - 1);
                            }
                            if b < row.len() - 1 {
                                to_add.insert(b + 1);
                            }
                        }
                        _ => { }
                    }
                }
            });
        });
        beams.retain(|b| !to_remove.contains(b));
        beams.extend(to_add.iter());
    });

    split_count
}

fn step(rows: &[&str], cache: &mut HashMap<(usize, usize), u64> ,pos: usize, row_idx: usize) -> u64 {
    println!("Row: {}, pos {}", row_idx, pos);
    if row_idx >= rows.len() {
        return 1;
    }
    let row = rows[row_idx];
    match row.chars().nth(pos).unwrap() {
        '^' => {
            match cache.get(&(pos, row_idx)) {
                Some(&cached) => {
                    println!("Cache hit for ({}, {}) = {}", pos, row_idx, cached);
                    return cached;
                }
                None => { }
            }
            let mut total = 0;
            if pos > 0 {
                total += step(rows, cache, pos - 1, row_idx + 1);
            }
            if pos < row.len() - 1 {
                total += step(rows, cache, pos + 1, row_idx + 1);
            }
            cache.insert((pos, row_idx), total);
            total
        }
        _ => step(rows, cache, pos, row_idx + 1),
    }
}

fn solve_part2(input: &str) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    let header = rows[0];
    let rows: Vec<&str> = rows[1..].to_vec();
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    step(&rows, &mut cache, header.find('S').unwrap(), 0)
}

fn main() {
    let day = 7;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 21);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 40);
    }
}
