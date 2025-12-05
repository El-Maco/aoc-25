mod utils;

fn is_accessible_roll(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    if grid[y][x] != '@' {
        return false;
    }
    let mut adjacent_roll_count = 0;
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for (dx, dy) in directions.iter() {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
            if grid[new_y as usize][new_x as usize] == '@' {
                adjacent_roll_count += 1;
            }
        }
    }
    adjacent_roll_count < 4
}

fn print_accessible_rolls(grid: &Vec<Vec<char>>, accessible_rolls: &Vec<(usize, usize)>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if accessible_rolls.contains(&(x, y)) {
                print!("x");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}

fn solve_part1(input: &str) -> u32 {
    let rows = utils::parse_input(input);
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let mut accessible_rolls: Vec<(usize, usize)> = vec![];
    let mut accessible_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_accessible_roll(&grid, x, y) {
                accessible_count += 1;
                accessible_rolls.push((x, y));
            }
        }
    }
    print_accessible_rolls(&grid, &accessible_rolls);
    accessible_count
}

fn solve_part2(input: &str) -> u32 {
    let rows = utils::parse_input(input);
    let mut grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let mut accessible_rolls: Vec<(usize, usize)> = vec![];
    let mut removed_count = 0;
    let mut i = 0;
    loop {
        println!("Iteration {}, removed so far: {}", i, removed_count);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if is_accessible_roll(&grid, x, y) {
                    accessible_rolls.push((x, y));
                }
            }
        }
        if accessible_rolls.is_empty() {
            break;
        }
        accessible_rolls.iter().for_each(|(x, y)| {
            grid[*y][*x] = '.';
            removed_count += 1;
        });
        accessible_rolls.clear();
    }
    removed_count
}

fn main() {
    let input_text: String = utils::load_input(4);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 43);
    }
}
