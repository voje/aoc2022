use std::fs;
use aoc2022::day08::{part1};

const INPUT :&str = "./inputs/day08_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    println!("Visible trees: {}", res1);

}
