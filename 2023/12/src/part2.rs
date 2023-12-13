use std::io::stdin;
use std::sync::OnceLock;
use std::collections::HashMap;
use std::time::Instant;

static mut SEEN: OnceLock<HashMap<Row, usize>> = OnceLock::new();

#[derive(Hash, PartialEq, Eq)]
struct Row {
    data: Vec<char>,
    springs: Vec<usize>,
}

impl Row {
    fn new(data: &str) -> Row {
        let (line, springs) = data.split_once(" ").unwrap();
        let line = line.chars().collect();
        let springs = springs.split(",").map(|n| n.parse().unwrap()).collect();
        // println!("{:?} {:?}", line, springs);
        Row{data: line, springs}
    }
    
    fn quintuple(mut self) -> Row {
        self.data.push('?');
        let mut data = self.data.repeat(5);
        let _  = data.pop();
        let springs = self.springs.repeat(5);
        Row{data, springs}
    }


    fn sub_row(&self, skip: usize, consume_spring: bool) -> Row {
        let data = self.data[(skip * !(skip > self.data.len()) as usize)..].to_vec();
        let springs = self.springs[(consume_spring as usize)..].to_vec();
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
        if !self.data[..spring_chain].contains(&'.') && 
           (self.data.len() == spring_chain || self.data[spring_chain] != '#') {
            total = self.sub_row(spring_chain + 1, true).count();
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

    let start = Instant::now();
    let counts = lines.map(|line| {
        unsafe {
            // reset cache every iteration. Requires slightly more repeat calculations than
            // just leaving it be, but speeds up overall performance by about a full second
            // (unoptimzied. ~0.3s optimized)
            SEEN.get_mut().unwrap().clear();
        }
        // println!("{}", line);
        let c = Row::new(&line).quintuple().count();
        // println!("{}", c);
        c
    });

    println!("{}", counts.sum::<usize>());
    println!("{}", start.elapsed().as_secs_f32());
}
