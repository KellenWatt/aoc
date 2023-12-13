use std::io::stdin;
use std::sync::OnceLock;
use std::ops::Sub;

static GRID_WIDTH: OnceLock<isize> = OnceLock::new();
static GRID_HEIGHT: OnceLock<isize> = OnceLock::new();



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Position(isize, isize);

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        let mut neigh = vec![];
        // NEWS
        if self.0 > 0 {
            neigh.push(Position(self.0 - 1, self.1));
        }
        if self.0 + 1 < *GRID_WIDTH.get().unwrap() {
            neigh.push(Position(self.0 + 1, self.1));
        }
        if self.1 > 0 {
            neigh.push(Position(self.0, self.1 - 1));
        }
        if self.1 + 1 < *GRID_HEIGHT.get().unwrap() {
            neigh.push(Position(self.0, self.1 + 1));
        }
        neigh
    }

    fn shift(&self, x: isize, y: isize) -> Option<Position> {
        let new_x = self.0 + x;
        let new_y = self.1 + y;
        if !(0..(*GRID_WIDTH.get().unwrap())).contains(&new_x) && 
            (0..(*GRID_HEIGHT.get().unwrap())).contains(&new_y) {
            return None;
        }
        Some(Position(new_x, new_y))
    }
} 

impl PartialEq<(isize, isize)> for Position {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Clone, Copy, Debug)]
enum Pipe {
    Vert,
    Hor,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Pipe {
        match value {
            '|' => Pipe::Vert,
            '-' => Pipe::Hor,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid input: {}", value),
        }
    }
}

impl Pipe {
    fn is_start(&self) -> bool {
        match self {
            &Pipe::Start => true,
            _ => false, 
        }
    }
    fn is_ground(&self) -> bool {
        match self {
            &Pipe::Ground => true,
            _ => false,
        }        
    }
    fn is_angle(&self) -> bool {
        match self {
            &Pipe::NE | &Pipe::NW | &Pipe::SE | &Pipe::SW => true,
            _ => false,
        }
    }
    fn is_vert(&self) -> bool {
        match self {
            &Pipe::Vert => true,
            _ => false,
        }
    }
    fn verticality(&self) -> i32 {
        match self {
            &Pipe::Vert => 2,
            &Pipe::NE | &Pipe::SW => 1,
            &Pipe::NW | &Pipe::SE => -1,
            &Pipe::Start => {panic!("verticality of start is variable")},
            _ => 0,
        }
    }

    fn connects_to(&self, other: &Pipe, offset: Position) -> bool {
        // current pipe is "center", other pipe compatible if any ends connect
        match self {
            &Pipe::Vert => {
                match other {
                    &Pipe::Vert | &Pipe::SW | &Pipe::SE if offset == (0, -1) => true,
                    &Pipe::Vert | &Pipe::NE | &Pipe::NW if offset == (0, 1) => true,
                    _ => false,
                }
            },
    		&Pipe::Hor => {
                match other {
                    &Pipe::Hor | &Pipe::NW | &Pipe::SW if offset == (1, 0) => true,
                    &Pipe::Hor | &Pipe::NE | &Pipe::SE if offset == (-1, 0) => true,
                    _ => false,
                }
            },
            &Pipe::NE => {
                match other {
                    &Pipe::Hor | &Pipe::NW | &Pipe::SW if offset == (1, 0) => true,
                    &Pipe::Vert | &Pipe::SE | &Pipe::SW if offset == (0, -1) => true,
                    _ => false,
                }
            },
    		&Pipe::NW => {
                match other {
                    &Pipe::Hor | &Pipe::NE | &Pipe::SE if offset == (-1, 0) => true,
                    &Pipe::Vert | &Pipe::SE | &Pipe::SW if offset == (0, -1) => true,
                    _ => false,
                }
            },
    		&Pipe::SW => {
                match other {
                    &Pipe::Hor | &Pipe::NE | &Pipe::SE if offset == (-1, 0) => true,
                    &Pipe::Vert | &Pipe::NE | &Pipe::NW if offset == (0, 1) => true,
                    _ => false,
                }
            },
    		&Pipe::SE => {
                match other {
                    &Pipe::Hor | &Pipe::NW | &Pipe::SW if offset == (1, 0) => true,
                    &Pipe::Vert | &Pipe::NE | &Pipe::NW if offset == (0, 1) => true,
                    _ => false,
                }
            },
    		&Pipe::Start => {
                match other {
                    &Pipe::Hor | &Pipe::SE | &Pipe::NE if offset == (-1, 0) => true,
                    &Pipe::Vert | &Pipe::SE | &Pipe::SW if offset == (0, -1) => true,
                    &Pipe::Hor | &Pipe::NW | &Pipe::SW if offset == (1, 0) => true,
                    &Pipe::Vert | &Pipe::NE | &Pipe::NW if offset == (0, 1) => true,
                    _ => false,
                }
            },
    		&Pipe::Ground => false,
        }

    }
}


