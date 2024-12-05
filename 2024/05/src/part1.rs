use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());


    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut instrs = Vec::new();
    let mut reading_rules = true;
    for line in lines {
        if line.len() == 0 {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let (a, b) = line.split_once("|").unwrap();
            let a: u32 = a.parse().unwrap();
            let b: u32 = b.parse().unwrap();
            rules.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
        } else {
            instrs.push(line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>());
        }
    }

    let mut mids: Vec<u32> = vec![];

    // println!("{:?}", rules);

    'instructions:
    for inst_list in instrs {
        let mid = inst_list[inst_list.len()/2].clone();

        for (i, n) in inst_list.iter().enumerate() {
            for e in &inst_list[i+1..] {
                if rules.contains_key(e) && rules[e].contains(n) {
                    continue 'instructions;
                }
            }
        }
        println!("{:?}", inst_list);
        mids.push(mid);
    }
   
    use std::ops::Add;

    let total: u32 = mids.iter().sum();
    println!("{}", total);

}
