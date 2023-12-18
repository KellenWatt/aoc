use std::io::stdin;
use std::sync::{OnceLock};
use std::time::Instant;
use std::collections::VecDeque;

static GRID_HEIGHT: OnceLock<usize> = OnceLock::new();
static GRID_WIDTH: OnceLock<usize> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,E,S,W,
}

struct Beam {
    x: usize,
    y: usize,
    dir: Dir,
}

impl std::fmt::Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let dir = match self.dir {
            Dir::N => '^',
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<',
        };
        write!(f, "({}, {}) {}", self.x, self.y, dir)
    }
}

impl Beam {
    fn new(x: usize, y: usize, dir: Dir) -> Beam {
        Beam{x, y, dir}
    }

    fn step(&mut self) -> bool {
        match self.dir {
            Dir::N => {
                if self.y == 0 {
                    return false;
                }
                self.y -= 1;
            },
            Dir::S => {
                if self.y + 1 == *GRID_HEIGHT.get().unwrap() {
                    return false;
                }
                self.y += 1;
            },
            Dir::W => {
                if self.x == 0 {
                    return false;
                }
                self.x -= 1;
            },
            Dir::E => {
                if self.x + 1 == *GRID_WIDTH.get().unwrap() {
                    return false;
                }
                self.x += 1;
            }
        }
        true
    }

    fn split(&mut self) -> Beam {
        match self.dir {
            Dir::N | Dir::S => {
                self.dir = Dir::W;
                Beam{x: self.x, y: self.y, dir: Dir::E}
            },
            Dir::E | Dir::W => {
                self.dir = Dir::S;
                Beam{x: self.x, y: self.y, dir: Dir::N}
            }
        }
    }

    fn redirect(&mut self, dir: Dir) {
        self.dir = dir;
    }

    // does not currently account for cycles. Hopefully won't need to.
    fn launch(self, mut board: Board) -> Board {
        // println!("launching new beam");
        let mut todo = VecDeque::new();
        todo.push_back(self);
        while todo.len() > 0 {
            let mut beam = todo.pop_front().unwrap();
            loop {
                unsafe {
                    // could go elsewhere, but this make energizing the very first step more implicit
                    let cell = board.0.get_unchecked_mut(beam.y).get_unchecked_mut(beam.x);
                    if cell.light_entries.contains(&beam.dir) {
                        break;
                    } else {
                        cell.light_entries.push(beam.dir);
                    }
                    cell.energized = true;
                }
                let m = unsafe {
                    board.0.get_unchecked(beam.y).get_unchecked(beam.x).mirror
                };
                match m {
                    Mirror::Pos => {
                        let new_dir = match beam.dir {
                            Dir::N => Dir::E,
                            Dir::E => Dir::N,
                            Dir::W => Dir::S,
                            Dir::S => Dir::W,
                        };
                        beam.redirect(new_dir);
                    },
                    Mirror::Neg => {
                        let new_dir = match beam.dir {
                            Dir::N => Dir::W,
                            Dir::W => Dir::N,
                            Dir::S => Dir::E,
                            Dir::E => Dir::S,
                        };
                        beam.redirect(new_dir);
                    },
                    Mirror::Zero if beam.dir == Dir::N || beam.dir == Dir::S => {
                        let beam2 = beam.split();
                        todo.push_back(beam2);
                    },
                    Mirror::Inf if beam.dir == Dir::E || beam.dir == Dir::W => {
                        let beam2 = beam.split();
                        todo.push_back(beam2);
                    },
                    _ => {
                        // whole lotta nuthin'
                    }
                } // match m
                let res = beam.step();
                if !res {break;}
            }
        }
        board
    }
}

#[derive(Clone, Copy)]
enum Mirror {
    Pos,    // positive slope
    Neg,    // negative slope
    Zero,   // zero slope (hirzontal)
    Inf,    // Infinite slope (vertical)
    Empty,  // Not actually a mirror
}

#[derive(Clone)]
struct Cell {
    mirror: Mirror,
    energized: bool,
    light_entries: Vec<Dir>,
}

#[derive(Clone)]
struct Board(Vec<Vec<Cell>>);

impl Board {
    fn new(lines: Vec<String>) -> Board {
        let grid: Vec<Vec<Cell>> = lines.iter().map(|line| {
            line.chars().map(|c| {
                let mirror = match c {
                    '.' => Mirror::Empty,
                    '/' => Mirror::Pos,
                    '\\' => Mirror::Neg,
                    '-' => Mirror::Zero,
                    '|' => Mirror::Inf,
                    _ => panic!("unrecognized mirror '{}'", c)
                };
                Cell{mirror, energized: false, light_entries: vec![]}
            }).collect()
        }).collect();
        let _ = GRID_WIDTH.set(grid[0].len());
        let _ = GRID_HEIGHT.set(grid.len());
        Board(grid)
    }

    fn launch_beam(self, beam: Beam) -> usize {
        // println!("launching {}", beam);
        // let handle = spawn(move || {
            // let board = self;
            // let seed = board.clone();
            let board = beam.launch(self);
            
            // while Arc::strong_count(&board) > 1 {[> no-op <]}

            let mut energized: usize = 0;
            for row in board.0.iter() {
                for cell in row.iter() {
                    if cell.energized {
                        energized += 1;
                    }
                }
            }

            energized
        // });
        // handle
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect();
    let board = Board::new(lines);
    let mut results = vec![];


    let start = Instant::now();
    for i in 0..(*GRID_WIDTH.get().unwrap()) {
        let start_board = board.clone();
        let beam = Beam::new(i, 0, Dir::S);
        let res = start_board.launch_beam(beam);
        results.push(res);
        
        let start_board = board.clone();
        let beam = Beam::new(i, start_board.0.len() - 1, Dir::N);
        let res = start_board.launch_beam(beam);
        results.push(res);
    }
    
    for i in 0..(*GRID_HEIGHT.get().unwrap()) {
        let start_board = board.clone();
        let beam = Beam::new(0, i, Dir::E);
        let res = start_board.launch_beam(beam);
        results.push(res);
        
        let start_board = board.clone();
        let beam = Beam::new(start_board.0[0].len() - 1, i, Dir::W);
        let res = start_board.launch_beam(beam);
        results.push(res);
    }


    let mut max = 0;
    for res in results {
        if res > max {
            max = res;
        }
    }

    println!("{}", max);
    println!("elapsed: {}", start.elapsed().as_secs_f32());
    // A grim reminder to not abuse threads. This used ~100000 threads, and ran for ~24s. 
    // Without using threads, it runs in ~185ms...
    // unsafe {
    //     println!("total threads: {}", TOTAL_THREADS.read().unwrap());
    // }
}
