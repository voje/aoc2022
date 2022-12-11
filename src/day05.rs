use std::fmt::Display;
use regex::Regex;

#[derive(Debug)]
struct Move {
    num: i64,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Yard {
    stacks: Vec<Vec<char>>,
}

impl Display for Yard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        for (i, stack) in self.stacks.iter().enumerate() {
            let str_i = format!("{}", i + 1);
            out.push_str(&str_i);
            for c in stack {
                out.push_str(&format!(" {}", c).to_owned()); 
            }
            out.push_str("\n");
        }
        write!(f, "{}", out)
    }
}

impl Yard {
    fn new() -> Yard {
        let s: Vec<Vec<char>> = Vec::with_capacity(5 * 5);
        Yard {
            stacks: s,
        }
    }

    // XXABC -> XXCBA
    fn apply_move(mut self, m: &Move) -> Self {
        for _ in 0..m.num {
            let c = self.stacks[m.from - 1].pop();
            self.stacks[m.to - 1].push(c.unwrap());
        }
        self
    }

    // XXABC -> XXABC
    fn apply_sticky_move(mut self, m: &Move) -> Self {
        let mut acc: Vec<char> = vec![];
        for _ in 0..m.num {
            let c = self.stacks[m.from - 1].pop();
            acc.push(c.unwrap());
        }
        for c in acc.iter().rev() {
            self.stacks[m.to - 1].push(*c);
        }
        self
    }

    fn get_top_crates(self) -> String {
        let mut tops: Vec<char> = vec![];
        for s in self.stacks{
            let c = s.last().copied().unwrap();
            tops.push(c);
        }
        tops.iter().collect()
    }
}

fn read_input(input: &str) -> (Yard, Vec<Move>) {
    let re_stak = Regex::new(r"[\[\]]").unwrap();
    let re_nums = Regex::new(r"^[' '(\d)]*$").unwrap();
    let re_move = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut stak_lines: Vec<String> = vec![];
    let mut nums_line = String::new();

    let mut yard = Yard::new();
    let mut moves: Vec<Move> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        else if re_stak.is_match(line) {
            stak_lines.push(line.to_string());
        } else if re_nums.is_match(line) {
            nums_line = line.to_string();
        } else if re_move.is_match(line) {
            let cap = re_move.captures(line).unwrap();
            moves.push(
                Move {
                    // TODO: fix this code
                    num: cap.get(1).map_or(999, |m| m.as_str().parse().unwrap()),
                    from: cap.get(2).map_or(999, |m| m.as_str().parse().unwrap()),
                    to: cap.get(3).map_or(999, |m| m.as_str().parse().unwrap()),
                }
            );
        }
    }
    
    // TODO: dude this parsing is horrible...
    // It works though...
    for (i, c) in nums_line.chars().enumerate() {
        if c.is_numeric() {
            let mut stack: Vec<char> = vec![];
            for sl in stak_lines.iter().rev() {
                let chrs: Vec<char> = sl.chars().collect();
                if chrs[i].is_uppercase() {
                    stack.push(chrs[i]);
                }
            }
            yard.stacks.push(stack);
        }
    }

    (yard, moves)
}

pub fn part1(input: &str) -> String {
    let (mut yard, moves) = read_input(input);
    for m in moves {
        // println!("{:?}\n{}---", m, yard);
        yard = yard.apply_move(&m);
    }
    // println!("END:\n{}---", yard);
    let s: String = yard.get_top_crates();
    s
}

pub fn part2(input: &str) -> String {
    let (mut yard, moves) = read_input(input);
    for m in moves {
        // println!("{:?}\n{}---", m, yard);
        yard = yard.apply_sticky_move(&m);
    }
    // println!("END:\n{}---", yard);
    let s: String = yard.get_top_crates();
    s
}

#[cfg(test)]
const DATA1: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

/*
#[test]
fn day05_yard() {
    let (yard, _) = read_input(DATA1);
    println!("{}", yard);
}
*/

#[test]
fn day05_read_input() {
    let (stacks, moves) = read_input(DATA1);

    /*
    println!("{:?}", stacks);
    println!("{:?}", moves);
    */

    assert_eq!(stacks.stacks.len(), 3);
    assert_eq!(moves.len(), 4);
}

#[test]
fn day05_part1() {
    assert_eq!(part1(DATA1), "CMZ");
}

#[test]
fn day05_part2() {
    assert_eq!(part2(DATA1), "MCD");
}
