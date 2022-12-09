use std::fs;
use aoc2022::day08::{part1,part2};

const INPUT :&str = "./inputs/day08_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    let res2 = part2(&data);
    println!("Visible trees: {}, top scenic score: {}", res1, res2);
    // Visible trees: 1803, top scenic score: 268912
}
