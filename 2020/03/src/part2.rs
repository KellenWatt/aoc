use std::io::stdin;

enum Space {
    Tree,
    Empty,
}

fn count_trees(x: usize, y: usize, grid: &Vec<Vec<Space>>) -> u32 {
    let mut count = 0;
    let mut i = 0;
    let mut idx = 0;
    while i < grid.len() - y {
        idx = (idx + x) % grid[0].len();
        i += y;
        
        count += match grid[i][idx] {
            Space::Tree => 1,
            Space::Empty => 0,
        };
    }
    count
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

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut collisions = 1;
    for (x, y) in slopes {
        let count = count_trees(x, y, &grid);
        println!("{}", count);
        collisions *= count;
    }

    println!("{}", collisions);
}
