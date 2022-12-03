use std::fs;
use aoc2022::day03::{part1, part2};

const FILE_PATH: &str = "./inputs/day03_1.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).unwrap();
    let res = part1(&contents);
    let res2 = part2(&contents);
    println!("Part1: {}, part2: {}", res, res2);
}
