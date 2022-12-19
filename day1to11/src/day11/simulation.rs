use super::monkey::Monkey;

use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct Simulation {
    pub monkeys: Vec<Monkey>,
    relief: bool,
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ss = String::new();
        for (i, m) in self.monkeys.iter().enumerate() {
            ss.push_str(&format!("{}| {}\n", i, m).to_owned())
        }
        write!(f, "{}", ss)  
    }
}

impl Simulation {
    pub fn new(relief: bool) -> Simulation {
        Simulation {
            monkeys: vec![],
            relief,
        }
    }

    fn _print_monkeys(&self) {
        for m in &self.monkeys {
            println!("{:?}", m);
        }
    }

    pub fn get_insp_counts(&self) -> Vec<u128> {
        let mut insp: Vec<u128> = vec![];
        for m in &self.monkeys {
            insp.push(m.n_inspections);
        }
        insp
    }

    pub fn top2_result(&self) -> u128 {
        let mut insp = self.get_insp_counts();
        insp.sort();
        // Highest 2 n_inspections
        return insp.pop().unwrap() * insp.pop().unwrap();
    }

    pub fn run(&mut self, n_rounds: usize) {
        // self.print_monkeys();

        // println!("Start:\n{}", self);

        for _r in 0..n_rounds {
            for i in 0..self.monkeys.len() {
                let mut trans: Vec<(u128, usize)>;
                // Hope I'm doing this right.
                // First part of loop: mutably borrow first monkey, get
                // its transactions.
                { 
                    let m = self.monkeys.get_mut(i).unwrap();
                    // println!("{}", m);
                    trans = m.get_transactions();
                }
                // Second part of loop: mutably borrow monkeys to
                // "throw" them items.
                // println!("{:?}", trans);
                self.execute_transactions(&mut trans);
            }

            // println!("End of round {}:\n{}", _r + 1, self);
            println!("{}: {:?}", _r, self.get_insp_counts());

        }
        // self.print_monkeys();
    }

    // Monkey popped items into tr vector.
    // Pop again, to simulate monkey throwing items 
    // from idx 0 to last.
    fn execute_transactions(&mut self, tr: &mut Vec<(u128, usize)>) {
        for _ in 0..tr.len() {
            let (item, mnky_idx) = tr.pop().unwrap();
            self.monkeys[mnky_idx].items.push(item);
        }
        assert!(tr.len() == 0);
    }

    pub fn parse_input(&mut self, data: &str) {
        // Cleanup \r
        let data_clean = str::replace(data, "\r", "");

        for monkey_line in data_clean.split("\n\n") {
            // println!("---\n{}", monkey_line); 
            let mut monkee = Monkey::new();

            let r_items = Regex::new(r"Starting items: (.*)").unwrap();
            let c_items = r_items.captures(monkey_line).unwrap();
            // println!("{:?}\n{:?}", r_items, c_items);
            let items: Vec<u128> = c_items[1].trim().split(", ")
                .map(|m| m.parse::<u128>().unwrap()).collect();
            monkee.items = items;

            let r_oper = Regex::new(r"Operation: new = (\w+) (.) (\w+)").unwrap();
            let c_oper = r_oper.captures(monkey_line).unwrap();
            for i in 1..c_oper.len() {
                monkee.oper.push(c_oper[i].to_string());
            }

            let r_test_div = Regex::new(r"Test: divisible by (\d*)").unwrap();
            let c_test_div = r_test_div.captures(monkey_line).unwrap();
            monkee.test_div = c_test_div[1].to_string().parse::<u128>().unwrap();

            let r_test_true = Regex::new(r"If true: throw to monkey (\d*)").unwrap();
            let c_test_true = r_test_true.captures(monkey_line).unwrap();
            monkee.test_true = c_test_true[1].to_string().parse::<usize>().unwrap();

            let r_test_false = Regex::new(r"If false: throw to monkey (\d*)").unwrap();
            let c_test_false = r_test_false.captures(monkey_line).unwrap();
            monkee.test_false = c_test_false[1].to_string().parse::<usize>().unwrap();

            monkee.relief = self.relief;

            // println!("{:?}\n---", monkee); 
            self.monkeys.push(monkee);
        }
    }

}

