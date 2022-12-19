use std::fs;
use day1to11::day04::{part1, part2};

const INPUT :&str = "./inputs/day04_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    println!("Pairs where one dwarf's task list contains another's: {}", res1);

    let res2 = part2(&data);
    println!("Dwarf pairs with overlapping tasks: {}", res2);

    // Pairs where one dwarf's task list contains another's: 503
    // Dwarf pairs with overlapping tasks: 827
}
