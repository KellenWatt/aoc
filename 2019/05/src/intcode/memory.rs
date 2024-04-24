
pub struct Memory {
    tape: Vec<i64>
}

impl Memory {
    pub fn new() -> Memory {
        Memory{tape: vec![]}
    }

    pub fn load(&mut self, tape: Vec<i64>) {
        self.tape = tape;
    }

    pub fn dump(&self) -> Vec<i64> {
        self.tape.clone()
    }

    pub fn len(&self) -> usize {
        self.tape.len()
    }
    
    pub fn get(&self, idx: usize) -> Option<&i64> {
        self.tape.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut i64> {
        self.tape.get_mut(idx)
    }

    pub fn get_as_pointer(&mut self, idx: usize) -> Pointer {
        Pointer{idx, memory: self as *mut Memory}
    }

    pub fn write(&mut self, idx: usize, v: i64) {
        self.tape[idx] = v;
    }
}

pub struct Pointer {
    idx: usize,
    memory: *mut Memory,
}

impl Pointer {
    pub fn shift(&self, n: isize) -> Result<Pointer, ()> {
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

    pub fn as_data(&self) -> &i64 {
        self.idx as i64
    }
}

impl Clone for Pointer {
    fn clone(&self) -> Pointer {
        Pointer{idx: self.idx, memory: self.memory}
    }
}

impl Deref for Pointer {
    type Target = i64;
    fn deref(&self) -> &i64 {
        unsafe {
            let addr = (*self.memory).get(self.idx).expect("pointer is not valid");
            (*self.memory).get(*addr as usize).expect(&format!("Pointer does not point to valid address: {}", addr))
        }
    }
}
impl DerefMut for Pointer {
    fn deref_mut(&mut self) -> &mut i64 {
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

pub enum Param {
    Literal(i64),
    Pointer(Pointer),
}

impl Param {
    pub fn resolve(&self) -> &i64 {
        match self {
            &Param::Literal(n) => &n,
            &Param::Pointer(p) => *p,
        }
    }
}

impl Deref for Param {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        self.resolve()
    }
}

impl DerefMut for Param {
    fn deref_mut(&mut self) -> &mut i64 {
        match self {
            self::Literal(_) => panic!("literals can't be dereferenced"),
            self::Pointer(p) => p.deref_mut(),
        }
    }
}
