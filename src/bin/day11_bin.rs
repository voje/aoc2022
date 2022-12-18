use std::fs;
use aoc2022::day11::part1;

const INPUT :&str = "./inputs/day11_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    // let res2 = part2(&data);
    println!("Level of monkey business after 20 rounds: {}", res1);
    // Level of monkey business after 20 rounds: 99840
    //
    // TODO For part2, lookup modulo arithmetic (math!)
}
