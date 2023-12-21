use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use std::sync::OnceLock;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{RangeInclusive, Range};

// Inclusive range
struct SubRange {
    start: isize,
    end: isize,
}

impl SubRange {
    fn empty() -> SubRange {
        SubRange{1, 0}
    }

    fn overlaps(&self, other: &SubRange) -> bool {
        !(self.is_empty() || other.is_empty())
        self.start < other.end && !(self.end < other.start) ||
            other.start < self.end && !(other.end < self.start)
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
            SubRange{1, 0}
        }
    }

    fn is_empty(&self) -> bool {
        self.start > self.end
    }
}

impl From<Range> for SubRange {
    fn from(r: Range) -> SubRange {
        SubRange{start: r.start, end: r.end-1}
    }
}

impl From<RangeInclusive> for SubRange {
    fn from(r: RangeInclusive) -> SubRange {
        SubRange{start: r.start(), end: r.end()}
    }
}

struct MultiRange {
    ranges: Vec<SubRange>;
}

impl MultiRange {
    fn new() -> MultiRange {
        MultiRange{ranges: vec![]}
    }

    fn add(&mut self, s: SubRange) {
        if self.is_empty() {
            self.ranges.push(s);
            return;
        }
        for i in 0..self.ranges.len() {
            if self.ranges[i].overlaps(&s) {
                let j = i;
                while self.ranges[j].overlaps(&s) {
                    j += 1;
                }
                let start = self.ranges[i].start.min(s.start);
                let end = self.ranges[j].end.max(s.end);
                let new_range = SubRange{start, end};
                self.ranges.retain(|r| !r.overlaps(new_range));
                self.ranges.insert(i, new_range);
            }
        }
        
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}




#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
    // state: State,
}

#[derive(Clone)]
struct AcceptablePart {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

impl AcceptablePart {
}

// fn intersect_range(r: RangeInclusive, s: RangeInclusive) -> RangeInclusive {
//     if r.end() < s.start() || s.end() < r.start(){
//         // empty
//         1..=0 
//     } else {
//         (r.start().max(s.start()))..=(r.end().min(s.end()))
//     }
// }
// 
// // only joins overlapping ranges. Non-overlapping ranges have an empty result
// fn join_overlapping_range(r: RangeInclusive, s: RangeInclusive) -> RangeInclusive {
//     if r.end() < s.start() || s.end() < r.start(){
//         // empty
//         1..=0 
//     } else {
//         (r.start().min(s.start()))..=(r.end().max(s.end()))
//     }
// }

fn split_range(r: RangeInclusive, n: u64) -> (RangeInclusive, RangeInclusive) {
    if n > r.end() {
        (r, 1..=0)
    } else if n < r.start() {
        (1..=0, r)
    } else {
        (r.start()..=n, (n+1)..=r.end())
    }
}

impl AcceptablePart {
    fn full() -> AcceptablePart {
        let range = 1..=4000;
        AcceptablePart(x: range, m: range, a: range, s: range)
    }

    fn empty() -> AcceptablePart {
        let range = 1..=0;
        AcceptablePart(x: range, m: range, a: range, s: range)
    }

    fn split(mut self, field: &str, at: u64) -> (AcceptablePart, AcceptablePart) {
        let mut other = self.clone();
        match field {
            "x" => {
                let (a, b) = split_range(self.x);
                self.x = a;
                other.x = b;
            },
            "m" => {
                let (a, b) = split_range(self.m);
                self.m = a;
                other.m = b;
            },
            "a" => {
                let (a, b) = split_range(self.a);
                self.a = a;
                other.a = b;
            },
            "s" => {
                let (a, b) = split_range(self.s);
                self.s = a;
                other.s = b;
            },
            _ => unreachable!()
        }
        (self, other)
    }

    fn count(self) -> u64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

#[derive(Debug)]
enum Rule {
    Less(String, u64, String),
    Greater(String, u64, String),
    Jump(String),
    // Accept,
    // Reject,
}

impl Rule {
    fn target(&self) -> &String {
        match self {
            Rule::Less(_, _, t) => t,
            Rule::Greater(_, _, t) => t,
            Rule::Jump(t) => t,
        }
    }
}

static RULE_PATTERN: OnceLock<Regex> = OnceLock::new();

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let rule = RULE_PATTERN.get_or_init(|| Regex::new(r"(?:(?<rating>[xmas])(?<op>>|<)(?<operand>\d+):)?(?<target>\w+)").unwrap());
        let caps = rule.captures(s).unwrap();
        let target = caps.name("target").unwrap().as_str();
        if let Some(rating) = caps.name("rating") {
            let rating = rating.as_str();
            let op = caps.name("op").unwrap().as_str();
            let operand = caps.name("operand").unwrap().as_str().parse().unwrap();
            Ok(match op {
                "<" => Rule::Less(rating.to_owned(), operand, target.to_owned()),
                ">" => Rule::Greater(rating.to_owned(), operand, target.to_owned()),
                _ => unreachable!()
            })
        } else {
            Ok(Rule::Jump(target.to_owned()))
        }
        
    }
}

impl Rule {
    fn limit(&self, a: mut AcceptablePart) -> AcceptablePart {
        match self {
            Rule::Less(f, o, _) => a.split(f, o-1).0,
            Rule::Greater(f, o, _) = {
                let (_rest, me) = a.split(f, o);
                me
            }
            Rule::Jump(_) => panic!("shouldn't be called here");
        }
    }
}

static mut WORKFLOWS: OnceLock<HashMap<String, Workflow>> = OnceLock::new();

struct Workflow {
    rules: Vec<Rule>,
}

static WORKFLOW_PATTERN: OnceLock<Regex> = OnceLock::new();
impl Workflow {
    fn create(s: &str) -> (String, Workflow) {
        let workflow = WORKFLOW_PATTERN.get_or_init(|| Regex::new(r"(?<name>\w+)\{(?<rules>.+)\}").unwrap());
        let caps = workflow.captures(s).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let rules = caps.name("rules").unwrap().as_str().split(",").map(|r| r.parse().unwrap());

        let w = Workflow{rules: rules.collect()};
        unsafe {
            WORKFLOWS.get_mut().unwrap().insert(name.to_owned(), w);
        }
    }

    fn acceptance(&self) -> AcceptablePart {
        if self.is_final() {
            let mut res = AcceptablePart::empty();
            let mut rest = AcceptablePart::full();
            for rule in self.rules.iter() {
                let (pos, neg) = rule.limit(rest);
                // generate all independently
                // detect overlap between current and latter (each) 
                // sum
                // subtract total overlap from total
            }
        }
    }

    fn is_final(&self) -> bool {
        rules.iter().all(|r| {
            r.target() == "A" || r.target() == "R"
        })
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    for w in lines.take_while(|l| l.trim().len() > 0) {
        Workflow::create(&w);
    }

    let mut total = 0;
    for part in parts.iter_mut() {
        let mut flow = &workflows["in"];
        // println!("checking workflow 'in'");
        // print!("rules: ");
        // for rule in flow.rules.iter() {
        //     print!("{:?} ", rule);
        // }
        // println!("");
        while let Some(next) = flow.apply(part) {
            // println!("checking workflow '{}'", next);
            // print!("rules: ");
            // for rule in flow.rules.iter() {
            //     print!("{:?} ", rule);
            // }
            // println!("");
            flow = &workflows[&next];
        }

        if part.accepted() {
            total += part.rating();
        }
    }

    println!("{}", total);

}
