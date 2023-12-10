use std::io::stdin;
use std::collections::HashMap;
use std::cell::OnceCell;
use regex::Regex;


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

#[derive(Hash)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn main() {
    let node_pattern = OnceCell::new();

    let mut lines = stdin().lines().map(|l| l.unwrap());

    let insts = lines.next().unwrap();
    let mut insts = StringRepeat::new(&insts);
    let mut lines: Vec<_> = lines.skip(1).collect();

    // let start = lines[0].split_once(" = ").unwrap().0.to_owned();
    // let end = lines[lines.len()-1].split_once(" = ").unwrap().0.to_owned();
    let start = "AAA".to_owned();
    let end = "ZZZ".to_owned();

    let mut map = HashMap::new();
    for line in lines.iter() {
        let pat = node_pattern.get_or_init(|| {
            Regex::new(r"(?<name>.{3}) = \((?<left>.{3}), (?<right>.{3})\)").unwrap()
        });

        let caps = pat.captures(&line).unwrap();
        let name = caps.name("name").unwrap().as_str().to_owned();
        let left = caps.name("left").unwrap().as_str().to_owned();
        let right = caps.name("right").unwrap().as_str().to_owned();
        let node = Node{name: name.clone(), left, right};
        map.insert(name, node);
    }

    let mut steps = 0;
    let mut node = &start;

    while node != &end {
        println!("{}", node);
        let inst = insts.next().unwrap();
        steps += 1;
        node = match inst {
            'L' => &map[node].left,
            'R' => &map[node].right,
            _ => panic!("unreachable")
        };
    }

    println!("{}", steps);

}
