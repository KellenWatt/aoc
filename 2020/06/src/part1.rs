use std::io::stdin;
use itertools::Itertools;

struct AlphaSet {
    data: u32,
}

impl AlphaSet {
    fn new() -> AlphaSet {
        AlphaSet{data: 0}
    }

    fn set(&mut self, c: char) {
        let idx = c as u32 - 97;
        self.data |= 1 << idx;
    }

    fn clr(&mut self, c: char) {
        let idx = c as u32 - 97;
        self.data &= !(1 << idx);
    }

    fn has(&mut self, c: char) -> bool {
        let idx = c as u32 - 97;
        (self.data & (1 << idx)) != 0
    }

    fn len(&self) -> u32 {
        self.data.count_ones()
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());
    let groups = lines.group_by(|l| l.trim().len() == 0);

    let counts = groups.into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            let mut set = AlphaSet::new();
            for person in group {
                for c in person.chars() {
                    set.set(c);
                }
            }
            set.len()
        });

    let total: u32 = counts.sum();

    println!("{}", total);
}
