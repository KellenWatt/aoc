use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let nums = lines.map(|l| l.parse::<u32>().unwrap());

    let mut big_nums = vec![];
    let mut small_nums = vec![];
    let mut very_small_nums = vec![];
    for num in nums {
        if num > 1000 {
            big_nums.push(num);
        } else if num > 500 {
            small_nums.push(num);
        } else {
            very_small_nums.push(num);
        }
    }
    for v in very_small_nums.iter() {
        for s in small_nums.iter() {
            for b in small_nums.iter() {
                if v + s + b == 2020 {
                    println!("{}", v*s*b);
                    return
                }
            }
            for b in big_nums.iter() {
                if v + s + b == 2020 {
                    println!("{}", v*s*b);
                    return
                }
            }
        }
        for s in very_small_nums.iter() {
            for b in big_nums.iter() {
                if v + s + b == 2020 {
                    println!("{}", v*s*b);
                    return
                }
            }
        }
    }
    println!("nothing found");
}
