mod utils;

fn separate_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].trim().parse::<u64>().ok()?;
                let end = parts[1].trim().parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

fn is_invalid(num: &u64) -> bool {
    let num_str = num.to_string();
    let l = num_str.len();
    let first_half = &num_str[..l / 2];
    let second_half = &num_str[l / 2..];
    if first_half == second_half {
        return true;
    }
    false
}

fn is_invalid_part2(num: &u64) -> bool {
    let num_str = num.to_string();
    let l = num_str.len() as u32;
    let windows: Vec<u32> = (1..=l / 2).collect();
    let mut window_repeats = windows.iter().map(|window_size| {
        if l % window_size != 0 {
            return false;
        }

        let mut repeats = true;
        let pattern = &num_str[0..*window_size as usize];
        for i in (0..l).step_by(*window_size as usize) {
            let end = (i + *window_size) as usize;
            let segment = &num_str[i as usize..end];
            if segment != pattern {
                repeats = false;
                break;
            }
        }
        repeats
    });
    window_repeats.any(|x| x)
}

fn solve_part1(input: &str) -> u64 {
    let ranges = separate_ranges(input);
    ranges.iter().map(|(start, end)| {
        // Create a range from start to end (inclusive)
        println!("Range: {}-{}", start, end);
        let mut part_sum = 0;
        for num in *start..=*end {
            if is_invalid(&num) {
                println!("  Invalid number: {}", num);
                part_sum += num;
            }
        }
        part_sum
    }).sum()
}

fn solve_part2(input: &str) -> u64 {
    let ranges = separate_ranges(input);
    ranges.iter().map(|(start, end)| {
        // Create a range from start to end (inclusive)
        println!("Range: {}-{}", start, end);
        let mut part_sum = 0;
        for num in *start..=*end {
            if is_invalid_part2(&num) {
                println!("  Invalid number: {}", num);
                part_sum += num;
            }
        }
        part_sum
    }).sum()
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

    static TEST_INPUT: &'static str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_separate_ranges() {
        let result = separate_ranges("10-20,30-40,50-60");
        assert_eq!(result, vec![(10, 20), (30, 40), (50, 60)]);
    }

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 1227775554);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 4174379265);
    }
}
