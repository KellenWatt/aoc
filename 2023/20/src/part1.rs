use std::io::stdin;
use std::collections::{HashMap, VecDeque};
use regex::Regex;
use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    High,
    Low,
}

impl std::ops::Not for State {
    type Output = State;
    fn not(self) -> State {
        match self {
            State::High => State::Low,
            State::Low => State::High,
        }
    }
}

struct Signal {
    source: String,
    dest: String,
    value: State,
}

impl Signal {
    fn button() -> Signal {
        // the source name doesn't technically matter, but we'll call it 'button' to be nice
        Signal{source: "button".to_owned(), dest: "broadcaster".to_owned(), value: State::Low}
    }
}

trait Module {
    fn add_source(&mut self, src: &str);
    fn add_dest(&mut self, dest: &str);
    fn dests(&self) -> &Vec<String>;
    fn trigger(&mut self, s: Signal) -> Vec<Signal>;
}

struct Broadcaster {
    targets: Vec<String>,
}

impl Broadcaster {
    fn new() -> Broadcaster {
        Broadcaster{targets: vec![]}
    }
}

impl Module for Broadcaster {
    fn add_source(&mut self, _src: &str) {
        panic!("Broadcasters should never have a source");
    }

    fn add_dest(&mut self, dest: &str) {
        self.targets.push(dest.to_owned());
    }
    fn dests(&self) -> &Vec<String> {
        &self.targets
    }

    fn trigger(&mut self, _s: Signal) -> Vec<Signal> {
        self.targets.iter().map(|t| {
            Signal{source: String::from("broadcaster"), dest: t.clone(), value: State::Low}
        }).collect()
    }
}

struct FlipFlop {
    name: String,
    targets: Vec<String>,
    state: State,
}

impl FlipFlop {
    fn new(name: &str) -> FlipFlop {
        FlipFlop{name: name.to_owned(), targets: vec![], state: State::Low}
    }
}

impl Module for FlipFlop {
    fn add_source(&mut self, _src: &str) {
        // No-op. Flip-flop doesn't care about sources
    }
    fn add_dest(&mut self, dest: &str) {
        self.targets.push(dest.to_owned());
    }
    fn dests(&self) -> &Vec<String> {
        &self.targets
    }
    
    fn trigger(&mut self, s: Signal) -> Vec<Signal> {
        if let State::Low = s.value {
            self.state = !self.state;
            self.targets.iter().map(|t| {
                Signal{source: self.name.clone(), dest: t.clone(), value: self.state}
            }).collect()
        } else {
            vec![]
        }
    }
}

struct Conjunction {
    name: String,
    sources: HashMap<String, State>,
    targets: Vec<String>,
    state: State,
}

impl Conjunction {
    fn new(name: &str) -> Conjunction {
        Conjunction{
            name: name.to_owned(),
            sources: HashMap::new(),
            targets: vec![],
            state: State::High,
        }
    }

    fn all_high(&self) -> bool {
        self.sources.values().all(|v| v == &State::High)
    }
}

impl Module for Conjunction {
    fn add_source(&mut self, src: &str) {
        self.sources.insert(src.to_owned(), State::Low);
    }
    fn add_dest(&mut self, dest: &str) {
        self.targets.push(dest.to_owned());
    }
    fn dests(&self) -> &Vec<String> {
        &self.targets
    }
    
    fn trigger(&mut self, s: Signal) -> Vec<Signal> {
        self.sources.insert(s.source, s.value);
        let value = if s.value == State::High && self.all_high(){
            State::Low
        } else {
            State::High
        };

        self.targets.iter().map(|t| {
            Signal{source: self.name.clone(), dest: t.clone(), value}
        }).collect()
    }
}

fn main() {
    let module_pattern = Regex::new(r"(?<kind>%|&|)(?<name>[a-z]+) -> (?<dests>.+)").unwrap();
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    for line in lines {
        let caps = module_pattern.captures(&line).unwrap();
        let kind = caps.name("kind").unwrap().as_str();
        let name = caps.name("name").unwrap().as_str();
        let dests = caps.name("dests").unwrap().as_str().split(", ");

        let mut m: Box<dyn Module> = match kind {
            "%" => Box::new(FlipFlop::new(name)),
            "&" => Box::new(Conjunction::new(name)),
            "" if name == "broadcaster" => Box::new(Broadcaster::new()),
            _ => panic!("Unrecognized type: {}{}", kind, name),
        };
        for dest in dests {
            m.add_dest(dest);
        }

        modules.insert(name.to_owned(), m);
    }

    let names: Vec<_> = modules.keys().map(|k| k.clone()).collect();
    for name in names.iter() {
        let dests = modules[name].dests().clone();
        for d in dests.iter() {
            modules.get_mut(d).map(|m| m.add_source(&name));
        }
    }

    let start = Instant::now();
    let mut signals = 0;
    let iters = (0..100000).map(|i| {
        let mut queue = VecDeque::new();
        queue.push_back(Signal::button());
        let mut highs = 0u64;
        let mut lows = 0u64;
        while !queue.is_empty() {
            let s = queue.pop_front().unwrap();
            signals += 1;
            match s.value {
                State::High => highs += 1,
                State::Low => lows += 1,
            }
            // println!("{} -{:?}-> {}", s.source, s.value, s.dest);
            if s.dest == "rx" && s.value == State::Low {
                println!("\rpresses: {}", 0);
            }

            if !modules.contains_key(&s.dest) {
                continue;
            }

            for sig in modules.get_mut(&s.dest).unwrap().trigger(s) {
                queue.push_back(sig);
            }
        }

        // keeping I for potential cycle detection
        (i, highs, lows)
    });

    let mut high = 0;
    let mut low = 0;

    for (_i, h, l) in iters {
        high += h;
        low += l;
    }
    println!("-----------------------");
    println!("total signals: {}", signals);
    println!("  high pulses: {}", high);
    println!("  low pulses: {}", low);
    println!("elapsed: {}s", start.elapsed().as_secs_f32());
    println!("{}", high * low);
}
