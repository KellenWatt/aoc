use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use std::sync::OnceLock;
use regex::Regex;
use std::collections::HashMap;

mod multirange;
use crate::multirange::{SubRange, MultiRange};


#[derive(Clone, Debug)]
struct AcceptablePart {
    x: MultiRange,
    m: MultiRange,
    a: MultiRange,
    s: MultiRange,
}

impl AcceptablePart {
    fn full() -> AcceptablePart {
        let range = MultiRange::from(SubRange::from(1..=4000));
        AcceptablePart{x: range.clone(), m: range.clone(), a: range.clone(), s: range.clone()}
    }

    fn empty() -> AcceptablePart {
        let range = MultiRange::new();
        AcceptablePart{x: range.clone(), m: range.clone(), a: range.clone(), s: range.clone()}
    }

    fn split(mut self, field: &str, at: u64) -> (AcceptablePart, AcceptablePart) {
        let mut m = MultiRange::new();
        m.add(SubRange::from(1..(at as isize)));
        let a = match field {
            "x" => self.x.clone(),
            "m" => self.m.clone(),
            "a" => self.a.clone(),
            "s" => self.s.clone(),
            _ => unreachable!()
        };
        m = m.intersection(&a);
        let inv = m.invert(SubRange::from(1..=4000)).unwrap();

        let mut other = self.clone();

        match field {
            "x" => {
                self.x = m;
                other.x = inv;
            },
            "m" => {
                self.m = m;
                other.m = inv;
            },
            "a" => {
                self.a = m;
                other.a = inv;
            },
            "s" => {
                self.s = m;
                other.s = inv;
            },
            _ => unreachable!()
        }
        
        (self, other)

    }

    fn union(&self, other: &AcceptablePart) -> AcceptablePart {
        let mut out = self.clone();
        out.x = out.x.union(&other.x);
        out.m = out.m.union(&other.m);
        out.a = out.a.union(&other.a);
        out.s = out.s.union(&other.s);
        out
    }
    
    fn intersection(&self, other: &AcceptablePart) -> AcceptablePart {
        let mut out = self.clone();
        out.x = out.x.intersection(&other.x);
        out.m = out.m.intersection(&other.m);
        out.a = out.a.intersection(&other.a);
        out.s = out.s.intersection(&other.s);
        out
    }

    fn invert(&self, field: &str) -> AcceptablePart {
        let mut other = self.clone();
        let within = SubRange::from(1..=4000);
        match field {
            "x" => other.x.invert(within).unwrap(),
            "m" => other.m.invert(within).unwrap(),
            "a" => other.a.invert(within).unwrap(),
            "s" => other.s.invert(within).unwrap(),
            _ => unreachable!()
        };
        other
    }
    
    fn invert_full(&self) -> AcceptablePart {
        let mut other = self.clone();
        let within = SubRange::from(1..=4000);
        other.x = other.x.invert(within).unwrap();
        other.m = other.m.invert(within).unwrap();
        other.a = other.a.invert(within).unwrap();
        other.s = other.s.invert(within).unwrap();
        other
    }

    fn count(self) -> usize {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

struct Rule {
    rating: String,
    acceptance: AcceptablePart,
    target: String,
}


static RULE_PATTERN: OnceLock<Regex> = OnceLock::new();

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let rule = RULE_PATTERN.get_or_init(|| Regex::new(r"(?:(?<rating>[xmas])(?<op>>|<)(?<operand>\d+):)?(?<target>\w+)").unwrap());
        let caps = rule.captures(s).unwrap();
        let target = caps.name("target").unwrap().as_str().to_owned();
        if let Some(rating) = caps.name("rating") {
            let rating = rating.as_str();
            let op = caps.name("op").unwrap().as_str();
            let operand = caps.name("operand").unwrap().as_str().parse().unwrap();
            let mut acceptance = AcceptablePart::full();
            acceptance = match op {
                "<" => acceptance.split(rating, operand).0,
                ">" => acceptance.split(rating, operand+1).1,
                _ => unreachable!()
            };
            Ok(Rule{rating: rating.to_string(), acceptance, target})
        } else {
            Ok(Rule{rating: "".to_string(), acceptance: AcceptablePart::full(), target})
        }
    }
}


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
        (name.to_owned(), w)
    }

}


fn workflow_acceptance(name: &str, workflows: &HashMap<String, Workflow>, history: &mut HashMap<String, AcceptablePart>) -> AcceptablePart {
    println!("checking acceptance of: {}", name);

    if name == "A" {
        return AcceptablePart::full()
    } else if name == "R" {
        return AcceptablePart::empty()
    }

    if history.contains_key(&name.to_string()) {
        println!("  reusing previous result");
        return history.get(&name.to_string()).unwrap().clone();
    }
    
    // workflow cost union of all rules costs
    // rule costs are intersect self, negs of all previous, and associated target

    let mut mask = AcceptablePart::full();

    let mut rule_acc = vec![];

    let w = &workflows[name];
    for r in w.rules.iter() {
        println!("checking rule: {:?}", r.acceptance);
        let a = r.acceptance.intersection(&mask);
       
        let a = a.intersection(&workflow_acceptance(&r.target, workflows, history));
        println!("after checking sub-workflow: {:?}", a);

        if r.rating != "" {
            mask = mask.intersection(&r.acceptance.invert(&r.rating));
        }
        rule_acc.push(a);
    }

    let mut res = rule_acc.iter().fold(AcceptablePart::full(), |acc, r| {
        acc.union(r)
    });

    println!("final workflow: {:?}", res);
    history.insert(name.to_string(), res.clone());
    res
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut workflows = HashMap::new();

    for w in lines.take_while(|l| l.trim().len() > 0) {
        let (name, w) = Workflow::create(&w);
        workflows.insert(name, w);
    }    

    let total_acceptance = workflow_acceptance("pv", &workflows, &mut HashMap::new());

    println!("{}", total_acceptance.count());

}
