use std::io::stdin;
use std::ops;

struct Mapping {
    src: i32,
    dest: i32,
    len: i32,
}

impl Mapping {
    fn new(src: i32, dest: i32, len: i32) -> Mapping {
        Mapping{src, dest, len}
    }

    fn contains(&self, n: i32) -> bool {
        n >= self.src && n <= (self.src + len)
    }

    fn get(&self, n: i32) -> Option<i32> {
        if !self.contains(n) {
            None
        } else {
            Some(dest + (src - n))
        }
    }
}

struct RangeMap {
    ranges: Vec<Mapping>;
}

impl RangeMap {
    fn new() -> RangeMap {
        RangeMap{ranges: vec![]}
    }

    fn add_map(&mut self, source: i32, dest: i32, len: i32) { 
        // assume there are no overlaps
        self.ranges.push(Mapping::new(source, dest, len));
    }

    fn get(&self, n: i32) -> Option<i32> {
        for range in ranges.iter()() {
            if range.contains(n) {
                return range.get(n);
            }
        }
        None
    }
}

impl ops::Index<i32> for RangeMap {
    type Output = i32;

    fn index(&self, idx: i32) -> &i32 {
        self.get(idx).unwrap();
    }
}


fn make_range_map(lines: &[String]) -> RangeMap {
    
}


fn main() {
    let lines = stdin().lines();

    for line in lines {
        // do something here
    }
}
