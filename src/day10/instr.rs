pub enum Instr {
    Noop(u32),
    Addx(u32, i32),
}

pub trait Visitor<T> {
    fn visit_instr(&mut self, ins: &Instr) -> T;
}

