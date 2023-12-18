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

    let mut seats = vec![false; 1024];
    for line in lines {
        let seat = parse_seat(&line);
        seats[seat as usize] = true;
    }

    // let mine = 0;
    for i in 1..(seats.len()-1) {
        if !seats[i] && seats[i-1] && seats[i+1] {
            println!("{}", i);
            return;
        }
    }

    println!("seat not found");
}
