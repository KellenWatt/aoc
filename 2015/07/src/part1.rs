use std::io::stdin;
use regex::Regex;

fn main() {
    let INSTRUCTION = Regex::new(r"(?<op>.*) -> (?<register>.+)");
    let Operation = Regex::new(r"(?<a>())")
    let lines = stdin().lines().map(|l| l.unwrap());

    for line in lines {
        // do something here
    }
}
