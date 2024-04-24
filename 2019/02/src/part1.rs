use std::io::stdin;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};
use std::ops::{Add, Sub};
use std::collections::{HashMap};

struct Mem {
    program: Vec<u32>,
    tape: Vec<u32>,
    head: usize
}

impl Mem {
    fn new() -> Mem {
        Mem{program: vec![], tape: vec![], head: 0}
    }

    fn reset_head(&mut self) -> &mut Self {
        self.head = 0;
        self
    }

    fn reset(&mut self) -> &mut Self {
        for (addr, c) in self.program.iter().enumerate() {
            if &self.tape[addr] != c {
                self.tape[addr] = *c;
            }
        }
        self.reset_head()
    }

    fn len(&self) -> usize {
        self.tape.len()
    }

    fn jump_to(&mut self, i: usize) -> Result<&mut Self, usize> {
        if i > self.len() {
            Err(self.len() - 1)
        } else {
            self.head = i;
            Ok(self)
        }
    }

    fn step(&mut self) -> Result<&mut Self, usize> {
        if self.head == self.len() - 1 {
            Err(self.len()-1)
        } else {
            self.head += 1;
            Ok(self)
        }
    }

    fn load(&self) -> &u32 {
        &self.tape[self.head]
    }

    fn load_as_pointer(&mut self) -> Pointer {
        Pointer{idx: self.head, memory: self as *mut Mem}
    }

    fn write(&mut self, v: u32) {
        self.tape[self.head] = v;
    } 

    fn get(&self, i: usize) -> Option<&u32> {
        self.tape.get(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut u32> {
        self.tape.get_mut(i)
    }
}

impl FromStr for Mem {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Mem, Self::Err> {
        let mut program = Vec::with_capacity(s.len()/2+1); // absolute maximum number of tape cells
        for c in s.split(",") {
            program.push(c.parse()?);
        }
        Ok(Mem{program: program.clone(), tape: program, head: 0})
    }
}

struct Pointer {
    idx: usize,
    memory: *mut Mem,
}

impl Pointer {
    fn shift(&self, n: isize) -> Result<Pointer, ()> {
        let new_idx = self.idx as isize + n;
        unsafe {
            if new_idx >= (*self.memory).len() as isize ||
               new_idx < 0 {
                return Err(());
            } 
        }
        let idx = new_idx as usize;
        Ok(Pointer{idx, memory: self.memory})
    }

    fn as_data(&self) -> &u32 {
        unsafe {
            (*self.memory).get(self.idx).expect("pointer is not valid")
        }
    }
}

impl Clone for Pointer {
    fn clone(&self) -> Pointer {
        Pointer{idx: self.idx, memory: self.memory}
    }
}

impl Deref for Pointer {
    type Target = u32;
    fn deref(&self) -> & u32 {
        unsafe {
            let addr = (*self.memory).get(self.idx).expect("pointer is not valid");
            (*self.memory).get(*addr as usize).expect(&format!("Pointer does not point to valid address: {}", addr))
        }
    }
}
impl DerefMut for Pointer {
    fn deref_mut(&mut self) -> & mut u32 {
        unsafe {
            let addr = (*self.memory).get(self.idx).expect("pointer is not valid");
            (*self.memory).get_mut(*addr as usize).expect(&format!("Pointer does not point to valid address: {}", addr))
        }
    }
}

impl Add<isize> for &Pointer {
    type Output = Pointer;
    fn add(self, n: isize) -> Pointer {
        
        self.shift(n).expect("offset is not in bounds")
    }
}

impl Sub<isize> for &Pointer {
    type Output = Pointer;
    fn sub(self, n: isize) -> Pointer {
        self.shift(-n).expect("offset is not in bounds")
    }
}


struct Processor {
    opcodes: HashMap<u32, Box<dyn Fn(Pointer) -> Option<usize>>>,
}

impl Processor {
    fn new() -> Processor {
        Processor{opcodes: HashMap::new()}
    }

    fn register_op<F>(&mut self, code: u32, f: F) 
        where F: Fn(Pointer) -> Option<usize> + 'static {
        self.opcodes.insert(code, Box::new(f));
    }

    fn exec_instr(&self, mem: &mut Mem) -> Option<usize> {
        let code = mem.load();
        self.opcodes.get(code).expect(&format!("{} is not a registered opcode", code))(mem.load_as_pointer())
    }

    fn run_program(&self, mem: &mut Mem) {
        while let Some(next) = self.exec_instr(mem) {
            mem.jump_to(next).expect(&format!("jumped to invalid memory cell: {}", next));
        }
    }
}


fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    let mut memory: Mem = lines.next().unwrap().parse().unwrap();

    let mut proc = Processor::new();
    proc.register_op(1, |op| {
        let arg1 = &op + 1;
        let arg2 = &op + 2;
        let mut output = &op + 3;
        *output = *arg1 + *arg2;
        Some(op.idx + 4)
    });

    proc.register_op(2, |op| {
        let arg1 = &op + 1;
        let arg2 = &op + 2;
        let mut output = &op + 3;
        *output = *arg1 * *arg2;
        Some(op.idx + 4)
    });

    proc.register_op(99, |_op| {
        None
    });

    for noun in 0..=99 {
        for verb in 0..=99 {
            memory.reset();
            memory.jump_to(1).unwrap().write(noun);
            memory.jump_to(2).unwrap().write(verb);
            memory.reset_head();

            proc.run_program(&mut memory);
            // println!("{}*{}: {:?}", noun, verb, memory.tape);

            if memory.get(0).unwrap() == &19690720 {
                println!("noun: {}", noun);
                println!("verb: {}", verb);
                println!("{}", noun *100 + verb);
                return
            }
        }
    }

    println!("None found");
}
