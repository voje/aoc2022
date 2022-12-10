use std::ops::{Sub,Add};
use std::{time,thread};
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up(Coord),
    Down(Coord),
    Left(Coord),
    Right(Coord),
}

impl Direction {
    fn new(s: &str) -> Direction {
        match s {
            "U" => Direction::Up(Coord{ x: 0, y: -1 }),
            "D" => Direction::Down(Coord{ x: 0, y: 1 }),
            "L" => Direction::Left(Coord{ x: -1, y: 0 }),
            "R" => Direction::Right(Coord{ x: 1, y: 0 }),
            &_ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    } 
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    } 
}

fn sign(i: i32) -> i32 {
    if i == 0 { return 0 }
    if i > 0 { return 1 }
    -1
}

impl Coord {
    fn unit(&self) -> Coord {
        Coord {
            x: sign(self.x),
            y: sign(self.y),
        }
    }
}


#[derive(Debug)]
struct Rope {
    head: Coord,
    tail: Vec<Coord>,
    tail_visited: HashSet<String>,
}

fn draw_grid(r: &Rope) {
    let mut s = String::new();
    for y in-5..20 {
        for x in -5..20 {
            let c = Coord {x: x, y: y};

            let mut it_was_tail = false;
            for (i, t) in r.tail.iter().enumerate() {
                if c == *t {
                    s.push_str(&format!("{} ", i + 1));
                    it_was_tail = true;
                    break;
                }
            }

            if !it_was_tail {
                if c == r.head {
                    s.push_str("H "); 
                } else {
                    s.push_str(". ");
                }
            }
        }
        s.push_str("\n");
    }
    println!("{}", s);
    // Clear ternimal
    print!("{}[2J", 27 as char);
    thread::sleep(time::Duration::from_secs(1));
}

impl Rope {
    fn new(tail_len: usize) -> Rope {
        Rope {
            head: Coord{x: 0, y: 0},
            tail: vec![Coord {x: 0, y: 0}; tail_len],
            tail_visited: HashSet::new(),
        }
    }

    fn move_head_one_step(&mut self, d: &Direction) {
        let step: Coord = match d {
            Direction::Up(c) => *c,
            Direction::Down(c) => *c,
            Direction::Left(c) => *c,
            Direction::Right(c) => *c,
        };
        self.head = self.head + step;
        draw_grid(&self);
    }

    fn move_head(&mut self, d: Direction, distance: i32) {
        for step in 0..distance {
            self.move_head_one_step(&d);
            self.follow_tail_one_step();
            // draw_grid(&self);
        }
    }
    
    fn follow_tail_one_step(&mut self) {
        // Cann't figure out how to borrow one vector element as immutable 
        // and the other vector el. as mutable at the same time...
        // Let's just build a new vector and swap
        let mut new_tail: Vec<Coord> = Vec::new();

        let mut h = &self.head; 
        for i in 0..self.tail.len() {
            if i > 0 {
                h = self.tail.get(i-1).unwrap();
            }
            let t = self.tail.get(i).unwrap();
            let th = *h - *t;
            let mut new_knot = t.clone();
            if th.x.abs() >= 2 || th.y.abs() >= 2 {
                new_knot = *t + th.unit();
            }
            new_tail.push(new_knot);
        }
        self.tail = new_tail;
        self.record_tail_coord()
    }

    fn record_tail_coord(&mut self) {
        let tail_last = self.tail.last().unwrap();
        let tail_last_str = format!("{}{}", tail_last.x, tail_last.y);
        self.tail_visited.insert(tail_last_str);
    }
}

fn parse_line(line: &str) -> (i32, Direction) {
    let spl: Vec<&str> = line.split(" ").collect(); 
    let distance = spl[1].parse::<i32>().unwrap();
    let direction = Direction::new(spl[0]);
    (distance, direction)
}

pub fn part1(data: &str) -> i32 {
    let mut r = Rope::new(1);
    for line in data.lines() {
        let (distance, direction) = parse_line(line);
        r.move_head(direction, distance); 
    }
    r.tail_visited.len().try_into().unwrap()
}

pub fn part2(data: &str) -> i32 {
    let mut r = Rope::new(9);
    draw_grid(&r);
    for line in data.lines() {
        let (distance, direction) = parse_line(line);
        r.move_head(direction, distance); 
    }
    r.tail_visited.len().try_into().unwrap()
}

#[cfg(test)]
const DATA: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const DATA2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn day09_coord() {
    let a = Coord{ x: 5, y: 6 };
    let b = Coord{ x: 2, y: 4 }; 
    let c = a + b;
    let d = b - a;

    assert_eq!(c, Coord{ x: 7, y: 10 });
    assert_eq!(d, Coord{ x: -3, y: -2 });
}

#[test]
fn day09_part1() {
    assert_eq!(part1(&DATA), 13);
}

#[test]
fn day09_part2() {
    assert_eq!(part2(&DATA), 36);
}

