// use std::io::stdin;

fn main() {
    // let lines = stdin().lines().map(|l| l.unwrap());
    // let (start, end) = lines.next().unwrap().split_once("-").unwrap();
    // let range = (start.parse::<u32>().unwrap())..=(end.parse::<u32>().unwrap());
    let start = 146810;
    let end = 612564;

    let mut count = 0;

    for d1 in 1..=6 {
        let d = d1;
        for d2 in d1..=9 {
            let d = d * 10 + d2;
            if d > 61 || d < 14 {continue;}
            for d3 in d2..=9 {
                let d = d * 10 + d3;
                if d > 612 || d < 146 {continue;}
                for d4 in d3..=9 {
                    let d = d * 10 + d4;
                    if d > 6125 || d < 1468 {continue;}
                    for d5 in d4..=9 {
                        let d = d * 10 + d5;
                        if d > 61256 || d < 14681 {continue;}
                        for d6 in d5..=9 {
                            let d = d * 10 + d6;
                            if d > 612564 || d < 146810 {continue;}
                            if d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6 { 
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
