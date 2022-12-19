use std::fs;
use day1to11::day06::{part1, part2};

const INPUT :&str = "./inputs/day06_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
	let res2 = part2(&data);
    println!("End of signal marker (4): {}, end of message marker (14): {}",
		res1, res2);

}
