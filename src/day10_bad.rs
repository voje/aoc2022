pub mod test_data;
pub mod instr;

use instr::{Instr,Addx,Noop};
use std::fmt;

pub struct Cpu {
    clock: i32,
    reg_1: i32,
    instructions: Vec<Box<dyn Instr>>,
    ins_pointer: usize,

    // Signal strength sum, part1
    ss_sum: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            clock: 0,
            reg_1: 1,
            instructions: vec![],
            ins_pointer: 0,
            ss_sum: 0,
        }
    }

	fn add_instr(&mut self, ins: Box<dyn Instr>) {
		self.instructions.push(ins);
	}

    fn run(&mut self) {
        self.clock += 1;

        let ins = self.instructions.get_mut(self.ins_pointer).unwrap();
		ins.decrease_cycles();
		if ins.get_cycles() == 0 {
			ins.exec(&mut self);
			self.ins_pointer += 1;
		}	

        if self.clock % 20 == 0 {
            self.ss_sum += self.clock * self.reg_1;
            println!("{}", self)
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpu c: {}", self.clock)
    }
}

fn read_instructions(data: &str) {
    let mut cpu = Cpu::new();
    for line in data.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        match spl[0] {
            "noop" => cpu.add_instr(Box::new(Noop::new())),
            "addx" => cpu.add_instr(Box::new(Addx::new(spl[1].parse::<i32>().unwrap()))),
            &_ => (),
        };
    }
}

#[test]
fn day10_part1() {
    read_instructions(test_data::DATA1);
}

