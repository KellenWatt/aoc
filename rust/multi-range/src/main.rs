use std::ops::{RangeInclusive, Range};
use std::fmt;

// Inclusive range
struct SubRange {
    start: isize,
    end: isize,
}

impl SubRange {
    fn empty() -> SubRange {
        SubRange{start: 1, end: 0}
    }

    fn overlaps(&self, other: &SubRange) -> bool {
        !(self.is_empty() || other.is_empty()) &&
        (self.start < other.end && !(self.end < other.start) ||
            other.start < self.end && !(other.end < self.start))
    }

    // fn union(&self, other: &SubRange) -> Result<SubRange, ()> {
    //     if self.overlaps(other) {
    //         let start = self.start.min(other.start);
    //         let end = self.end.max(other.end);
    //         SubRange{start, end}
    //     } else {
    //         Err(())
    //     }
    // }

    fn intersection(&self, other:&SubRange) -> SubRange {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            SubRange{start, end}
        } else {
            SubRange::empty()
        }
    }

    fn is_empty(&self) -> bool {
        self.start > self.end
    }
}

impl fmt::Display for SubRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[{},{}]", self.start, self.end)
    }
}

impl From<Range<isize>> for SubRange {
    fn from(r: Range<isize>) -> SubRange {
        SubRange{start: r.start, end: r.end-1}
    }
}

impl From<RangeInclusive<isize>> for SubRange {
    fn from(r: RangeInclusive<isize>) -> SubRange {
        SubRange{start: *r.start(), end: *r.end()}
    }
}

struct MultiRange {
    ranges: Vec<SubRange>
}

impl MultiRange {
    fn new() -> MultiRange {
        MultiRange{ranges: vec![]}
    }

    fn add(&mut self, s: SubRange) {
        if s.is_empty() {return;}
        if self.is_empty() {
            self.ranges.push(s);
            return;
        }
        for i in 0..self.ranges.len() {
            if self.ranges[i].overlaps(&s) {
                let mut j = i;
                while j+1 < self.ranges.len() && self.ranges[j+1].overlaps(&s) {
                    j += 1;
                }
                let start = self.ranges[i].start.min(s.start);
                let end = self.ranges[j].end.max(s.end);
                let new_range = SubRange{start, end};
                self.ranges.retain(|r| !r.overlaps(&new_range));
                self.ranges.insert(i, new_range);
                return;
            }
        }
        for i in 0..(self.ranges.len()) {
            if s.end < self.ranges[i].start {
                self.ranges.insert(i, s);
                return
            }
        }
        self.ranges.push(s);
    }

    fn remove(&mut self, s: SubRange) {
        if s.is_empty() {return;}
        for i in 0..self.ranges.len() {
            if self.ranges[i].overlaps(&s) {
                let mut j = i;
                while j+1 < self.ranges.len() && self.ranges[j].overlaps(&s) {
                    j += 1;
                }

                let new_start = SubRange{start: self.ranges[i].start, end: s.start-1};
                let new_end = SubRange{start: s.end+1, end: self.ranges[j].end};
                // println!("before: {}", new_start);
                // println!("after: {}", new_end);

                self.ranges.retain(|r| !r.overlaps(&s));
                if !new_end.is_empty() {
                    self.ranges.insert(i, new_end);
                }
                if !new_start.is_empty() {
                    self.ranges.insert(i, new_start);
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

impl From<Vec<SubRange>> for MultiRange {
    fn from(rs: Vec<SubRange>) -> MultiRange {
        let mut m = MultiRange::new();
        for r in rs {
            m.add(r);
        }
        m
    }
}

impl fmt::Display for MultiRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{")?;
        if self.ranges.len() > 1 {
            for i in 0..(self.ranges.len()-1) {
                write!(f, "{}, ", self.ranges[i])?;
            }
        }
        if let Some(r) = self.ranges.last() {
            write!(f, "{}", r)?;
        }
        write!(f, "}}")
    }
}


fn main() {
    let s = SubRange::from(10..=20);
    let mut m = MultiRange::new();
    m.add(s);
    println!("{}", m);
    m.add(SubRange::from(15..40));
    println!("{}", m);
    m.add(SubRange::from(0..5));
    println!("{}", m);
    m.add(SubRange::from(50..100));
    println!("{}", m);
    m.add(SubRange::from(39..=50));
    println!("{}", m);

    m.add(SubRange::from(40..45));
    println!("{}", m);

    m.remove(SubRange::from(50..51));
    println!("{}", m);

    m.remove(SubRange::from(30..61));
    println!("{}", m);

    m.remove(SubRange::from(100..1000));
    println!("{}", m);

    m.add(SubRange::from(35..46));
    println!("{}", m);

    m.add(SubRange::from(0..15));
    println!("{}", m);

    m.add(SubRange::from(29..=61));
    println!("{}", m);

    m.remove(SubRange::from(0..100));
    println!("{}", m);
}
