pub mod test_data;
pub mod instr;

use instr::{Visitor, Instr};

#[derive(Debug)]
pub struct Cpu {
    clock: i32,
    reg_1: i32,
    ins_pointer: usize,

    // -1 means uninitiated
    // whenever it's 0, it means it's
    // finished executing an instruction
    // and should move to the next one
    ins_cycle_counter: Option<u32>,

    // Signal strength sum, part1
    ss_sum: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            clock: 0,
            reg_1: 1,
            ins_pointer: 0,
            ins_cycle_counter: None,
            ss_sum: 0,
        }
    }

    fn run(&mut self, instr: &Vec<Instr>) {
        while self.ins_pointer < instr.len() {
            let ins = instr.get(self.ins_pointer).unwrap(); 
            // TODO  
        }
    }

    fn handle_ins_cycle_counter(&mut self, c: u32) {
        match self.ins_cycle_counter {
            Some(x) => self.ins_cycle_counter = Some(x - 1),
            None => self.ins_cycle_counter = Some(c),
        }
    }

    fn exec_noop(&mut self, c: u32) {
        self.handle_ins_cycle_counter(c);
    }

    fn exec_addx(&mut self, c: u32, x: i32) {
        self.handle_ins_cycle_counter(c);
        if self.ins_cycle_counter == Some(0) {
            self.reg_1 += x; 
        }
    }

    fn update_ss_sum(&mut self) {
        if self.clock % 20 == 0 {
            self.ss_sum += self.reg_1 * self.clock;
        }
    }
}

impl Visitor<()> for Cpu {
    fn visit_instr(&mut self, ins: &Instr) {
        self.clock += 1;
        match ins {
            Instr::Noop(c) => self.exec_noop(*c),
            Instr::Addx(c, x) => self.exec_addx(*c, *x),
        }
        self.update_ss_sum();
    }
}


fn read_instructions(data: &str) {
    let mut cpu = Cpu::new();
    let mut instr: Vec<Instr> = Vec::new();
    for line in data.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        match spl[0] {
            "noop" => instr.push(Instr::Noop(1)),
            "addx" => instr.push(Instr::Addx(2, spl[1].parse().unwrap())),
            &_ => (),
        };
    }

    cpu.run(instr); 
}

#[test]
fn day10_part1() {
    read_instructions(test_data::DATA1);
}

