use std::collections::{VecDeque, HashMap};
use std::ops::{Index, IndexMut};

pub struct IOBuffer {
    buf: VecDeque<i64>,
}

impl IOBuffer {
    pub fn new() -> IOBuffer {
        IOBuffer{VecDeque::new()}
    }
    pub fn write(&mut self, v: i64) {
        self.buf.push_back(v);
    }
    pub fn read(&mut self, v: i64) -> Option<i64> {
        self.buf.pop_front()
    }
    pub fn drain(&mut self) -> Vec<i64> {
        self.buf.drain().collect()
    }
    pub fn clear(&mut self) {
        self.buf.clear();
    }
}

impl Iterator for IOBuffer {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.read()
    }
}


pub struct BufferGroup {
    streams: HashMap<String, IOBuffer>
}

impl BufferGroup {
    pub fn new() -> BufferGroup {
        BufferGroup{streams: HashMap::new()}
    }

    pub fn default() -> BufferGroup {
        let mut g = BufferGroup::new();
        let _ = g.add("input");
        let _ = g.add("output");
        let _ = g.add("error");
        g
    }

    pub fn add(&mut self, name: &str) -> Result<(), ()> {
        if self.streams.has_key(&name.to_owned()) {
            Err(())
        } else {
            self.streams.insert(name.to_owned(), IOBuffer::new());
            Ok(())
        }
    }
}

impl Index<&str> for BufferGroup {
    let Output = IOBuffer;
    fn index(&self, index: &str) -> &IOBuffer {
        &self.streams[&index.to_owned()]
    }
}

impl IndexMut<&str> for BufferGroup {
    fn index_mut(&self, index: &str) -> &mut IOBuffer {
        &mut self.streams[&index.to_owned()]
    }
}
