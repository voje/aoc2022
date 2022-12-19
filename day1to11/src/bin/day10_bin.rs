use std::fs;
use day1to11::day10::{part1, part2};

const INPUT :&str = "./inputs/day10_1.txt";

fn main() {
    let data = fs::read_to_string(INPUT)
        .expect("Failed reading input file");
    let res1 = part1(&data);
    let res2 = part2(&data);
    println!("Signal strengths: {}", res1);
    println!("CRT:\n{}", res2);

/*
 * Signal strengths: 14040
 * CRT:
 * 
 * ####..##...##....##.####...##.####.#....
 * ...#.#..#.#..#....#....#....#.#....#....
 * ..#..#....#.......#...#.....#.###..#....
 * .#...#.##.#.......#..#......#.#....#....
 * #....#..#.#..#.#..#.#....#..#.#....#....
 * ####..###..##...##..####..##..#....####.
 */
}
