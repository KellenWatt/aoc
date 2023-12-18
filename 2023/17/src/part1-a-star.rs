use std::io::stdin;
use std::ops::Deref;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N, E, S, W,
}

struct Grid<T: Copy>(Vec<Vec<T>>);

impl<T: Copy> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy> Grid<T> {
    fn width(&self) -> usize {
        self[0].len()
    }
    fn height(&self) -> usize {
        self.len()
    }
}

struct Pointer<'a, T: Copy> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

impl<'a, T: Copy> std::fmt::Display for Pointer<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<'a, T: Copy> PartialEq for Pointer<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && std::ptr::eq(self.grid, other.grid)
    }
}

impl<'a, T: Copy> Eq for Pointer<'a, T> {}

impl<'a, T: Copy> std::hash::Hash for Pointer<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.x);
        state.write_usize(self.y);
    }
}

impl<'a, T: Copy> Clone for Pointer<'a, T> {
    fn clone(&self) -> Self {
        Pointer{x: self.x, y: self.y, grid: self.grid}
    }
}

impl<'a, T: Copy> Pointer<'a, T> {
    fn shift(&self, d: Dir) -> Option<Pointer<'a, T>> {
        match d {
            Dir::N => {
                if self.y == 0 {
                    None
                } else {
                    Some(Pointer{x: self.x, y: self.y-1, grid: self.grid})
                }
            },
            Dir::E => {
                if self.x == self.grid.width() - 1 {
                    None
                } else {
                    Some(Pointer{x: self.x + 1, y: self.y, grid: self.grid})
                }
            },
            Dir::S => {
                if self.y == self.grid.height() - 1 {
                    None
                } else {
                    Some(Pointer{x: self.x, y: self.y + 1, grid: self.grid})
                }
            },
            Dir::W => {
                if self.x == 0 {
                    None
                } else {
                    Some(Pointer{x: self.x - 1, y: self.y, grid: self.grid})
                }
            }
        }
    }
}

// I actually have a decent reason for this. This is to make the other code more reusable by
// copy-paste.
impl<'a> Pointer<'a, u32> {
    fn cost(&self) -> u32 {
        self.grid[self.y][self.x]
    }
}


#[derive(Eq, Clone)]
struct State<'a> {
    pos: Pointer<'a, u32>,
    facing: Dir,
    straight_dist: usize, // the number of steps in the current direction
    cost: u32,
    goal: (usize, usize)
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &State<'a>) -> bool {
        self.pos == other.pos && self.cost == other.cost // && self.straight_dist == other.straight_dist 
    }
}

impl<'a> std::hash::Hash for State<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        // self.facing.hash(state);
        // state.write_usize(straight_dist);
        state.write_u32(self.cost);
    }
}

// reverse ordering to create min-heap from BinaryHeap
// This also establishes the A* heuristic.
impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &State<'a>) -> Option<Ordering> {
        // let weight = self.cost + self.distance_from_goal() as u32;
        // let o_weight = other.cost + other.distance_from_goal() as u32;
        // o_weight.partial_cmp(&weight)
        other.cost.partial_cmp(&self.cost).map(|o| {
            o.then(other.straight_dist.cmp(&self.straight_dist))
        })
        // if weight < o_weight {
        //     Some(Ordering::Greater)
        // } else if weight > o_weight {
        //     Some(Ordering::Less)
        // } else {
        //     other.straight_dist.partial_cmp(&self.straight_dist)
        // }
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &State<'a>) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> State<'a> {
    fn initial(grid: &'a Grid<u32>) -> State<'a> {
        State{
            pos: Pointer{
                x: 0,
                y: 0,
                grid
            }, 
            facing: Dir::S, 
            straight_dist: 0, 
            cost: 0,
            goal: (grid.width()-1, grid.height()-1),
        }
    }

    fn nexts(&self) -> Vec<State<'a>> {
        let mut options = vec![];
        let at_limit = self.straight_dist == 3;
        if !at_limit {
            options.push(self.facing);
        }
        let (d1, d2) = match self.facing {
            Dir::N | Dir::S => (Dir::E, Dir::W),
            Dir::E | Dir::W => (Dir::N, Dir::S),
        };
        options.push(d1);
        options.push(d2);
        options.iter().filter_map(|d| {
            self.pos.shift(*d).map(|p| {
                let straight_dist = if d == &self.facing {
                    self.straight_dist + 1
                } else {
                    1
                };
                let cost = self.cost + p.cost();
                State{pos: p, facing: *d, straight_dist, cost, goal: self.goal}
            })
        }).collect()
    }

    fn at_goal(&self) -> bool {
        // self.pos.x == self.goal.0 && self.pos.y == self.goal.1
        self.distance_from_goal() == 0
    }

    // works when goal is at largest extreme. Otherwise needs more work
    fn distance_from_goal(&self) -> usize {
        self.goal.0 - self.pos.x + self.goal.1 - self.pos.y
    }
}



fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let proto_grid = lines.map(|row| {
        row.chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
    }).collect();
    let grid = Grid(proto_grid);

    let mut queue = BinaryHeap::new();
    queue.push(State::initial(&grid));

    let mut seen = HashSet::new();

    let mut i = 2u64;
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());
        i += 1;
        print!("\rstates inspected: {}", i);

        if state.at_goal() {
            println!("\n{}", state.cost);
            return;
        }
        
        for s in state.nexts() {
            if !seen.contains(&s) {
                queue.push(s);
            }
        }
    }
    println!("A* didn't find a path in an open grid! Something horrible has gone wrong with math!");
}
