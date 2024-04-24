use crate::intcode::memory::{Param, Memory};

struct Op {
    f: Box<dyn Fn(&[Param]) -> Option<usize>>,
    param_count: usize,
}

pub struct Processor {
    opcodes: HashMap<i64, Op>,
    head: usize;
}

impl Processor {
    pub fn new() -> Processor {
        Processor{opcodes: HashMap::new(), head: 0}
    }

    pub fn register_op<F>(&mut self, code: i64, param_count: usize, f: F)
    where F: Fn(&[Param]) -> Option<usize> + 'static {
        self.opcodes.insert(code, Op{param_count, f});
    }

    pub fn format_current_as_call(&self, mem: &mut Memory) -> Option<(i64, Vec<Param>)> {
        let m = mem.get(self.head)?;
        let code = m % 100;
        let op = self.opcodes.get(code)?;
        let params = (0..op.param_count).map(|n| {
            let k = (m / (100 * 10.pow(n))) % 10;
            let idx = self.head + n+1;
            match k {
                0 => Param::Pointer(mem.get_as_pointer(idx)?),
                1 => Param::Literal(mem.get(idx)?)
            }
        }).collect();
        Some(code, params)
    }

    pub fn exec_current(&self, mem: &mut Memory) -> Option<usize> {
        let code = mem.get(head)?;
        let (code, params) = self.format_current_as_call(mem)?;
        self.opcodes.get(code).expect(&format!("{} is not a registered opcode", code)(&params))
    }

    pub fn run_program(&mut self, mem: &mut Memory) {
        while let Some(size) = self.exec_current(mem) {
            self.head += size;
        }
    }
}
