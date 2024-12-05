use regex::Regex;
use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0u64;
    for line in lines {
        for cap in mul.captures_iter(&line) {
            // println!("{}", &cap[0]);
            let a: u64 = cap[1].parse().unwrap();
            let b: u64 = cap[2].parse().unwrap();
            println!("{} * {}", a, b);
            total += a*b
        }
    }

    println!("{}", total);
}
