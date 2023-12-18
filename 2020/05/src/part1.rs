use std::io::stdin;

fn parse_seat(seat: &str) -> u16 {
    seat.chars().fold(0, |acc, c| {
        (acc << 1) + match c {
            'F'|'L' => 0,
            'B'|'R' => 1,
            _ => panic!("invalid character")
        }
    })
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut max = 0;
    for line in lines {
        let seat = parse_seat(&line);
        if seat > max {
            max = seat;
        }
    }

    println!("{}", max);
}
