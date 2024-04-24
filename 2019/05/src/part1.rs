use std::io::stdin;
use std::ops::{Deref, DerefMut};
use std::ops::{Add, Sub};
use std::collections::HashMap;

mod intcode;
use crate::intcode::memory::{Memory};
use crate::intcode::processor::Processor;
use crate::intcode::io::{IOBuffer, BufferGroup};




struct Computer {
    proc: Processor,
    mem: Memory,
    buffers: BufferGroup,
}

impl Computer {
    fn new() -> Computer {
        Computer{proc: Processor::new(), mem: Memory::new(), buffers: BufferGroup::default()}
    }

    fn compile(&mut self, prog: &str) -> Vec<i64> {
        prog.split(",").map(|n| n.parse.unwrap()).collect()
    }

    fn load(&mut self, prog: Vec<i64>) {
        self.mem.load(prog);
        self.proc.head = 0;
    }

    fn run(&mut self) {
        self.proc.run_program(&mut self.mem);
    }

    fn processor(&mut self) -> &mut Processor {
        &mut self.proc
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    for line in lines {
        // do something here
    }
}
