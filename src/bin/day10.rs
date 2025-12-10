mod utils;
use std::collections::{HashSet, VecDeque};

fn press_button(lights: &mut Vec<bool>, button: &Vec<u32>) {
    for &index in button.iter() {
        let idx = index as usize;
        if idx < lights.len() {
            lights[idx] = !lights[idx];
        }
        else
        {
            panic!("Button index {} out of bounds for lights of length {}", idx, lights.len());
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    rows.iter()
        .map(|row| {
            let mut iter = row.split(']').into_iter();
            let desired_lights: &str = iter.next().unwrap().trim_start_matches('[');
            println!("Lights: {}", desired_lights);
            let rest: Vec<&str> = iter.next().unwrap().trim().split(' ').collect();
            let buttons: Vec<Vec<u32>> = rest[..rest.len() - 1]
                .iter()
                .map(|button_str| {
                    button_str
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|num_str| num_str.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();
            println!("Buttons: {:?}", buttons);

            let lights: Vec<bool> = vec![false; desired_lights.len()];

            // BFS to find the minimum button presses to reach the desired state
            // light state
            // button press sequence

            let mut queue: VecDeque<(Vec<bool>, u32)> = VecDeque::new();
            let mut visited: HashSet<Vec<bool>> = HashSet::new();

            queue.push_back((lights.clone(), 0));
            visited.insert(lights.clone());
            while let Some((current_lights, presses)) = queue.pop_front() {
                // Check if current lights match desired state
                let current_state: String = current_lights
                    .iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect();
                if current_state == desired_lights {
                    println!("Reached desired state with {} presses", presses);
                    return presses as u64;
                }

                // Try pressing each button
                for button in &buttons {
                    let mut new_lights = current_lights.clone();
                    press_button(&mut new_lights, button);
                    if !visited.contains(&new_lights) {
                        visited.insert(new_lights.clone());
                        queue.push_back((new_lights, presses + 1));
                    }
                }
            }
            println!("Could not reach desired state");

            0
        })
        .sum::<u64>()
}

fn solve_part2(input: &str) -> u64 {
    0
}

fn main() {
    let day = 10;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 7);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 0);
    }
}
