use std::io::stdin;

#[derive(Clone, Copy, Debug)]
struct File {
    id: usize,
    size: usize,
}

impl File {
    fn replace_with_empty(self) -> Chunk {
        Chunk::empty(self.size)
    } 
}

#[derive(Debug)]
struct Chunk {
    capacity: usize,
    contents: Vec<File>, // id, size 
    full: bool
}

impl Chunk {
    fn empty(capacity: usize) -> Chunk {
        Chunk {
            capacity,
            contents: vec![],
            full: capacity == 0,
        }
    }

    fn len(&self) -> usize {
        self.contents.iter().map(|f| f.size).sum()
    }

    fn blocks_free(&self) -> usize {
        self.capacity - self.len()
    }

    fn can_hold_file(&self, size: usize) -> bool {
        self.blocks_free() >= size
    }

    fn push_file(&mut self, f: File) -> Result<(), ()> {
        if self.can_hold_file(f.size) {
            self.full = self.len() + f.size == self.capacity;
            self.contents.push(f);
            Ok(())
        } else {
            Err(())
        }
    }
}


#[derive(Debug)]
enum Slot {
    Chunk(Chunk),
    File(File),
}


fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    let descriptor = lines.next().unwrap();

    let mut mem: Vec<Slot> = Vec::with_capacity(descriptor.len());

    let mut file = true;
    let mut id = 0;
    for d in descriptor.bytes() {
        let size = (d - b'0') as usize;
        if file {
            mem.push(Slot::File(File{id, size}));
            id += 1;
        } else if size > 0{
            mem.push(Slot::Chunk(Chunk::empty(size)));
        }
        file = !file;
    }


    let mut i = mem.len();
    // starting from the back ...
    while i > 0 {
        i -= 1;
        match mem[i] {
            // We only care about moving full files, not chunks
            Slot::Chunk(_) => {},
            Slot::File(f) => {
                // search from start for chunks
                for c in 0..i {
                    match mem[c] {
                        // File can only be put in Chunks
                        Slot::Chunk(ref mut ch) if !ch.full => {
                            // at this point, f is File, ch is Chunk
                            if ch.can_hold_file(f.size) {
                                let _ = ch.push_file(f);
                                // replace the moved file with an empty chunk.
                                mem[i] = Slot::Chunk(f.replace_with_empty());
                                break;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
    }


    let mem = populate_memory(&mem);
    #[cfg(feature="verbose")]
    {
    for b in mem.iter() {
        if *b == -1 {
            print!(".");
        } else {
            print!("{}", b);
        }
    }
    println!();
    }
    println!("{}", checksum(mem));
}

fn populate_memory(m: &[Slot]) -> Vec<isize> {
    let mut mem = vec![-1; m.len() * 9];
    let mut i = 0;
    for s in m {
        match s {
            Slot::File(f) => {
                for j in i..i+f.size {
                    mem[j] = f.id as isize;
                }
                i += f.size;
            }
            Slot::Chunk(c) => {
                for f in c.contents.iter() {
                    for j in i..i+f.size {
                        mem[j] = f.id as isize;
                    }
                    i += f.size;
                }
                i += c.blocks_free();
            }
        }
    }


    mem.truncate(i);
    mem
}

fn checksum(m: Vec<isize>) -> usize {
    m.into_iter().enumerate().fold(0, |acc, (i, b)| {
        acc + if b == -1 {
            0
        } else {
            i * b as usize
        }
    })
}
