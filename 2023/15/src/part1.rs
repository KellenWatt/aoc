use std::io::stdin;

fn hash(cs: &[u8]) -> u8 {
    cs.iter().fold(0u16, |acc, c| {
        let c = *c as u16;
        ((acc + c) * 17) & 0xFF
    }) as u8
}

fn main() {
    let line = stdin().lines().map(|l| l.unwrap()).next().unwrap();
    let segs = line.split(",");

    let total = segs.map(|word| {
        hash(word.as_bytes()) as u32
    }).sum::<u32>();

    println!("{}", total);
}
