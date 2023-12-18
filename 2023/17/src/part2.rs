use std::io::stdin;
use std::cmp::Ordering;
// use std::sync::OnceLock;
use std::collections::{BinaryHeap, HashMap, HashSet};

// Dijkstra
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N, E, S, W,
}

struct Grid(Vec<Vec<u32>>); // fuck that generic nonsense
impl Grid {
    // fn new(grid: Vec<Vec<u32>>) -> Grid {
    //     Grid(grid)
    // }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
}

impl std::ops::Deref for Grid {
    type Target = Vec<Vec<u32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Index<Point> for Grid {
    type Output = u32;
    fn index(&self, idx: Point) -> &u32 {
        &self.0[idx.1][idx.0]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn distance(&self, other: &Point) -> usize {
        let dx = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        let dy = if self.1 > other.1 {
            self.1 - other.1
        } else {
            other.1 - self.1
        };
        dx + dy
    }

    fn shift(&self, d: Dir, grid: &Grid) -> Option<Point> {
        match d {
            Dir::N => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Point(self.0, self.1-1))
                }
            },
            Dir::E => {
                if self.0 == grid.width() - 1 {
                    None
                } else {
                    Some(Point(self.0 + 1, self.1))
                }
            },
            Dir::S => {
                if self.1 == grid.height() - 1 {
                    None
                } else {
                    Some(Point(self.0, self.1 + 1))
                }
            },
            Dir::W => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Point(self.0 - 1, self.1))
                }
            }
        }
    }

}

#[derive(Clone)]
struct State {
    pos: Point,
    facing: Dir,
    cost: u32,
    goal: Point,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        // A*
        let weight = self.cost + self.distance_from_goal();
        let o_weight = other.cost + other.distance_from_goal();
        o_weight.partial_cmp(&weight)
        
        // Dijkstra
        // other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.pos == other.pos &&
            self.facing == other.facing
    }
}
impl Eq for State {}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.facing.hash(state);
    }
}

impl State {
    fn initial(start: Point, goal: Point) -> State {
        State{pos: start, facing: Dir::S, cost: 0, goal}
    }

    fn firsts(start: Point, goal: Point) -> Vec<State> {
        vec![
            State{pos: start, facing: Dir::S, cost: 0, goal},
            State{pos: start, facing: Dir::E, cost: 0, goal},
        ]
    }

    fn nexts(&self, grid: &Grid) -> Vec<State> {
        let mut options = vec![];
        let (a,b) = match self.facing {
            Dir::N | Dir::S => (Dir::E, Dir::W),
            Dir::E | Dir::W => (Dir::N, Dir::S),
        };
        for i in 4..=10 {
            options.push(vec![a; i]);
            options.push(vec![b; i]);
        }

        options.iter().filter_map(|ds| {
            ds.iter().fold(Some((self.pos, 0)), |acc, d| {
                match acc {
                    Some((pos, cost)) => {
                        pos.shift(*d, &grid).map(|p| {
                            (p, cost + grid[p])
                        })
                    },
                    None => None,
                }
            }).map(|(pos, cost)| {
                let cost = self.cost + cost;
                State{pos, facing: ds[0], cost, goal: self.goal}
            })
        }).collect()
    }
    fn distance_from_goal(&self) -> u32 {
        self.pos.distance(&self.goal) as u32
    }

    // fn rep(&self) -> (Point, Dir) {
    //     (self.pos, self.facing)
    // }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    // let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    // let mut history = HashMap::new();
    let mut seen = HashSet::new();

    let grid = lines.map(|row| {
        row.chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
    }).collect();

    let grid = Grid(grid);

    let start = Point(0,0);
    let goal = Point(grid.width()-1, grid.height()-1);
    for s in State::firsts(start, goal) {
        queue.push(s);
    }

    let mut i = 0;
    let mut result = State::initial(start, goal);
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        // let min_rep = state.rep();
        i += 1;
        if i % 1000 == 0 {
            print!("\rstates checked: {}", i);
        }
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());
        if state.pos == goal {
            // println!("\n{}", state.cost);
            result = state;
            break;
        }
        
        for s in state.nexts(&grid) {
            if seen.contains(&s) {
                continue;
            }
            // history.insert(s.clone(), state.clone());
            queue.push(s);
        }
        // let d = dist.get(&s.pos);
        // if d.is_none() || d.unwrap() >= &s.cost {
        //         dist.insert(s.pos, s.cost);
        //         history.insert(s.pos, state.pos);
        //         queue.push(s);
        //     }
        // }
    }
    println!("\rstates checked: {}", i);

    // println!("Map:");
    // let mut output = vec![vec!['.'; grid.width()]; grid.height()];
    // let mut point = &result;
    // while let Some(r) = history.get(point) {
    //     // println!("{:?}", p);
    //     output[r.pos.1][r.pos.0] = match r.facing {
    //         Dir::N => '^',
    //         Dir::E => '>',
    //         Dir::W => '<',
    //         Dir::S => 'v',
    //     };
    //     point = r;
    // } 
    // for row in output {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!("");
    // }

    println!("{}", result.cost);

}
