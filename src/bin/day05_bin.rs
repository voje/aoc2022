use std::fs;
use aoc2022::day05::{part1, part2};

const INPUT :&str = "./inputs/day05_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    let res2 = part2(&data);
    println!("Top crates part1: {}, top crates part2: {}", res1, res2);
    // Top crates part1: VCTFTJQCG, top crates part2: GCFGLDNJZ
}
