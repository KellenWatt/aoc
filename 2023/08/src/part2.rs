use std::io::stdin;
use std::collections::{HashMap, HashSet};
use std::cell::OnceCell;
use regex::Regex;

// could make this a whole lot better, but meh.
fn factorize(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut n = n;
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut f = 3;
    while n != 1 {
        while n % f == 0 {
            factors.push(f);
            n /= f;
        }
        f += 2;
    }

    factors
}

fn lcm(nums: Vec<u64>) -> u64 {
    let nums: HashSet<u64> = HashSet::from_iter(nums.iter().cloned());
    let mut factor_counts = HashMap::new();
    for n in nums.iter() {
        let factors = factorize(*n);
        let mut counts = HashMap::new();
        for f in factors.iter() {
            counts.entry(*f).and_modify(|n| *n += 1).or_insert(1);
        }
        for (f, count) in counts.iter() {
            factor_counts.entry(*f).and_modify(|n| {
                if *n > *count {
                    *n = *count;
                }
            }).or_insert(*count);
        }
    }
    factor_counts.iter().fold(1, |total, (f, e)| {
        total * f.pow(*e)
    }) 
}



struct StringRepeat {
    data: Vec<char>,
    step: usize,
}

impl StringRepeat {
    fn new(s: &str) -> StringRepeat {
        let data = s.chars().collect();
        StringRepeat{data, step: 0}
    }
}

impl Iterator for StringRepeat {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let res = self.data[self.step];
        self.step = (self.step + 1) % self.data.len();
        Some(res)
    }
}

#[derive(PartialEq, Hash)]
enum NodeType {
    Start,
    End,
    Other,
}

#[derive(Hash)]
struct Node {
    name: String,
    left: String,
    right: String,

    kind: NodeType,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Node {
        let last = name.chars().last().unwrap();
        let kind = match last {
            'Z' => NodeType::End,
            'A' => NodeType::Start,
            _   => NodeType::Other,
        };
        Node{name: name.to_owned(), left: left.to_owned(), right: right.to_owned(), kind}
    }

    fn is_start(&self) -> bool {
        self.kind == NodeType::Start
    }

    fn is_end(&self) -> bool {
        self.kind == NodeType::End
    }
}

fn main() {
    let node_pattern = OnceCell::new();

    let mut lines = stdin().lines().map(|l| l.unwrap());

    let insts = lines.next().unwrap();
    let mut insts = StringRepeat::new(&insts);
    let lines: Vec<_> = lines.skip(1).collect();

    let mut starts = vec![];

    let mut map = HashMap::new();
    for line in lines.iter() {
        let pat = node_pattern.get_or_init(|| {
            Regex::new(r"(?<name>.{3}) = \((?<left>.{3}), (?<right>.{3})\)").unwrap()
        });

        let caps = pat.captures(&line).unwrap();
        let name = caps.name("name").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();
        let node = Node::new(name, left, right);
        if node.is_start() {
            starts.push(name.to_owned());
        }
        map.insert(name.to_owned(), node);
    }


    let dists = starts.iter().map(|n| {
        let mut steps = 0;
        let mut n = n;
        while !map[n].is_end() {
            let inst = insts.next().unwrap();
            steps += 1;
            n = match inst {
                'L' => &map[n].left,
                'R' => &map[n].right,
                _ => panic!("unreachable") 
            }
        }
        steps
    }).collect();
    println!("{:?}", dists);

    let steps = lcm(dists);

    println!("{}", steps);

}
