use std::ops::{Sub,Add};
// use std::{time,thread};
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
    }

    fn move_head(&mut self, d: Direction, distance: i32, displ_grid: bool) {
        if displ_grid {
            println!("move_head {:?}, {}", d, distance);
            println!("Starting grid:");
            draw_grid(&self);
        }
        for step in 0..distance {
            self.move_head_one_step(&d);
            if displ_grid {
                println!("Move head step: {}", step);
                draw_grid(&self);
            }
            self.follow_tail_one_step();
            if displ_grid {
                println!("Move tail step: {}", step);
                draw_grid(&self);
            }
        }
        if displ_grid {
            println!("Ending grid:");
            draw_grid(&self);
        }
    }
    
    fn follow_tail_one_step(&mut self) {
        // Cann't figure out how to borrow one vector element as immutable 
        // and the other vector el. as mutable at the same time...
        // Let's just build a new vector and swap
        let mut new_tail: Vec<Coord> = Vec::new();

        let mut h = self.head.clone(); 
        // println!("Tail before: {:?}", self.tail);
        for i in 0..self.tail.len() {
            if new_tail.len() > 0 {
                h = new_tail.last().unwrap().clone();
            }
            let t = self.tail.get(i).unwrap();
            // println!("Knot pair: {:?} - {:?}", h, t);
            let th = h - *t;
            let mut new_knot = t.clone();
            if th.x.abs() >= 2 || th.y.abs() >= 2 {
                new_knot = *t + th.unit();
            }
            new_tail.push(new_knot);
        }
        self.tail = new_tail;
        // println!("Tail after: {:?}", self.tail);
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
    simulate_rope(data, 1, false)
}

pub fn part2(data: &str) -> i32 {
    simulate_rope(data, 9, false)
}

pub fn simulate_rope(data: &str, rope_len: usize, displ_grid: bool) -> i32 {
    let mut r = Rope::new(rope_len);
    for line in data.lines() {
        let (distance, direction) = parse_line(line);
        r.move_head(direction, distance, displ_grid); 
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

#[cfg(test)]
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
    assert_eq!(part2(&DATA2), 36);
}

#[test]
fn day09_test_data3() {
    // The tail doesn't move in this case, the rope just stretches.
    const DATA3: &str = "R 9";
    assert_eq!(part2(&DATA3), 1);
}

#[test]
fn day09_test_data4() {
    // The tail doesn't move in this case, the rope just stretches.
    const DATA3: &str = "R 4\nU 5";
    assert_eq!(simulate_rope(&DATA3, 3, true), 1);
}

