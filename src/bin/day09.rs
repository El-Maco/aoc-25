use rayon::prelude::*;
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

fn display_grid(grid: &Vec<String>) {
    if grid.len() > 50 || grid[0].len() > 50 {
        println!("Grid too large to display ({}x{})", grid.len(), grid[0].len());
        return;
    }
    for line in grid {
        println!("{}", line);
    }
}


fn solve_part2(input: &str) -> u64 {
    let rows = input
        .lines()
        .map(|r| {
            let mut split = r.split(',');
            let x: usize = split.next().unwrap().parse().unwrap();
            let y: usize = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();

    println!("Rows parsed");

    let (max_x, max_y) = rows.par_iter().fold(|| (0, 0), |(mx, my), (x, y)| {
        (mx.max(*x), my.max(*y))
    }).reduce(|| (0, 0), |(mx1, my1), (mx2, my2)| {
        (mx1.max(mx2), my1.max(my2))
    });

    let mut str_grid: Vec<String> = vec![".".repeat(max_x + 1); max_y + 1];
    println!("Grid initialized");
    for row in &rows {
        str_grid[row.1].replace_range(row.0..=row.0, "#");
    }

    println!("Red tiles set");
    rows.iter().for_each(|&(x, y)| {
        // Set green tiles right until the next red tile
        let last_red_tile_on_row = rows
            .iter()
            .filter(|&&(_, yy)| yy == y)
            .map(|&(xx, _)| xx).max()
            .unwrap();

        for xx in x+1..last_red_tile_on_row {
            if str_grid[y].as_bytes()[xx] == b'#' {
                continue;
            }
            str_grid[y].replace_range(xx..=xx, "G");
        }

        let last_red_tile_on_column = rows
            .iter()
            .filter(|&&(xx, _)| xx == x)
            .map(|&(_, yy)| yy).max()
            .unwrap();

        for yy in y+1..last_red_tile_on_column {
            if str_grid[yy].as_bytes()[x] == b'#' {
                continue;
            }
            str_grid[yy].replace_range(x..=x, "G");
        }
    });

    println!("Green tiles set");

    str_grid
        .par_iter_mut()
        .for_each(|line| {
            let first_g_or_red = line.find(|c| c == 'G' || c == '#');
            let last_g_or_red = line.rfind(|c| c == 'G' || c == '#');
            if let (Some(first), Some(last)) = (first_g_or_red, last_g_or_red) {
                for i in first..=last {
                    if line.as_bytes()[i] == b'.' {
                        line.replace_range(i..=i, "G");
                    }
                }
            }
        });

    println!("Grid done");

    display_grid(&str_grid);

    let largest_area = rows.par_iter().map(|&(first_x, first_y)| {
        rows.iter().filter_map(|&(second_x, second_y)| {
            if second_x > first_x && second_y > first_y {
                let is_in_area = str_grid[first_y..second_y]
                    .iter()
                    .map(|row| {
                        println!("row: {:?}", row);
                        &row[first_x..second_x]
                    })
                    .all(|slice| slice.chars().all(|c| c == 'G' || c == '#'));

                if !is_in_area {
                    return None;
                }

                let area = (((first_x as isize - second_x as isize).abs() + 1)
                    * ((first_y as isize - second_y as isize).abs() + 1))
                    as u64;
                Some(area)
            } else {
                None
            }
        }).max().unwrap_or(0)
    }).max().unwrap_or(0);

    largest_area
}


fn main() {
    let day = 9;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............

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
        assert_eq!(result, 24);
    }
}
