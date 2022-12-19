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

    crt_on: bool,
    crt: Vec<char>,
    crt_idx: usize,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            clock: 0,
            reg_1: 1,
            ins_pointer: 0,
            ins_cycle_counter: None,
            ss_sum: 0,

            crt_on: false,
            // 6 * 40
            crt: vec!['.'; 240],
            // middle of a sprite of len 3
            crt_idx: 0,
        }
    }

    fn tick(&mut self) {
        self.draw_pixel();
        self.clock += 1;
        // [1, (6 * 40 - 2)]
        self.crt_idx = (self.crt_idx + 1) % (6 * 40);
        self.update_ss_sum();
    }

    fn draw_pixel(&mut self) {
        // println!("reg_1: {}, crt_idx: {}", self.reg_1, self.crt_idx);
        if (self.crt_idx % 40) as i32 >= self.reg_1 - 1 &&
            (self.crt_idx % 40) as i32 <= self.reg_1 + 1 {
                self.crt[self.crt_idx] = '#';
            }
    }

    fn draw_crt(&self) -> String {
        let mut out = String::new();
        for (i ,c) in self.crt.iter().enumerate() {
            if (i % 40) == 0 {
                out.push_str("\n");
            }
            out.push_str(&format!("{}", c));
        }
        out.push_str("\n");
        out
    }

    fn run(&mut self, instr: &Vec<Instr>) {
        // println!("Init state: {:?}\n---", self);
        while self.ins_pointer < instr.len() {
            self.tick();

            let ins = instr.get(self.ins_pointer).unwrap(); 

            self.visit_instr(ins);

            /*
            println!("[{}] [{}] || {:?}", self.clock, self.ins_pointer, ins);
            println!("{:?}", self);
            println!("---");
            */
            if self.crt_on {
                println!("{}", self.draw_crt());
            }
            /*
            if self.clock >= 10 {
                break;
            }
            */
        }
    }

    // At the end of each instruction
    fn try_set_ins_cycle_counter(&mut self, c: u32) {
        match self.ins_cycle_counter {
            Some(x) => self.ins_cycle_counter = Some(x - 1),
            None => self.ins_cycle_counter = Some(c - 1),
        }
    }

    // At the start of each instruction
    fn try_reset_ins_cycle_counter(&mut self) {
        if self.ins_cycle_counter == Some(0) {
            self.ins_cycle_counter = None;
            self.ins_pointer += 1;
        }
    }

    fn exec_noop(&mut self, c: u32) {
        self.try_set_ins_cycle_counter(c);
        self.try_reset_ins_cycle_counter();
    }

    fn exec_addx(&mut self, c: u32, x: i32) {
        self.try_set_ins_cycle_counter(c);
        if self.ins_cycle_counter == Some(0) {
            self.reg_1 += x; 
        }
        self.try_reset_ins_cycle_counter();
    }

    fn update_ss_sum(&mut self) {
        if (self.clock - 20) % 40 == 0 && self.clock <= 220 {
            let ss = self.reg_1 * self.clock;
            self.ss_sum += ss;
            // println!("ss_sum += {}", ss);
        }
    }
}

impl Visitor<()> for Cpu {
    fn visit_instr(&mut self, ins: &Instr) {
        match ins {
            Instr::Noop(c) => self.exec_noop(*c),
            Instr::Addx(c, x) => self.exec_addx(*c, *x),
        }
    }
}


fn read_instructions(cpu: &mut Cpu, data: &str) -> i32 {
    let mut instr: Vec<Instr> = Vec::new();
    for line in data.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        match spl[0] {
            "noop" => instr.push(Instr::Noop(1)),
            "addx" => instr.push(Instr::Addx(2, spl[1].parse().unwrap())),
            &_ => (),
        };
    }

    cpu.run(&instr); 
    cpu.ss_sum
}

pub fn part1(data: &str) -> i32 {
    let mut cpu = Cpu::new();
    read_instructions(&mut cpu, data)
}

pub fn part2(data: &str) -> String {
    let mut cpu = Cpu::new();

    // Draw each tick
    cpu.crt_on = false;
    read_instructions(&mut cpu, data);
    cpu.draw_crt()
}

#[test]
fn day10_part1() {
    assert_eq!(part1(test_data::DATA1), 13140);
}

#[test]
fn day10_part2() {
    const RES: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    assert_eq!(part2(test_data::DATA1), RES);
}
