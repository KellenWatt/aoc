use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use std::sync::OnceLock;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum State {
    Undecided, 
    Accept,
    Reject,
}
impl State {
    fn is_resolved(&self) -> bool {
        match self {
            State::Undecided => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
    state: State,
}

impl Part {
    fn blank() -> Part {
        Part{x: 0, m: 0, a: 0, s: 0, state: State::Undecided}
    }

    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, rating: &str) -> u32 {
        match rating {
            "x" => self.x, 
            "m" => self.m, 
            "a" => self.a, 
            "s" => self.s, 
            _ => unreachable!()
        }
    }

    fn resolved(&self) -> bool {
        self.state.is_resolved()
    }

    fn accepted(&self) -> bool {
        match self.state {
            State::Accept => true,
            _ => false,
        }
    }

    fn accept(&mut self) {
        if !self.resolved() {
            self.state = State::Accept;
        }
    }
    fn reject(&mut self) {
        if !self.resolved() {
            self.state = State::Reject;
        }
    }
}

impl FromStr for Part {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Part, Self::Err> {
        let mut p = Part::blank();
        for rating in s[1..(s.len()-1)].split(",") {
            let (r, value) = rating.split_once("=").unwrap();
            let value = value.parse().unwrap();
            match r {
                "x" => p.x = value,
                "m" => p.m = value,
                "a" => p.a = value,
                "s" => p.s = value,
                _ => panic!("unrecognized rating: {}", r)
            }
        }
        Ok(p)
    }
}

#[derive(Debug)]
enum Rule {
    Less(String, u32, String),
    Greater(String, u32, String),
    Jump(String),
    // Accept,
    // Reject,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Less(rating, operand, target) => {
                // println!("{}, {}", part.get(rating), *operand);
                // print!("   rule less: ");
                if part.get(rating) < *operand {
                    // println!("passed");
                    Some(target.clone())
                } else {
                    // println!("failed");
                    None
                }
            },
            Rule::Greater(rating, operand, target) => {
                if part.get(rating) > *operand {
                    Some(target.clone())
                } else {
                    None
                }
            },
            Rule::Jump(target) => Some(target.clone()),
            // Accept => {
            //     part.accept();
            //     None
            // },
            // Reject => {
            //     part.reject();
            //     None
            // }
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


struct Workflow {
    rules: Vec<Rule>,
}

static WORKFLOW_PATTERN: OnceLock<Regex> = OnceLock::new();
impl Workflow {
    fn from_str(s: &str) -> (String, Workflow) {
        let workflow = WORKFLOW_PATTERN.get_or_init(|| Regex::new(r"(?<name>\w+)\{(?<rules>.+)\}").unwrap());
        let caps = workflow.captures(s).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let rules = caps.name("rules").unwrap().as_str().split(",").map(|r| r.parse().unwrap());
        (name.to_owned(), Workflow{rules: rules.collect()})
    }
}

impl Workflow {
    fn apply(&self, p: &mut Part) -> Option<String> {
        // println!("For {:?}", p);
        for rule in self.rules.iter() {
            // println!("  applying {:?}", rule);
            let res = rule.apply(p);
            match res {
                Some(name) => {
                    return match name.as_str() {
                        "A" => {
                            p.accept();
                            None
                        },
                        "R" => {
                            p.reject();
                            None
                        },
                        name => Some(name.to_owned())
                    }
                },
                None => {continue;},
            }
        }
        unreachable!();
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let groups = lines.group_by(|l| l.trim().len() == 0);
    let mut groups = groups.into_iter().filter(|(key, _)| !key);

    let mut workflows = HashMap::new();
    for w in groups.next().unwrap().1 {
        let (name, wf) = Workflow::from_str(&w);
        workflows.insert(name, wf);
    }
    let mut parts: Vec<Part> = groups.next().unwrap().1.map(|p| p.parse().unwrap()).collect();


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
