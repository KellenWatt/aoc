use std::io::stdin;
use std::collections::HashMap;

macro_rules! debugln {
    ($($tts:tt)*) => {
        #[cfg(feature = "verbose")]
        println!($($tts)*)
    }
}
macro_rules! debug {
    ($($tts:tt)*) => {
        #[cfg(feature = "verbose")]
        print!($($tts)*)
    }
}



fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    let nums = lines.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut stones = HashMap::new();

    for n in nums {
        stones.entry(n).and_modify(|c| *c += 1).or_insert(1u64);
    }

    let times = 101;
    for _ in 0..times {
        let mut next_gen = HashMap::new();
        for (n, count) in stones {
            if n == 0 {
                next_gen.entry(1).and_modify(|c| *c += count).or_insert(count);
            } else if digit_count(n) % 2 == 0 {
                let (left, right) = split_num(n);
                next_gen.entry(left).and_modify(|c| *c += count).or_insert(count);
                next_gen.entry(right).and_modify(|c| *c += count).or_insert(count);
            } else {
                next_gen.entry(n * 2024).and_modify(|c| *c += count).or_insert(count);
            }
        }
        stones = next_gen;
    }

    let mut sum = 0u64;
    for n in stones.values() {

        sum = sum.checked_add(*n).unwrap();
    }
    println!("{:?}", stones.values().sum::<u64>());
}

fn digit_count(n: u64) -> u32 {
    n.ilog10() + 1
}

fn split_num(n: u64) -> (u64, u64) {
    let half = digit_count(n) / 2;
    let m = 10u64.pow(half);
    let left = n / m;
    let right = n % m;
    (left, right)
}

