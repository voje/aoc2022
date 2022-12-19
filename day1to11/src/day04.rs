use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct ReadRangeError;
impl FromStr for Range {
    type Err = ReadRangeError;
    fn from_str(s: &str) -> Result<Self, ReadRangeError> {
        let halves: Vec<&str> = s.split("-").collect();
        if halves.len() != 2 {
            return Err(ReadRangeError)
        }
        Ok(Range{
            start: halves[0].parse::<i64>().unwrap(),
            end: halves[1].parse::<i64>().unwrap(),
        })
    }
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        self.start <= r.start && self.end >= r.end 
    }
    fn contains_point(&self, p: i64) -> bool {
        p >= self.start && p <= self.end
    }
    fn overlaps_with(&self, r: &Range) -> bool {
        self.contains_point(r.start) ||
            self.contains_point(r.end) ||
            r.contains(self)
    }
}

pub fn part1(data: &str) -> i64 {
    let mut count = 0;
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        let dwarves: Vec<&str> = line.split(",").collect();
        if dwarves.len() != 2 {
            panic!("Weird input line: {}", line); 
        }
        let d0 = Range::from_str(dwarves[0]).unwrap();
        let d1 = Range::from_str(dwarves[1]).unwrap();

        // println!("d0: {:?}, d1. {:?}", d0, d1);
        if d0.contains(&d1) || d1.contains(&d0) {
            count += 1;
        }
    }
    count
}

pub fn part2(data: &str) -> i64 {
    let mut count = 0;
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        let dwarves: Vec<&str> = line.split(",").collect();
        if dwarves.len() != 2 {
            panic!("Weird input line: {}", line); 
        }
        let d0 = Range::from_str(dwarves[0]).unwrap();
        let d1 = Range::from_str(dwarves[1]).unwrap();

        // println!("d0: {:?}, d1. {:?}", d0, d1);
        if d0.overlaps_with(&d1) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9

2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn day04_1() {
    assert_eq!(part1(DATA), 2);
}

#[test]
fn day04_2() {
    assert_eq!(part2(DATA), 4);
}
