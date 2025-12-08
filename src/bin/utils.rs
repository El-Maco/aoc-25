use std::fs;

pub fn load_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(path).expect("Failed to read input file")
}

pub fn parse_input(input: &str) -> Vec<&str> {
    let rows: Vec<&str> = input.trim().split("\n").collect();
    rows
}

pub fn parse_line(row: &str) -> Vec<i32> {
    let elements: Vec<&str> = row.split_whitespace().collect();
    elements.iter().map(|v| v.parse().unwrap()).collect()
}

fn main() {
    todo!()
}

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}
