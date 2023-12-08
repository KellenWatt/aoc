
// Absolute maximum: 1493867

use std::io::stdin;
// use std::ops;
use itertools::Itertools;
use std::thread;

#[derive(Clone, Debug)]
struct Mapping {
    src: i64,
    dest: i64,
    len: i64,
}

impl Mapping {
    fn new(src: i64, dest: i64, len: i64) -> Mapping {
        Mapping{src, dest, len}
    }

    fn contains(&self, n: i64) -> bool {
        n >= self.dest && n <= (self.dest + self.len)
    }

    fn get(&self, n: i64) -> Option<i64> {
        if !self.contains(n) {
            None
        } else {
            // reverse map
            Some(self.src + (n - self.dest))
        }
    }
}

#[derive(Clone, Debug)]
struct RangeMap {
    ranges: Vec<Mapping>,
}

impl RangeMap {
    fn new() -> RangeMap {
        RangeMap{ranges: vec![]}
    }

    fn add_map(&mut self, source: i64, dest: i64, len: i64) { 
        // assume there are no overlaps
        self.ranges.push(Mapping::new(source, dest, len));
    }

    fn get(&self, n: i64) -> i64 {
        for range in self.ranges.iter() {
            if range.contains(n) {
                return range.get(n).unwrap();
            }
        }
        n
    }
}


fn make_range_map(lines: Vec<String>) -> RangeMap {
    let mut ranges = RangeMap::new();
    for line in lines.iter() {
        let mut nums = line.split(" ").map(|n| n.parse().unwrap());
        let dest = nums.next().unwrap();
        let src = nums.next().unwrap();
        let len = nums.next().unwrap();
        ranges.add_map(src, dest, len);
    }
    ranges
}


fn main() {
    let mut lines = stdin().lines();

    let seed_line = lines.next().unwrap().unwrap();
    println!("{}", seed_line);

    let mut total = 0;
    let mut seed_count = 0;
    let seeds = seed_line.split(" ").skip(1).map(|s| s.parse().unwrap()).chunks(2);
    let seeds: Vec<_> = seeds.into_iter().map(|mut pair| {
        let start: i64 = pair.next().unwrap();
        let len = pair.next().unwrap();
        seed_count += len;
        total += len;
        start..(start+len)
    }).collect();
    println!("{}", seed_count);

    let steps: Vec<RangeMap> = lines.map(|l| l.unwrap())
         .group_by(|l| l.trim().len() == 0).into_iter()
         .filter(|(key, _)| !key)
         .map(|(_, group)| {
        // skip the title, since it's consistent
        make_range_map(group.skip(1).collect())
    }).collect();
    let steps: Vec<_> = steps.iter().rev().collect();

    // let n = 82;
    // println!("-- {} --", n);
    // let out = steps.iter().fold(n, |s, step| {
    //     // println!("{:?}", step);
    //     let next = step.get(s);
    //     print!("< {} ", next);
    //     next
    // });
    // println!("");

    println!("{:?}", seeds);

    for n in 0..1493867 {
        // println!("-- {} --", n);
        let possible_seed = steps.iter().fold(n, |s, step| {
            let next = step.get(s);
            // println!("{}", next);
            next
        });
        // println!("{} < {}", n, possible_seed);
    
        for seed_range in seeds.iter() {
            if seed_range.contains(&possible_seed) {
                println!("{}", n);
                return;
            }
        }
    }
    println!("No answer found");
}
