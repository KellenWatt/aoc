use std::io::stdin;
use itertools::Itertools;
use std::ops::BitAnd;

struct AlphaSet {
    data: u32,
}

impl AlphaSet {
    fn new() -> AlphaSet {
        AlphaSet{data: 0}
    }

    fn full() -> AlphaSet {
        AlphaSet{data: 67108863}
    }

    fn set(&mut self, c: char) {
        let idx = c as u32 - 97;
        self.data |= 1 << idx;
    }

    fn len(&self) -> u32 {
        self.data.count_ones()
    }
}

impl BitAnd for AlphaSet {
    type Output = AlphaSet;

    fn bitand(self, other: AlphaSet) -> AlphaSet {
        AlphaSet{data: self.data & other.data}
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());
    let groups = lines.group_by(|l| l.trim().len() == 0);

    let counts = groups.into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            let mut set = AlphaSet::full();
            group.fold(set, |consensus, person| {
                let mut answers = AlphaSet::new();
                for c in person.chars() {
                    answers.set(c);
                }
                consensus & answers
            }).len()
        });

    let total: u32 = counts.sum();

    println!("{}", total);
}
