use std::ops::{RangeInclusive, Range, BitOr, BitAnd};
use std::fmt;

// Inclusive range
#[derive(Clone, Copy)]
pub struct SubRange {
    start: isize,
    end: isize,
}

impl SubRange {
    pub fn empty() -> SubRange {
        SubRange{start: 1, end: 0}
    }

    pub fn overlaps(&self, other: &SubRange) -> bool {
        !(self.is_empty() || other.is_empty()) &&
        (self.start < other.end && !(self.end < other.start) ||
            other.start < self.end && !(other.end < self.start))
    }

    pub fn union(&self, other: &SubRange) -> MultiRange {
        let mut m = MultiRange::new();
        m.add(*self);
        m.add(*other);
        m
    }

    pub fn intersection(&self, other: &SubRange) -> SubRange {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            SubRange{start, end}
        } else {
            SubRange::empty()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }
}

impl BitOr for &SubRange {
    type Output = MultiRange;
    fn bitor(self, rhs: &SubRange) -> MultiRange {
        self.union(rhs)
    }
}

impl BitAnd for &SubRange {
    type Output = SubRange;
    fn bitand(self, rhs: &SubRange) -> SubRange {
        self.intersection(rhs)
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

pub struct MultiRange {
    ranges: Vec<SubRange>
}

impl MultiRange {
    pub fn new() -> MultiRange {
        MultiRange{ranges: vec![]}
    }
    
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn add(&mut self, s: SubRange) {
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

    pub fn subtract(&mut self, s: SubRange) {
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

    pub fn union(&self, other: &MultiRange) -> MultiRange {
        let mut m = MultiRange::new();
        for r in self.ranges.iter() {
            m.add(*r);
        }
        for r in other.ranges.iter() {
            m.add(*r);
        }
        m
    }

    pub fn intersection(&self, other: &MultiRange) -> MultiRange {
        let mut m = MultiRange::new();

        for r in other.ranges.iter() {
            for s in self.ranges.iter() {
                if s.overlaps(r) {
                    m.add(s.intersection(r));
                }
            }
        }
        m
    }
    
    pub fn invert(&self, within: SubRange) -> Result<MultiRange, ()> {
        let mut m = MultiRange::new();
        if !self.is_empty() {
            if within.start > self.ranges[0].start || within.end < self.ranges[self.ranges.len() - 1].end {
                return Err(());
            }
        } else {
            m.add(within);
            return Ok(m);
        }

        m.add(SubRange::from(within.start..=(self.ranges[0].start - 1)));
        for i in 0..(self.ranges.len() - 1) {
            let start = self.ranges[i].end+1;
            let end = self.ranges[i+1].start-1;
            m.add(SubRange{start, end});
        }
        m.add(SubRange::from((self.ranges[self.ranges.len() - 1].end + 1)..=within.end));
        Ok(m)
    }

}

impl BitOr for &MultiRange {
    type Output = MultiRange;
    fn bitor(self, rhs: &MultiRange) -> MultiRange {
        self.union(rhs)
    }
}

impl BitAnd for &MultiRange {
    type Output = MultiRange;
    fn bitand(self, rhs: &MultiRange) -> MultiRange {
        self.intersection(rhs)
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


