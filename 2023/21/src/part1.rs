use std::io::stdin;
use std::collections::{HashSet};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Stone,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self, g: &Garden) -> Vec<Point> {
        let mut out = vec![];
        if self.x > 0 {
            out.push(Point{x: self.x-1, y: self.y});
        }
        if self.y > 0 {
            out.push(Point{x: self.x, y: self.y-1});
        }
        if self.x < g.width-1 {
            out.push(Point{x: self.x+1, y: self.y});
        }
        if self.y < g.height-1 {
            out.push(Point{x: self.x, y: self.y+1});
        }
        out.iter().filter_map(|p| (g[p] != Cell::Stone).then_some(*p)).collect()
    }
}


struct Garden {
    space: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(space: Vec<Vec<Cell>>) -> Garden {
        let height = space.len();
        let width = space[0].len();
        Garden{space, width, height}
    }
}

impl std::ops::Index<&Point> for Garden {
    type Output = Cell;
    fn index(&self, p: &Point) -> &Cell {
        &self.space[p.y][p.x]
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut start_point = Point{x: 0, y: 0};
    let grid = lines.enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => Cell::Empty,
                '#' => Cell::Stone,
                'S' => {
                    start_point = Point{x, y};
                    Cell::Empty
                },
                _ => panic!("unexpected char '{}' at ({},{})", c, x, y)
            }
        }).collect()
    }).collect();
    let garden = Garden::new(grid);

    let start = Instant::now();

    let mut current = HashSet::new();
    current.insert(start_point);
    for i in 0..64 {
        let mut next = HashSet::new();
        for p in current {
            for n in p.neighbors(&garden) {
                next.insert(n);
            }
        }
        current = next;
    }

    println!("{}", current.len());
    println!("elapsed: {}s", start.elapsed().as_secs_f32());
}
