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
        // println!("{}", line);
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

    let mut antinodes = HashSet::new();

    for (_, locs) in antennae.iter() {
        for (i, p) in locs.iter().enumerate() {
            for q in &locs[i+1..] {
                let delta = vec_between(p, q);
                for point in apply_vec_within(p, &delta, width, height) {
                    antinodes.insert(point);
                }
            }
        }
    }

    let antinodes = antinodes.into_iter().filter(|(x, y)| {
        x >= &0 && x < &width && y >= &0 && y < &height
    }).collect::<Vec<_>>();


    #[cfg(feature = "verbose")]
    {
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
    }

    println!("{}", antinodes.len());
}

fn vec_between(p: &Point, q: &Point) -> Vector {
    let x = p.0 - q.0;
    let y = p.1 - q.1;

    (x, y)
}

fn apply_vec_within(p: &Point, v: &Vector, width: isize, height: isize) -> Vec<Point> {
    let mut out = Vec::new();
    let mut q = p.clone();
    while q.0 >= 0 && q.1 >= 0 && q.0 < width && q.1 < height {
        out.push(q);
        q = apply_vec(&q, &neg_vec(v));
    }

    let mut q = p.clone();
    while q.0 >= 0 && q.1 >= 0 && q.0 < width && q.1 < height {
        out.push(q);
        q = apply_vec(&q, v);
    }

    out
}

fn apply_vec(p: &Point, v: &Vector) -> Point {
    (p.0 + v.0, p.1 + v.1)
}

fn neg_vec(v: &Vector) -> Vector {
    (-v.0, -v.1)
}

