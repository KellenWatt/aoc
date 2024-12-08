use std::io::stdin;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy)]
enum Space {
    Empty,
    Blocked,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    let options = Arc::new(AtomicUsize::new(0));


    std::thread::scope(|s| {
        for x in 0..grid[0].len() {
            let mut grid = grid.clone();
            let options = options.clone();
            s.spawn(move || {
                let mut last_pos: Option<(usize, usize)> = None;
                for y in 0..grid.len() {
                    if let Some(last) = last_pos {
                        grid[last.1][last.0] = Space::Empty;
                    }
                    
                    match grid[y][x] {
                        Space::Empty => {
                            grid[y][x] = Space::Blocked;
                            last_pos = Some((x, y));
                        }
                        Space::Blocked => continue,
                    }
                    let mut guard = Guard {
                        x: pos.0,
                        y: pos.1,
                        dir: Dir::N,
                    };
                    let mut seen = HashSet::new();
                    seen.insert(guard.clone());

                    loop {
                        match guard.step(&grid) {
                            Some(_) => {
                                if seen.contains(&guard) {
                                    options.fetch_add(1, Ordering::Relaxed);
                                    break;
                                }
                                seen.insert(guard.clone());
                            }
                            None => {
                                break;
                            }
                        }
                    }

                }
            });
        }
    });

    // let mut seen = HashSet::new();
    // seen.insert(pos);
    // 
    // while let Some(pos) = guard.step(&grid) {
    //     seen.insert(pos);
    // }

    println!("{}", options.load(Ordering::Acquire));
}

