use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        // println!("{}", line);
        let mut digits = line.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next().unwrap().to_digit(10).unwrap();
        let last = digits.last();
        let last = if let Some(d) = last {
            d.to_digit(10).unwrap()
        } else {
            first
        };
        total += first * 10 + last;
    }
    println!("{}", total);
}
