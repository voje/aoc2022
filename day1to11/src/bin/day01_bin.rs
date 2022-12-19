use std::fs;
use day1to11::day01::{aggregate_calories, top_dwarves};

const FILE_PATH: &str = "./inputs/day01_1.txt";
// const MAX_DWARF_ANSW: i32 = 69310;
// const MAX_THREE_ANSW: i32 = 206104;

fn main() {
    println!("Day 1 baby!");

    let contents = fs::read_to_string(FILE_PATH).unwrap();

	let dwarves = aggregate_calories(&contents);
	let (first, second, third) = top_dwarves(dwarves);

    println!("Calorie boii: {}", first);
    println!("Max boys: {}", first + second + third);
}
