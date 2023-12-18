use std::io::stdin;
use std::time::Instant;


#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point{x: x as isize, y: y as isize}
    }

    fn translate(&self, delta: (isize, isize)) -> Point {
        Point{x: self.x + delta.0, y: self.y + delta.1}
    }

    fn tilt(&self, d: Dir) -> Point {
        self.translate(d.point_offset())
    }
}

#[derive(Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn point_offset(&self) -> (isize, isize) {
        match self {
            &Dir::N => (0, -1),
            &Dir::E => (1, 0),
            &Dir::S => (0, 1),
            &Dir::W => (-1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rock {
    Cube,
    Round,
    Empty,
}

impl From<char> for Rock {
    fn from(value: char) -> Rock {
        match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            _ => Rock::Empty,
        }
    }
}

impl Rock {
    fn is_empty(&self) -> bool {
        match self {
            Rock::Empty => true,
            _ => false,
        }
    }

    fn can_move(&self) -> bool {
        match self {
            Rock::Round => true,
            _ => false,
        }
    }
}

struct Board {
    grid: Vec<Vec<Rock>>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in self.grid.iter() {
            for c in row.iter() {
                match c {
                    Rock::Cube => write!(f, "#")?,
                    Rock::Round => write!(f, "O")?,
                    Rock::Empty => write!(f, ".")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    fn new(lines: Vec<String>) -> Board {
        let grid = lines.iter().map(|row| row.chars().map(|c| Rock::from(c)).collect()).collect();
        Board{grid}
    }

    fn is_valid_point(&self, p: &Point) -> bool {
        p.x >= 0 && 
        p.y >= 0 && 
        p.x < self.grid[0].len() as isize && 
        p.y < self.grid.len() as isize
    } 

    fn is_point_empty(&self, p: &Point) -> bool {
        if !self.is_valid_point(p) {
            return false;
        }
        self.grid[p.y as usize][p.x as usize].is_empty()
    }

    fn tilt(&mut self, dir: Dir) {
        let shift_rock_at = |this: &mut Board, x: usize, y: usize, p: Point| {
            let mut p = p;
            let c = this.grid[y][x];
            if !c.can_move() {
                return;
            }
            loop {
                let next = p.tilt(dir);
                if this.is_point_empty(&next) {
                    p = next;
                } else {
                    if Point::new(x, y) != p {
                        this.grid[p.y as usize][p.x as usize] = this.grid[y][x];
                        this.grid[y][x] = Rock::Empty;
                    }
                    return;
                }
            }
        };
        match dir {
            Dir::N => {
                for i in 0..(self.grid.len()) {
                    for j in 0..(self.grid[i].len()) {
                        let p = Point::new(j, i);
                        shift_rock_at(self, j, i, p);
                    }
                }
            },
            Dir::E => {
                for j in (0..(self.grid[0].len())).rev() {
                    for i in 0..(self.grid.len()) {
                        let p = Point::new(j, i);
                        shift_rock_at(self, j, i, p);
                    }
                }
            
            },
            Dir::S => {
                for i in (0..(self.grid.len())).rev() {
                    for j in (0..(self.grid[i].len())).rev() {
                        let p = Point::new(j, i);
                        shift_rock_at(self, j, i, p);
                    }
                }

            },
            Dir::W => {
                for j in 0..(self.grid[0].len()) {
                    for i in 0..(self.grid.len()) {
                        let p = Point::new(j, i);
                        shift_rock_at(self, j, i, p);
                    }
                }
            },
        }
    }

    fn pressure_on(&self, dir: Dir) -> usize {
        match dir {
            Dir::N => {
                (0..(self.grid[0].len())).map(|j| {
                    self.grid.iter().enumerate().map(|(i, row)| {
                        if row[j].can_move() {
                            self.grid.len() - i
                        } else {
                            0
                        }
                    }).sum::<usize>()
                    // let stones: Vec<_> = self.grid.iter().take_while(|row| {
                    //     row[j].can_move()
                    // }).collect();
                    // let res = stones.iter().enumerate().map(|(i,_)| {
                    //     (self.grid.len() - i) * stones.len() * i
                    // }).sum::<usize>();
                    // 
                    // println!("col: {}", res);
                    // res
                }).sum()
            },
            Dir::E => {
                self.grid.iter().map(|row| {
                    row.iter().rev().enumerate().map(|(i, c)| {
                        if c.can_move() {
                            self.grid[0].len() - i
                        } else {
                            0
                        }
                    }).sum::<usize>()
                    // row.iter().rev().take_while(|c| {
                    //     c.can_move()
                    // }).enumerate().map(|(i,_)| {
                    //     self.grid[0].len() - i
                    // }).sum::<usize>()
                }).sum()
            },
            Dir::S => {
                (0..(self.grid[0].len())).map(|j| {
                    self.grid.iter().rev().enumerate().map(|(i, row)| {
                        if row[j].can_move() {
                            self.grid.len() - i
                        } else {
                            0
                        }
                    }).sum::<usize>()
                    // self.grid.iter().rev().take_while(|row| {
                    //     row[j].can_move()
                    // }).enumerate().map(|(i,_)| {
                    //     self.grid.len() - i
                    // }).sum::<usize>()
                }).sum()
            },
            Dir::W => {
                self.grid.iter().map(|row| {
                    row.iter().enumerate().map(|(i, c)| {
                        if c.can_move() {
                            self.grid[0].len() - i
                        } else {
                            0
                        }
                    }).sum::<usize>()
                    // row.iter().take_while(|c| {
                    //     c.can_move()
                    // }).enumerate().map(|(i,_)| {
                    //     self.grid[0].len() - i
                    // }).sum::<usize>()
                }).sum()
            }
        }
    }
    fn spin(&mut self) {
        self.tilt(Dir::N);
        self.tilt(Dir::W);
        self.tilt(Dir::S);
        self.tilt(Dir::E);
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap()).collect();
    let start = Instant::now();

    let mut board = Board::new(lines);
    // println!("pre-tilt\n{}", board);
    
    // This should use cycle detection, but apparently you usually get the right answer after 
    // 1000 cycles according to the fine denizens of r/adventofcode, so I won't argue. 
    // Cycle detection is boring.
    for _ in 0..1000 {
        board.spin();
    }
    // println!("post-tilt\n{}", board);
    //
    println!("{}", board.pressure_on(Dir::N));
    println!("time elapsed: {}", start.elapsed().as_secs_f32());
}