struct PipeLine {
    start: Position,
    prev: Option<Position>,
    pos: Position,
    grid: Vec<Vec<Pipe>>,
}

impl PipeLine {
    // fn new(grid: Vec<Vec<Pipe>>) -> PipeLine {
    //     PipeLine::from_start(grid, Position(0, 0))
    // }

    fn from_start(grid: Vec<Vec<Pipe>>, pos: Position) -> PipeLine {
        PipeLine{pos, grid, prev: None, start: pos}
    }

    fn current_pipe(&self) -> &Pipe {
        self.pipe_at(self.pos)
    }

    fn pipe_at(&self, pos: Position) -> &Pipe {
        &self.grid[pos.1 as usize][pos.0 as usize]
    }

    fn current_connects_to(&self, pos: Position) -> bool {
        self.current_pipe().connects_to(self.pipe_at(pos), pos - self.pos)
    }

    fn at_start(&self) -> bool {
        self.pos == self.start
    }

    // fn restart_at(&mut self, p: Position) -> &PipeLine{
    //     self.pos = p;
    //     self.prev = None;
    //     self
    // }
    fn force_start_pipe(&mut self) {
        // Convert the start point (or current point) to a pipe
        // if already a normal pipe, no-op
        if !self.current_pipe().is_start() {
            return
        }

        let offsets: Vec<Position> = self.pos.neighbors().iter().filter(|n| {
            self.current_connects_to(**n)
        }).map(|p| *p - self.pos).collect();
        // order: NEWS
        // guaranteed to have 1 around the start
        let new_pipe = match (offsets[0], offsets[1]) {
        /*NE*/(Position(0, -1), Position(1, 0)) => Pipe::NE,
        /*NW*/(Position(0, -1), Position(-1, 0)) => Pipe::NW,
        /*NS*/(Position(0, -1), Position(0, 1)) => Pipe::Vert,
        /*EW*/(Position(1, 0), Position(-1, 0)) => Pipe::Hor,
        /*ES*/(Position(1, 0), Position(0, 1)) => Pipe::SE,
        /*WS*/(Position(-1, 0), Position(0, 1)) => Pipe::SW,
              _ => panic!("should match one of the patterns"),
        };

        self.grid[self.pos.1 as usize][self.pos.0 as usize] = new_pipe;
    }
}

impl Iterator for PipeLine {
    type Item = (Position, Pipe);

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{:?} {:?}", self.prev, self.pos);
        if self.prev.is_none() {
            for p in self.pos.neighbors() {
                if self.current_connects_to(p) {
                    let tmp = self.pos;
                    self.pos = p;
                    self.prev = Some(tmp);

                    return Some((self.pos, *self.current_pipe()));
                }    
            }
            // guaranteed to return a pipe, by spec, but just to be safe
            return None;
        }

        let prev = self.prev.as_ref().unwrap();

