use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let total = lines.map(|l| l.parse::<u32>().unwrap() / 3 - 2).sum::<u32>();
    println!("{}", total);
}
