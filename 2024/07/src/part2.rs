use std::io::stdin;
use std::collections::VecDeque;


fn concat(n: u64, m: u64) -> u64 {
    let pow = m.ilog10() + 1;
    n * 10u64.pow(pow) + m
}

fn valid_equation(target: u64, nums: Vec<u64>) -> Option<u64> {
    let number_count = nums.len();
    let mut stack = VecDeque::new();
    stack.push_back((1, nums[0]));
    while !stack.is_empty() {
        let (i, n) = stack.pop_back().unwrap();

        if i == number_count {
            if n == target {
                return Some(target);
            }
            continue;
        }

        for p in [n + nums[i], n * nums[i], concat(n, nums[i])] {
            if p <= target {
                stack.push_back((i+1, p));
            }
        }
    }

    None
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let total = lines.filter_map(|line| {
        let (target, nums) = line.split_once(": ").unwrap();
        let target = target.parse().unwrap();
        let nums = nums.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();

        valid_equation(target, nums)
    }).sum::<u64>();

    println!("{}", total)
}
