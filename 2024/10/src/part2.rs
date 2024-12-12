use std::io::stdin;
use std::collections::{VecDeque, HashSet};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut trailheads = Vec::new();

    let map = lines.enumerate().map(|(y, line)| {
        line.bytes().enumerate().map(|(x, c)| {
            let height = c - b'0';
            if height == 0 {
                trailheads.push(Point{x, y});
            }
            height
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    print_map(&map);

    let mut stack: VecDeque<_> = trailheads.into_iter().enumerate().collect();
    // println!("{:?}", stack);

    let mut score = 0;
    // let mut seen_paths = HashSet::new();

    while !stack.is_empty() {
        let (start, p) = stack.pop_back().unwrap();

        let height = map[p.y][p.x];
        if height == 9 {
            score += 1;
            continue;
        }

        let mut next = nexts(&map, p).into_iter().map(|p| (start, p)).collect();
        stack.append(&mut next);
    }

    println!("{}", score);
}

fn nexts(map: &Vec<Vec<u8>>, p: Point) -> Vec<Point> {
    let height = map[p.y][p.x];
    let mut out = Vec::new();

    if p.y + 1 < map.len() && map[p.y+1][p.x] == height + 1 {
        out.push(Point{y: p.y+1, x: p.x});
    }
    if p.y >= 1 && p.y < map.len() && map[p.y-1][p.x] == height + 1 {
        out.push(Point{y: p.y-1, x: p.x});
    }
    if p.x + 1 < map[0].len() && map[p.y][p.x+1] == height + 1 {
        out.push(Point{y: p.y, x: p.x+1});
    }
    if p.x >= 1 && p.x < map[0].len() && map[p.y][p.x-1] == height + 1 {
        out.push(Point{y: p.y, x: p.x-1});
    }

    out
}


#[allow(unused_variables)]
fn print_map(map: &Vec<Vec<u8>>) {
    #[cfg(feature = "verbose")]
    for row in map.iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
