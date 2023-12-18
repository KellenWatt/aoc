use std::io::stdin;
use std::collections::{VecDeque, HashMap};
use regex::Regex;

fn hash(cs: &[u8]) -> u8 {
    cs.iter().fold(0u16, |acc, c| {
        let c = *c as u16;
        ((acc + c) * 17) & 0xFF
    }) as u8
}

fn find_label(slots: &VecDeque<Lens>, label: &str) -> Option<usize> {
    for (i, l) in slots.iter().enumerate() {
        if l.label == label {
            return Some(i);
        }
    }
    None
}

#[derive(Hash)]
struct Lens {
    label: String,
    focal_length: u32,
}

type HashTable = HashMap<u8, VecDeque<Lens>>;

fn main() {
    let step_pattern = Regex::new(r"(?<label>\w+)(?<op>-|=)(?<focal>[1-9])?").unwrap();
    let line = stdin().lines().map(|l| l.unwrap()).next().unwrap();
    let segs = line.split(",");

    let mut slots: HashTable = HashMap::new();

    for segment in segs {
        let caps = step_pattern.captures(segment).unwrap();
        let label = caps.name("label").unwrap().as_str();
        let op = caps.name("op").unwrap().as_str();
        let focal_length = caps.name("focal").map(|s| s.as_str().parse().unwrap());

        let index  = hash(label.as_bytes());

        slots.entry(index).and_modify(|l| {
            match op {
                "=" => {
                    let idx = find_label(l, label);
                    if idx.is_none() {
                        l.push_back(Lens{label: label.to_string(), focal_length: focal_length.unwrap()});
                    } else {
                        l[idx.unwrap()].focal_length = focal_length.unwrap();
                    }
                },
                "-" => {
                    let idx = find_label(l, label);
                    if idx.is_some() {
                        let _ = l.remove(idx.unwrap());
                    }
                }
                _ => panic!("unexpected operation")
            }
        }).or_insert_with(|| {
            let mut l = VecDeque::new();
            if op == "=" {
                l.push_back(Lens{label: label.to_string(), focal_length: focal_length.unwrap()})
            }
            l
        });
    }

    let total: u32 = slots.iter().map(|(slot, lenses)| {
        lenses.iter().enumerate().map(|(i, lens)| {
            (*slot as u32 + 1) * (i as u32+1) * lens.focal_length
        }).sum::<u32>()
    }).sum();

    println!("{}", total);
}
