use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u128>,
    pub oper: Vec<String>,
    pub test_div: u128,
    pub test_true: usize,
    pub test_false: usize,
    pub n_inspections: u128,
    pub relief: bool
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = String::new();
        for item in &self.items {
            items.push_str(&format!("{} ", item).to_owned())
        }
        write!(f, "{}", items)  
    }
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey { 
            items: vec![],
            oper: vec![],
            test_div: 0,
            test_true: 0,
            test_false: 0,
            n_inspections: 0,
            relief: false,
        }
    }

    // Monkey empties its inventory and decides where to throw each item
    pub fn get_transactions(&mut self) -> Vec<(u128, usize)> {
        let mut out: Vec<(u128, usize)> = vec![];
        for _ in 0..self.items.len() {
            let mut item = self.items.pop().unwrap();
            item = self.inspect_item(item); 
            out.push((
                item,
                self.decide_on_throw(item),
            ));
        }
        assert!(self.items.len() == 0);
        out
    }

    fn decide_on_throw(&self, item: u128) -> usize {
        if (item % self.test_div) == 0 {
            return self.test_true;
        }
        self.test_false
    }

    fn _parse_num(old: u128, num: &str) -> u128 {
        match num {
            "old" => old,
            n => n.parse::<u128>().unwrap()
        }
    }

    fn inspect_item(&mut self, item: u128) -> u128 {
        self.n_inspections += 1;
        let mut new_item = self.calc_oper(item).unwrap();
        // Relief that the item isn't broken
        if self.relief {
            new_item = new_item / 3;
        } 
        new_item
    }

    fn calc_oper(&self, old: u128) -> Result<u128, Box<dyn Error>> {
        let a = Self::_parse_num(old, &self.oper[0].to_string());
        let op = &self.oper[1];
        let b = Self::_parse_num(old, &self.oper[2].to_string());

        Ok(match op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            &_ => todo!(),
        })
    }
}
