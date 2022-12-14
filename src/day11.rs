use regex::Regex;

#[derive(Debug)]
struct Monke {
    items: Vec<u32>,
}

impl Monke {
    fn new() -> Monke {
        Monke { items: vec![] }
    }
}

struct Me {
    worry: u32,
}

impl Me {
    fn new() -> Me {
        Me { worry: 0 }
    }
}

fn parse_input(data: &str) {
    for monkey_line in data.split("\n\n") {
        println!("---{}---", monkey_line); 
        let mut monkee = Monke::new();

        let r_items = Regex::new(r"Starting items: (.*)").unwrap();
        let c_items = r_items.captures(monkey_line).unwrap();
        let items: Vec<u32> = c_items[1].split(", ")
            .map(|m| m.parse::<u32>().unwrap()).collect();
        println!("{:?}", items);
        monkee.items = items;

        let r_oper = Regex::new(r"Operation: new = (\w+) (.) (\w+)").unwrap();
        let c_oper = r_oper.captures(monkey_line).unwrap();
        println!("{:?}", c_oper);
    }
}

#[test]
fn day11_part1() {
    parse_input(DATA); 
}

#[cfg(test)]
const DATA: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
