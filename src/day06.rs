use std::iter::Iterator;
use std::collections::HashMap;

struct SlidingWindow {
    s: String,
    i: usize,
    ws: usize,
}

impl SlidingWindow {
    fn new(s: &str, window_size: usize) -> SlidingWindow {
        SlidingWindow {
            s: s.to_string(),
            i: 0,
            ws: window_size,
        }
    }
}

impl Iterator for SlidingWindow {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.i + self.ws > self.s.len() {
            return None
        }
        let sub = Some(self.s[(self.i)..(self.i + self.ws)].to_owned());
        self.i += 1;
        sub
    }

}

fn has_unique_chars(s: &str) -> bool {
    let mut m: HashMap<char, bool> = HashMap::new();
    for c in s.chars() {
        if m.contains_key(&c) {
            return false
        }
        m.insert(c, true);
    }
    true
}

fn end_of_marker(stream: &str, marker_len: usize) -> Option<usize> {
    let sw = SlidingWindow::new(stream, marker_len); 

    let mut i: usize = 0;
    for w in sw {
        if has_unique_chars(&w) {
            // println!("{}", w);
            return Some(i + marker_len);
        }
        i += 1; 
    }
    None    
}

pub fn part1(s: &str) -> i64 {
    end_of_marker(s, 4).unwrap().try_into().unwrap()
}

pub fn part2(s: &str) -> i64 {
    end_of_marker(s, 14).unwrap().try_into().unwrap()
}

#[test]
fn day06_part2() {
    let test_cases = HashMap::<&str, i64>::from([
        ("aabcdefghijklmn", 15),
		("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
		("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
		("nppdvjthqldpwncqszvftbrmjlhg", 23),
		("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
		("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ]);
    for (k,v) in test_cases {
        assert_eq!(part2(k), v);
    }
}

#[test]
fn day06_part1() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

/*
#[test]
fn day06_sliding_window() {
    let data = "abcdef";
    let w = SlidingWindow::new(data, 3); 
    for sub in w {
        println!("{}", sub);
    }
}
*/

