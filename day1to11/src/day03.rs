use std::str::FromStr;

// priority
//   Lowercase item types a through z have priorities 1 through 26.
//   Uppercase item types A through Z have priorities 27 through 52.
//   ASCII 'a' = 97, 'A' = 65
fn get_prio(c: char) -> Option<u8> {
    match c {
        'a'..='z' => Some(c as u8 - 96),
        'A'..='Z' => Some(c as u8 - 38),
        _ => None,
    }
}

// 'a', 'b', 'e' ==> 0x0000....000010011
fn str_to_bits(s: &str) -> u64 {
    let mut n: u64 = 0;
    for c in s.chars() {
        let prio = get_prio(c).unwrap();
        n |= 1 << (prio - 1);
    }
    n 
}

// 000001101 --> [4, 3, 1]
//               [d, c, a]
// Loop over bits, check which ones are 1
// Bit position equals prio
// Yes, I've overengineered this... :)
fn bits_to_prio(b: u64) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            v.push(i + 1);
        }
    }
    v
}

#[derive(Debug)]
struct Bag {
    left: String,
    right: String,
}

#[derive(Debug)]
struct WeirdBagErr;
impl FromStr for Bag {
    type Err = WeirdBagErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        Ok(Bag{
            left: s[..mid].to_string(),
            right: s[mid..].to_string(),
        })
    }
}

pub fn part1(s: &str) -> i64 {
    let mut total_score: i64 = 0;
    for line in s.lines() {
        let b = Bag::from_str(line).expect("Failed to parse bag line");
        
        let duplicated_items_bits = str_to_bits(&b.left) & str_to_bits(&b.right);
        let prio_score: u8 = bits_to_prio(duplicated_items_bits).iter().sum();
        total_score += prio_score as i64;
    }
    total_score
}

pub fn part2(s: &str) -> i64 {
	let mut total_score: i64 = 0;
	let mut three: Vec<String> = vec![];
	for line in s.lines() {
		three.push(line.to_string());
		if three.len() == 3 {
			let token_bits = str_to_bits(&three[0]) & 
				str_to_bits(&three[1]) &
				str_to_bits(&three[2]);
			let token_prio: u8 = bits_to_prio(token_bits).iter().sum();
			total_score += token_prio as i64;
			three = vec![];
		}
	}
	total_score
}

#[test]
fn day03_prio() {
    assert_eq!(get_prio('a').unwrap(), 1);
    assert_eq!(get_prio('c').unwrap(), 3);
    assert_eq!(get_prio('z').unwrap(), 26);
    assert_eq!(get_prio('A').unwrap(), 27);
    assert_eq!(get_prio('Z').unwrap(), 52);
}

#[test]
fn day03_str_to_bits() {
    assert_eq!(str_to_bits(""), 0);
    assert_eq!(str_to_bits("a"), 1);
    assert_eq!(str_to_bits("abc"), 7);
    assert_eq!(str_to_bits("abe"), 19);
}

#[test]
fn day03_bits_to_prio() {
    assert_eq!(bits_to_prio(str_to_bits("")), vec![]);
    assert_eq!(bits_to_prio(str_to_bits("a")), vec![1]);
    assert_eq!(bits_to_prio(str_to_bits("b")), vec![2]);
    assert_eq!(bits_to_prio(str_to_bits("ac")), vec![1,3]);
    assert_eq!(bits_to_prio(str_to_bits("Z")), vec![52]);
    assert_eq!(bits_to_prio(str_to_bits("aeABZ")), vec![1, 5, 27, 28, 52]);
}

#[test]
fn day03_1() {
    const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let res = part1(DATA); 
    assert_eq!(res, 157);
}

#[test]
fn day03_2() {
	const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

	assert_eq!(part2(DATA), 70);
}

