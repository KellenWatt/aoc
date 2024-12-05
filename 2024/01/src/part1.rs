use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut list1: Vec<u64> = vec![];
    let mut list2: Vec<u64> = vec![];
    for line in lines {
        let mut els = line.split_whitespace();
        list1.push(els.next().unwrap().parse().unwrap());
        list2.push(els.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();
    let res = list1.iter().zip(list2).fold(0u64, |acc, (a, b)| {
        acc + a.abs_diff(b)
    });

    println!("{}", res);
}
