use std::io::stdin;
use std::sync::OnceLock;
use std::collections::HashMap;

static mut SEEN: OnceLock<HashMap<Row, usize>> = OnceLock::new();

#[derive(Hash, PartialEq, Eq)]
struct Row {
    data: Vec<char>,
    springs: Vec<usize>,
}

impl Row {
    fn new(data: &str) -> Row {
        let mut data = data.split(" ");
        let line = data.next().unwrap().chars().collect();
        let springs = data.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        // println!("{:?} {:?}", line, springs);
        Row{data: line, springs}
    }

    fn sub_row(&self, skip: usize, consume: bool) -> Row {
        let data = if skip > self.data.len() {
            vec![]
        } else {
            self.data[skip..].to_vec()
        };

        let springs = if consume {
            &self.springs[1..]
        } else {
            &self.springs[..]
        }.to_vec();
        Row{data, springs}
    }

    fn count(self) -> usize {
        unsafe {
            if SEEN.get_mut().unwrap().contains_key(&self) {
                return *SEEN.get().unwrap().get(&self).unwrap();
            }
        }
        let mut total = 0;
        if self.springs.is_empty() {
            return (!self.data.contains(&'#')) as usize;
        }
        let spring_chain = self.springs[0];
        if self.data.len() < spring_chain {
            return 0;
        }
        if !self.data[..spring_chain].contains(&'.') {
            if self.data.len() == spring_chain || self.data[spring_chain] != '#' {
                total = self.sub_row(spring_chain + 1, true).count();
            }
        }
        if self.data[0] != '#' {
            total += self.sub_row(1, false).count();
        }
        unsafe {
            SEEN.get_mut().unwrap().insert(self, total);
        }
        total
    }
}



fn main() {
    unsafe {
        let _ = SEEN.set(HashMap::new());
    }
    let lines = stdin().lines().map(|l| l.unwrap());

    let counts = lines.map(|line| {
        let c = Row::new(&line).count();
        c
    });

    println!("{}", counts.sum::<usize>());
}
