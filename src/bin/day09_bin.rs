use std::fs;
use aoc2022::day09::{part1};

const INPUT :&str = "./inputs/day09_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    // let res2 = part2(&data);
    println!("Unique tail coordinates: {}", res1);
}
