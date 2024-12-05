use regex::Regex;
use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let pat = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(?:n't)?\(\)").unwrap();
    let mut total = 0u64;
    let mut it_counts = true;
    for line in lines {
        let mut offset = 0;
        while let Some(cap) = pat.captures(&line[offset..]) {
            let mat = cap.get(0).unwrap();
            match mat.as_str() {
                "do()" => {
                    it_counts = true;
                }
                "don't()" => {
                    it_counts = false;
                }
                _ => { // mul
                    if it_counts {
                        let a: u64 = cap[1].parse().unwrap();
                        let b: u64 = cap[2].parse().unwrap();
                        total += a * b;
                    }
                }
            }
            offset += mat.end();
        }
    }

    println!("{}", total);
}
