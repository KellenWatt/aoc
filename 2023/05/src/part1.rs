use std::io::stdin;
// use std::ops;
use itertools::Itertools;

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
        n >= self.src && n <= (self.src + self.len)
    }

    fn get(&self, n: i64) -> Option<i64> {
        if !self.contains(n) {
            None
        } else {
            Some(self.dest + (n - self.src))
        }
    }
}

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

    let seeds = seed_line.split(" ").skip(1).map(|s| s.parse().unwrap());

    let steps: Vec<RangeMap> = lines.map(|l| l.unwrap())
         .group_by(|l| l.trim().len() == 0).into_iter()
         .filter(|(key, _)| !key)
         .map(|(_, group)| {
        // skip the title, since it's consistent
        make_range_map(group.skip(1).collect())
    }).collect();

    let min_res = seeds.map(|seed| {
        print!("seed: {}", seed);
        let res = steps.iter().fold(seed, |s, step| {
            let tmp = step.get(s);
            print!(" > {}",tmp);
            tmp
        });
        
        println!("");
        res
    }).min().unwrap();

    println!("{}", min_res)

}
