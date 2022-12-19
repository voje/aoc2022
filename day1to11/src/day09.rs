pub mod grid;

use grid::{Grid, Point};
use std::collections::HashSet;

struct Rope {
    knots: Vec<Point>,

    // Points visited by the tail knot
    visited_points: HashSet<String>,
}

impl Rope {
    fn new(len: usize) -> Result<Rope, String> {
        if len == 0 {
            return Err("Length of Rope should be 1 or greater".to_owned());
        }
        let mut r = Rope {
            knots: Vec::new(),
            visited_points: HashSet::new(),
        };
        for i in 0..len {
            r.knots.push(Point::new(0, 0, Some(i.try_into().unwrap())));
        }
        Ok(r)
    }
    
    // Move head of the rope, other knots should follow
    // according to 'rope physics' rules
    fn move_one_step(&mut self, direction: &Point) {
        let head = self.knots.get_mut(0).unwrap();
        head.move_point(direction);

        // Move tail knots
        let mut last_modified = head.clone();
        for k in self.knots[1..].iter_mut() {
            let mvmt = Self::knot_follow_vec(k, &last_modified);
            // println!("{:?}", mvmt);
            k.move_point(&mvmt);
            last_modified = k.clone();
        }
        // Record position of last tail knot
        let ltk = self.knots.last().unwrap();
        self.visited_points.insert(format!("{}-{}", ltk.x, ltk.y));
    }

    // Returns a vector in which the knot should follow 
    // to join anchor
    // To be used with: knot.move_point(dir)
    fn knot_follow_vec(knot: &Point, anchor: &Point) -> Point {
        let vec = knot.vec_to(anchor);
        if vec.x.abs() >= 2 || vec.y.abs() >= 2 {
            return vec.unit(); 
        }
        Point::new(0, 0, None)
    }
}

// Parse input data, move rope, calculate output score 
// (points visited by last tail knot)
fn simulate_rope(grid: Option<&Grid>, mut rope: Rope, data: &str) -> i32 {
    if let Some(g) = grid {
        println!("START:\n{}", g.draw(&rope.knots));
    }
    for line in data.lines() {
        let (distance, direction) = parse_line(line);
        for _ in 0..distance {
            rope.move_one_step(&direction); 
            if let Some(g) = grid {
                println!("{}", g.draw(&rope.knots));
            }
        }
    }
    rope.visited_points.len().try_into().unwrap()
}

fn parse_line(line: &str) -> (i32, Point) {
    let spl: Vec<&str> = line.split(" ").collect(); 
    let distance = spl[1].parse::<i32>().unwrap();
    let direction = match spl[0] {
        "U" => Point::new(0,-1, None),
        "D" => Point::new(0,1, None),
        "L" => Point::new(-1,0, None),
        "R" => Point::new(1,0, None),
        &_ => todo!(),
    };
    (distance, direction)
}

pub fn part1(data: &str) -> i32 {
    let r = Rope::new(2).unwrap();
    simulate_rope(None, r, data)
}

pub fn part2(data: &str) -> i32 {
    let r = Rope::new(10).unwrap();
    simulate_rope(None, r, data)
}

/*
#[cfg(test)]
const DATA0: &str = "R 3
U 3";

#[test]
fn day09_test_rope_1() {
    let r = Rope::new(3).unwrap();
    let g = Grid::new(5);
    simulate_rope(Some(&g), r, DATA0);
}
*/

#[cfg(test)]
const DATA1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn day09_part1() {
    assert_eq!(part1(DATA1), 13);
}

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
fn day09_part2() {
    assert_eq!(part2(DATA2), 36);
}

