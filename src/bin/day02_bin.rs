use std::fs;
use aoc2022::day02::tournament;

const FILE_PATH: &str = "./inputs/day02_1.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    println!("Day02: rock, paper, scissors: {}", tournament(&contents));
}
