use std::io::stdin;
use std::collections::{VecDeque, HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N, E, S, W,
}

enum Cell {
    Tree,
    Path,
    Slope(Dir),
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Tree,
            '.' => Cell::Path,
            '^' => Cell::Slope(Dir::N),
            '>' => Cell::Slope(Dir::E),
            'v' => Cell::Slope(Dir::S),
            '<' => Cell::Slope(Dir::W),
            _ => panic!("unrecognized input: '{}'", c)
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            &Cell::Tree => '#',
            &Cell::Path => '.',
            &Cell::Slope(d) => {
                match d {
                    Dir::N => '^',
                    Dir::E => '>',
                    Dir::S => 'v',
                    Dir::W => '<',
                }
            }
        })
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn shift(&self, d: Dir, max_width: usize, max_height: usize) -> Option<Point> {
        match d {
            Dir::N => (self.y > 0).then(|| Point{x: self.x, y: self.y-1}),
            Dir::E => (self.x < max_width - 1).then(|| Point{x: self.x+1, y: self.y}),
            Dir::S => (self.y < max_height - 1).then(|| Point{x: self.x, y: self.y+1}),
            Dir::W => (self.x > 0).then(|| Point{x: self.x-1, y: self.y}),
        }
    }

    fn neighbors(&self, forest: &Forest) -> Vec<Point> {
        if let Cell::Slope(d) = forest[self] {
            return if let Some(p) = self.shift(d, forest.width, forest.height) {
                vec![p]
            } else {
                vec![]
            }
        }
        [Dir::N, Dir::E, Dir::S, Dir::W].iter()
            .filter_map(|d| self.shift(*d, forest.width, forest.height))
            .filter_map(|p| {
                match forest[&p] {
                    Cell::Tree => None,
                    _ => Some(p)
                }
            }).collect()
    }

    fn distance(&self, other: &Point) -> usize {
        let dx = if self.x < other.x {
            other.x - self.x
        } else {
            self.x  - other.x
        };
        let dy = if self.y < other.y {
            other.y - self.y
        } else {
            self.y - other.y
        };
        dx + dy
    }
}

struct Forest {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn new(grid: Vec<Vec<Cell>>) -> Forest {
        let width = grid[0].len();
        let height= grid.len();
        Forest{grid, width, height}
    }

    fn start(&self) -> Point {
        for (x, c) in self.grid[0].iter().enumerate() {
            if let Cell::Path = c {
                return Point{x, y: 0};
            }
        }
        unreachable!();
    }

    fn end(&self) -> Point {
        for (x, c) in self.grid[self.grid.len() - 1].iter().enumerate() {
            if let Cell::Path = c {
                return Point{x, y: self.grid.len() - 1};
            }
        }
        unreachable!();
    }
}

impl std::ops::Index<&Point> for Forest {
    type Output = Cell;
    fn index(&self, p: &Point) -> &Cell {
        &self.grid[p.y][p.x]
    }
}

struct State<'a> {
    prev: Point,
    pos: Point,
    dist: usize,
    goal: &'a Point,
}
impl<'a> Clone for State<'a> {
    fn clone(&self) -> State<'a> {
        State{prev: self.prev, pos: self.pos, dist: self.dist, goal: self.goal}
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &State) -> bool {
        self.prev == other.prev &&
        self.pos == other.pos
    }
}
impl<'a> Eq for State<'a> {}

impl<'a> std::hash::Hash for State<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.prev.hash(state);
        self.pos.hash(state);
    }
}

impl<'a> State<'a> {
    fn new(pos: Point, goal: &Point) -> State {
        State{prev: pos, pos, dist: 0, goal}
    }
    fn nexts(self, forest: &'a Forest) -> Vec<State> {
        self.pos.neighbors(forest).into_iter().filter_map(|pos| {
            (pos != self.prev).then_some(State{prev: self.pos, pos, dist: self.dist+1, goal: self.goal})
        }).collect()
    }

    fn at_goal(&self) -> bool {
        self.pos == *self.goal
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let forest = lines.map(|line| {
        line.chars().map(|c| {
            Cell::from(c)
        }).collect()
    }).collect();
    let forest = Forest::new(forest);
    let start = forest.start();
    let end = forest.end();

    let mut longest = HashMap::new();

    let mut seen: HashSet<State> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(State::new(start, &end));

    let mut count = 0;
    while let Some(state) = queue.pop_front() {
        // if seen.contains(&state) {
        //     continue;
        // }
        // seen.insert(state.clone());
        if &state.dist > longest.get(&state.pos).unwrap_or(&0) {
            longest.insert(state.pos, state.dist);
        }

        count += 1;
        if count % 10 == 0 {
            print!("\rstates observed: {}", count);
        }
        for s in state.nexts(&forest) {
            queue.push_back(s);
        }
    }

    println!("\rstates observed: {}", count);
    println!("longest path: {}", longest.get(&end).unwrap());

}
