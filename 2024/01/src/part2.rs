use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut list1: Vec<u64> = vec![];
    let mut list2: Vec<u64> = vec![];
    for line in lines {
        let mut els = line.split_whitespace();
        list1.push(els.next().unwrap().parse().unwrap());
        list2.push(els.next().unwrap().parse().unwrap());
    }


    let mut total = 0;

    for a in list1 {
        for b in list2.iter() {
            if a == *b {
                total += a;
            }
        }
    }
    println!("{}", total);
    

}
