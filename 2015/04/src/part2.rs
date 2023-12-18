use std::io::stdin;
use md5;

fn main() {
    let input = stdin().lines().next().unwrap().unwrap();

    let mut i: u32 = 254575;
    loop {
        i += 1;
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("000000") {
            println!("{}", i);
            return;
        }
    }
}
