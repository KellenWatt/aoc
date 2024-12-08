use std::io::stdin;
use std::collections::HashSet;

enum Space {
    Empty,
    Blocked,
}
enum Dir {
    N,
    E,
    S,
    W,
}

struct Guard {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Guard {
    fn step(&mut self, grid: &Vec<Vec<Space>>) -> Option<(usize, usize)> {
        let (x, y) = match self.dir {
            Dir::N => (self.x, self.y.wrapping_sub(1)),
            Dir::E => (self.x + 1, self.y),
            Dir::S => (self.x, self.y + 1),
            Dir::W => (self.x.wrapping_sub(1), self.y),
        };

        match grid.get(y)?.get(x)? {
            Space::Empty => {
                self.x = x;
                self.y = y;
                Some((x, y))
            }
            Space::Blocked => {
                self.dir = match self.dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::S,
                    Dir::S => Dir::W,
                    Dir::W => Dir::N,
                };
                Some((self.x, self.y))
            }
        }
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut pos = (0usize, 0usize);
    let grid = lines.enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, s)| {
            match s {
                '#' => Space::Blocked,
                '.' => Space::Empty,
                '^' => {
                    pos = (x, y);
                    Space::Empty
                }
                _ => unreachable!(),
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut guard = Guard {
        x: pos.0,
        y: pos.1,
        dir: Dir::N,
    };

    let mut seen = HashSet::new();
    seen.insert(pos);

    while let Some(pos) = guard.step(&grid) {
        seen.insert(pos);
    }

    println!("{}", seen.len());
}