        let next_pos = match self.current_pipe() {
            &Pipe::Vert => {
                // println!("Vertical Move");
                let dy = self.pos.1 - prev.1;
                self.pos.shift(0, dy)?
            },
    		&Pipe::Hor => {
                // println!("Horizontal Move");
                let dx = self.pos.0 - prev.0;
                self.pos.shift(dx, 0)?
            },
            &Pipe::NE => {
                if self.pos.0 == prev.0 { // same Col
                    // println!("Moving East");
                    self.pos.shift(1, 0)?
                } else {
                    // println!("Moving North");
                    self.pos.shift(0, -1)?
                }
            },
    		&Pipe::NW => {
                if self.pos.0 == prev.0 { // same Col
                    // println!("Moving West");
                    self.pos.shift(-1, 0)?
                } else {
                    // println!("Moving North");
                    self.pos.shift(0, -1)?
                }
            },
    		&Pipe::SW => {
                if self.pos.0 == prev.0 { // same Col
                    // println!("Moving West");
                    self.pos.shift(-1, 0)?
                } else {
                    // println!("Moving South"); 
                    self.pos.shift(0, 1)?
                }
            },
    		&Pipe::SE => {
                if self.pos.0 == prev.0 { // same Col
                    // println!("Moving East");
                    self.pos.shift(1, 0)?
                } else {
                    // println!("Moving South");
                    self.pos.shift(0, 1)?
                }
            },
            _ => {return None;},
        };
        
        let tmp = self.pos;
        self.pos = next_pos;
        self.prev = Some(tmp);

        if !self.at_start() {
            Some((self.pos, *self.current_pipe()))
        } else {
            None
        }
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut start = Position(-1, -1);
    let grid: Vec<Vec<Pipe>> = lines.enumerate().map(|(i, line)| {
        // println!("{}", line);
        let _ = GRID_WIDTH.set(line.len() as isize);
        line.chars().enumerate().map(|(j, c)| {
            if c == 'S' {
                start = Position(j as isize, i as isize);
            }
            Pipe::from(c)
        }).collect()
    }).collect();
    let _ = GRID_HEIGHT.set(grid.len() as isize);

    let mut pipes = PipeLine::from_start(grid.clone(), start);
    // println!("interpretting start pipe");
    pipes.force_start_pipe();
    // println!("pipe is {:?}", pipes.current_pipe());

    let mut main_circuit = vec![vec![Pipe::Ground; *GRID_WIDTH.get().unwrap() as usize]; *GRID_HEIGHT.get().unwrap() as usize];
    
    main_circuit[pipes.start.1 as usize][pipes.start.0 as usize] = *pipes.current_pipe();
    // println!("copying mapped pipes");
    for (pos, pipe) in pipes {
        main_circuit[pos.1 as usize][pos.0 as usize] = pipe;
    }
    // println!("done");

    let mut total = 0;
    let mut points = vec![];
    for (j, row) in main_circuit.iter().enumerate() {
        let len = row.len();
        let mut i = 0;
        let mut inside = false;
        while i < len {
            if row[i].is_vert() {
                inside = !inside;
            } else if row[i].is_angle() {
                let start = row[i]; // hack
                i += 1;
                while !row[i].is_ground() && !row[i].is_angle() {
                    i += 1;
                }
                // println!("{:?} {:?}", start, row[i]);
                inside = match (start, row[i]) {
                    (Pipe::NE, Pipe::SW) | (Pipe::SW, Pipe::NE) => !inside,
                    (Pipe::NE, Pipe::NW) | (Pipe::NW, Pipe::NE) => inside,
                    (Pipe::NW, Pipe::SE) | (Pipe::SE, Pipe::NW) => !inside,
                    (Pipe::SW, Pipe::SE) | (Pipe::SE, Pipe::SW) => inside,
                    _ => panic!("unforseen angle")
                }
            } else if row[i].is_ground() && inside {
                total += 1;
                points.push((i, j));
            }
                
            i += 1;
        }
    //     let mut verticality = 0;
    //     for (i, cell) in row.iter().enumerate() {
    //         // color if total row verticality = 2 mod 4
    //         verticality += cell.verticality();
    //         print!("{: >2}", verticality);
    //         if verticality % 4 == 2 && cell.is_ground() {
    //             total += 1;
    //             points.push((i, j));
    //         }
    //     }
    //     println!("");
    }
    // let real_points = points.iter().filter(|(x, y)| {
    //     grid[*y as usize][*x as usize].is_ground()
    // });
    // for p in points {
    //     println!("{:?}", p);
    // }
    

    println!("{}", total);
    // println!("{}", real_points.count());
}
