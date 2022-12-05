use regex::Regex;

#[derive(Debug)]
struct Stack {
    s: Vec<char>,
}

#[derive(Debug)]
struct Move {
    num: i64,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Yard {
    stks: Vec<Stack>,
}

impl Yard {
    fn apply_move(&self, m: &Move) {
        for _ in 0..m.num {
            self.stks[m.to].s.push(
                self.stks[m.from].s.pop().unwrap()
            );
        }
    }

    fn get_top_crates(&self) {
        let mut tops: Vec<char> = vec![];
        for s in self.stks {
            let c = s.s.last().unwrap();
        }

    }
}

fn read_input(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let re_stak = Regex::new(r"[\[\]]").unwrap();
    let re_nums = Regex::new(r"^[' '(\d)]*$").unwrap();
    let re_move = Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
    let mut stak_lines: Vec<String> = vec![];
    let mut nums_line = String::new();

    let mut stacks: Vec<Stack> = vec![];
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
            let mut stack = Stack {
                s: vec![],
            };
            for sl in stak_lines.iter().rev() {
                let chrs: Vec<char> = sl.chars().collect();
                if chrs[i].is_uppercase() {
                    // println!("TEST {:?}", chrs[i]);
                    stack.s.push(chrs[i]);
                }
            }
            stacks.push(stack);
        }
    }

    (stacks, moves)
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

#[test]
fn day05_read_input() {
    let (stacks, moves) = read_input(DATA1);

    println!("{:?}", stacks);
    println!("{:?}", moves);

    assert_eq!(stacks.len(), 3);
    assert_eq!(moves.len(), 4);
}

