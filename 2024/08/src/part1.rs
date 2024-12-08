use std::io::stdin;
use std::collections::{HashMap, HashSet};


type Point = (isize, isize);
type Vector = (isize, isize);

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut antennae = HashMap::new();

    let mut height = 0;
    let mut width = 0;
    for (y, line) in lines.enumerate() {
        println!("{}", line);
        height = y;
        for (x, c) in line.chars().enumerate() {
            width = x;
            if c == '.' {
                continue;
            }
            antennae.entry(c).or_insert_with(|| vec![]).push((x as isize, y as isize));
        }
    }

    let width = width as isize + 1;
    let height = height as isize + 1;

    println!("{:?}", antennae);

    let mut antinodes = HashSet::new();

    for (_, locs) in antennae.iter() {
        for (i, p) in locs.iter().enumerate() {
            for q in &locs[i+1..] {
                let delta = vec_between(p, q);
                let (a, b) = apply_bi_vec(p, &delta);
               
                if &a != q {
                    antinodes.insert(a);
                } else {
                    antinodes.insert(b);
                }

                let (a, b) = apply_bi_vec(q, &delta);
                if &a != p {
                    antinodes.insert(a);
                } else {
                    antinodes.insert(b);
                }
            }
        }
    }

    let antinodes = antinodes.into_iter().filter(|(x, y)| {
        x >= &0 && x < &width && y >= &0 && y < &height
    }).collect::<Vec<_>>();

    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for node in antinodes.iter() {
        grid[node.1 as usize][node.0 as usize] = '#';
    }

    println!("{:?}", antinodes);
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    println!("{}", antinodes.len());
}

fn vec_between(p: &Point, q: &Point) -> Vector {
    let x = (p.0 - q.0);
    let y = (p.1 - q.1);

    (x, y)
}

fn apply_bi_vec(p: &Point, v: &Vector) -> (Point, Point) {
    let a = (p.0 - v.0, p.1 - v.1);
    let b = (p.0 + v.0, p.1 + v.1);

    (a, b)
}
