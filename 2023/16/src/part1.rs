use std::io::stdin;
use std::sync::{OnceLock, Arc, RwLock};
use std::thread::{spawn};
use std::time::Instant;

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

impl Beam {
    fn new(x: usize, y: usize, dir: Dir) -> Beam {
        Beam{x, y, dir}
    }
    fn initial() -> Beam {
        Beam{x: 0, y: 0, dir: Dir::E}
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

    fn split(self) -> (Beam, Beam) {
        match self.dir {
            Dir::N | Dir::S => {
                (Beam{x: self.x, y: self.y, dir: Dir::E}, Beam{x: self.x, y: self.y, dir: Dir::W})
            },
            Dir::E | Dir::W => {
                (Beam{x: self.x, y: self.y, dir: Dir::N}, Beam{x: self.x, y: self.y, dir: Dir::S})
            }
        }
    }

    fn redirect(&mut self, dir: Dir) {
        self.dir = dir;
    }

    // does not currently account for cycles. Hopefully won't need to.
    fn launch(mut self, board: Arc<RwLock<Board>>) {
        // println!("launching new beam");
        let _handle = spawn(move || {
            loop {
                unsafe {
                    // could go elsewhere, but this make energizing the very first step more implicit
                    let mut b = board.write().unwrap();
                    let cell = b.0.get_unchecked_mut(self.y).get_unchecked_mut(self.x);
                    

                    if cell.light_entries.contains(&self.dir) {
                        break;
                    } else {
                        cell.light_entries.push(self.dir);
                    }
                    
                    cell.energized = true;
                }

                let m: Mirror = unsafe {
                    // may yell about temp value?
                    board.read().unwrap().0.get_unchecked(self.y).get_unchecked(self.x).mirror
                    // try to release the lock as soon as possible
                };

                match m {
                    Mirror::Pos => {
                        let new_dir = match self.dir {
                            Dir::N => Dir::E,
                            Dir::E => Dir::N,
                            Dir::W => Dir::S,
                            Dir::S => Dir::W,
                        };
                        self.redirect(new_dir);
                    },
                    Mirror::Neg => {
                        let new_dir = match self.dir {
                            Dir::N => Dir::W,
                            Dir::W => Dir::N,
                            Dir::S => Dir::E,
                            Dir::E => Dir::S,
                        };
                        self.redirect(new_dir);
                    },
                    Mirror::Zero if self.dir == Dir::N || self.dir == Dir::S => {
                        let (beam1, beam2) = self.split();
                        // let c1 = board.clone();
                        let b2 = board.clone();
                        beam1.launch(board); // try to not create new counts unless necessary
                        beam2.launch(b2);
                        break;
                    },
                    Mirror::Inf if self.dir == Dir::E || self.dir == Dir::W => {
                        let (beam1, beam2) = self.split();
                        // let c1 = board.clone();
                        let b2 = board.clone();
                        beam1.launch(board);
                        beam2.launch(b2);
                        break;
                    },
                    _ => {
                        // whole lotta nuthin'
                    }
                } // match m
                let res = self.step();
                if !res {break;}
            } // loop
        });
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
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect();
    let board = Board::new(lines);
    let board = Arc::new(RwLock::new(board));

    let start = Instant::now();
    let seed = board.clone();
    Beam::initial().launch(seed); 

    // alternative to tracking a potentially infinite number of threads.
    while Arc::strong_count(&board) > 1 {/* no-op */}

    let mut energized: usize = 0;
    for row in board.read().unwrap().0.iter() {
        for cell in row.iter() {
            if cell.energized {
                // print!("#");
                energized += 1;
            } else {
                // print!(".")
            }
        }
        // println!("");
    }

    println!("{}", energized);
    println!("elapsed: {}", start.elapsed().as_secs_f32());
}
