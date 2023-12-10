use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let nums = lines.map(|l| l.parse::<u32>().unwrap());

    let mut big_nums = vec![];
    let mut small_nums = vec![];
    for num in nums {
        if num > 1000 {
            big_nums.push(num);
        } else {
            small_nums.push(num);
        }
    }

    for s in small_nums.iter() {
        for b in big_nums.iter() {
            if s + b == 2020 {
                println!("{}", s*b);
                return
            }
        }
    }
    println!("nothing found");
}
