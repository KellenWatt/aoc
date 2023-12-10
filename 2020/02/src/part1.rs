use std::io::stdin;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

struct Req {
    letter: char, 
    acceptable: RangeInclusive<usize>,
}

impl Req {
    fn new(min: usize, max: usize, letter: char) -> Req {
        Req{letter, acceptable: min..=max}
    }
    fn is_valid(&self, pass: &str) -> bool {
        let mut counts = HashMap::new();
        
        for c in pass.chars() {
            counts.entry(c).and_modify(|n| *n += 1).or_insert(1usize);
        }
        
        let actual = counts.get(&self.letter).unwrap_or(&0);
        self.acceptable.contains(actual)
    }
}


fn main() {
    let line_pattern = Regex::new(r"(?<min>\d+)-(?<max>\d+) (?<letter>[a-z]): (?<password>[a-z]+)").unwrap();
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut total = 0;
    for line in lines {
        let caps = line_pattern.captures(&line).unwrap();
        let min = caps.name("min").unwrap().as_str().parse().unwrap();
        let max = caps.name("max").unwrap().as_str().parse().unwrap();
        let letter = caps.name("letter").unwrap().as_str().parse().unwrap();
        let pass = caps.name("password").unwrap().as_str();
        if Req::new(min, max, letter).is_valid(pass) {
            total += 1;
        }
    }
    println!("{}", total);
}
