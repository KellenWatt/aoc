use std::io::stdin;

enum Space {
    Tree,
    Empty,
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let grid: Vec<Vec<Space>> = lines.map(|line| {
        line.chars().map(|c| {
            match c {
                '#' => Space::Tree,
                _ => Space::Empty,
            }
        }).collect()
    }).collect();

    let mut x = 0;
    let mut collisions = 0u32;
    for row in grid.iter().skip(1) {
        x = (x + 3) % row.len();
        collisions += match row[x] {
            Space::Tree => 1,
            _ => 0,
        };
    }

    println!("{}", collisions);
}
