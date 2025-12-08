mod utils;

use utils::Point;

fn solve_part1(input: &str) -> u64 {
    let rows: Vec<&str> = input.lines().collect();
    let points: Vec<Point> = rows
        .iter()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();

    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for i in 0..10 {
        println!("Iteration {}", i);
        let mut min_points: (usize, usize, i32) = (0, 0, i32::MAX);
        points.iter().enumerate().for_each(|(i, p1)| {
            points.iter().enumerate().for_each(|(j, p2)| {
                if i != j
                    && circuits
                        .iter()
                        .all(|circuit| !(circuit.contains(&i) && circuit.contains(&j)))
                {
                    let dist = p1.distance(p2);
                    println!(
                        "Comparing points {} and {} dist {} vs {}",
                        i, j, dist, min_points.2
                    );
                    if dist < min_points.2 as i32 {
                        println!(
                            "New min points found: {} and {} with distance {}",
                            i, j, dist
                        );
                        min_points = (i, j, dist);
                    }
                }
            });
        });
        println!(
            "Min points: {:?} with distance {}",
            (min_points.0, min_points.1),
            min_points.2
        );

        let circuit_exists = circuits
            .iter()
            .any(|circuit| circuit.contains(&min_points.0) || circuit.contains(&min_points.1));

        let both_in_same_circuit = circuits
            .iter()
            .any(|circuit| circuit.contains(&min_points.0) && circuit.contains(&min_points.1));

        let requires_merge = circuit_exists && !both_in_same_circuit;

        if !circuit_exists {
            circuits.push(vec![min_points.0, min_points.1]);
            println!(
                "Creating new circuit with points: {:?} ",
                (min_points.0, min_points.1)
            );
        }

        if circuits
            .iter()
            .any(|circuit| circuit.contains(&min_points.0) || circuit.contains(&min_points.1))
        {
            circuits.iter_mut().for_each(|circuit| {
                if circuit.contains(&min_points.0) && !circuit.contains(&min_points.1) {
                    circuit.push(min_points.1);
                } else if circuit.contains(&min_points.1) && !circuit.contains(&min_points.0) {
                    circuit.push(min_points.0);
                } else if circuit.contains(&min_points.0) && circuit.contains(&min_points.1) {
                    // both points are already in the circuit
                }
            });
        }
    }
    println!("Circuits: {:?}", circuits);

    let a: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    a[..3].iter().map(|v| *v as u64).product()
}

fn solve_part2(input: &str) -> u64 {
    0
}

fn main() {
    let day = 8;
    let input_text: String = utils::load_input(day);
    let part1 = solve_part1(&input_text);
    println!("Part 1: The result is {}", part1);
    let part2 = solve_part2(&input_text);
    println!("Part 2: The result is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 40);
    }
    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 0);
    }
}
