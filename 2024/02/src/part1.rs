use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut safe = 0u32;
    'outer:
    for line in lines {
        let reports = line.split_whitespace().map(|e| e.parse().unwrap()).collect::<Vec<i64>>();
        println!("{:?}", reports);

        let mut last_sign = 0;
        for pair in reports.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let new_delta = a - b;
            if last_sign != new_delta.signum() && last_sign != 0 {
                continue 'outer;
            }
            if ![1i64,2,3].contains(&new_delta.abs()) {
                continue 'outer;
            }
            last_sign = new_delta.signum()
        }
        safe += 1;
    }

    println!("{}", safe);
}