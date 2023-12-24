use std::io::stdin;
use std::collections::{VecDeque, HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N, E, S, W,
}

enum Cell {
    Tree,
    Path,
    // Slope(Dir),
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        match c {
            '#' => Cell::Tree,
            _ => Cell::Path,
            // _ => panic!("unrecognized input: '{}'", c)
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            &Cell::Tree => '\u{2588}',
            &Cell::Path => ' ',
            // &Cell::Slope(d) => {
            //     match d {
            //         Dir::N => '^',
            //         Dir::E => '>',
            //         Dir::S => 'v',
            //         Dir::W => '<',
            //     }
            // }
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
        
        // if let Cell::Slope(d) = forest[self] {
        //     return if let Some(p) = self.shift(d, forest.width, forest.height) {
        //         vec![p]
        //     } else {
        //         vec![]
        //     }
        // }
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

struct Graph(HashMap<Point, HashMap<Point, usize>>);

impl Graph {
    fn reduce(forest: &Forest, start: Point) -> Graph {
        let mut nodes = HashMap::new();
        let mut queue = VecDeque::new();
        let mut seen: HashSet<Point> = HashSet::new();
        queue.push_back(start);
        while let Some(p) = queue.pop_front() {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);
            nodes.entry(p).or_insert_with(|| HashMap::new());

            // for each path out of node, explore it until you find anoter intesrsection
            for mut n in p.neighbors(&forest) {
                let mut depth = 0;
                loop {
                    let ns = n.neighbors(&forest);
                    let ns: Vec<Point> = ns.into_iter().filter(|neigh| {
                        !seen.contains(neigh) || (nodes.contains_key(neigh) && neigh != &p)
                    }).collect();
                    depth += 1;
                    if ns.len() > 1 {
                        nodes.get_mut(&p).unwrap().insert(n, depth);
                        nodes.entry(n).and_modify(|m| {m.insert(p, depth);}).or_insert_with(|| {
                            let mut h = HashMap::new();
                            h.insert(p, depth);
                            h
                        });
                        queue.push_back(n);
                        break;
                    } else if ns.len() == 0 && n.y == forest.grid.len()-1{
                        nodes.get_mut(&p).unwrap().insert(n, depth);
                        nodes.entry(n).and_modify(|m| {m.insert(p, depth);}).or_insert_with(|| {
                            let mut h = HashMap::new();
                            h.insert(p, depth);
                            h
                        });
                        break;

                    } else if ns.len() == 1 && nodes.contains_key(&ns[0]) {
                        nodes.get_mut(&p).unwrap().insert(ns[0], depth+1);
                        nodes.entry(ns[0]).and_modify(|m| {m.insert(p, depth+1);}).or_insert_with(|| {
                            let mut h = HashMap::new();
                            h.insert(p, depth+1);
                            h
                        });

                        break
                    } else if ns.len() == 0 {
                        break;
                    }
                    
                    seen.insert(n); 
                    n = ns[0];
                }
            }
        }
        Graph(nodes)
    }
    
    fn neighbors(&self, p: Point) -> Vec<(Point, usize)> {
        let mut out = vec![];
        if let Some(ns) = self.0.get(&p) {
            for (n, dist) in ns.iter() {
                out.push((*n, *dist))
            }
        }
        out
    }

    fn longest_path(&self, start: &Point, end: &Point, visited: &mut HashSet<Point>) -> Option<usize> {
        if start == end {
            return Some(0);
        }
        visited.insert(*start);
        let mut max = None;
        for (p, dist) in self.0.get(&start).unwrap().iter() {
            if visited.contains(p) {
                continue;
            }
            let Some(long) = self.longest_path(p, end, visited) else {
                continue;
            };

            let total = dist + long;

            if max.is_none() || total > max.unwrap() {
                max = Some(total);
            }
        }

        visited.remove(start);
        max
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let init = Instant::now();
    let forest = lines.map(|line| {
        line.chars().map(|c| {
            Cell::from(c)
        }).collect()
    }).collect();
    let forest = Forest::new(forest);
    let start = forest.start();
    let end = forest.end();


    // TODO 
    // reduce graph to minimal representation
    //   - have intersections as nodes, and distance between as edge weights 
    //     - maybe just do by hand?
    //     - modified BFGS would work too
    // After, similar algorithm to now should do well enough

    // let reduced = reduce_graph(start, &forest);
    let reduced = Graph::reduce(&forest, start);


    // print reduction
    // for (p, ps) in reduced.0.iter() {
    //     println!("{:?}", p);
    //     for (q, dist) in ps.iter() {
    //         println!("  {:?} = {}", q, dist);
    //     }
    // }
    // return;

    // overlay reduction positions on forest
    // for (y, row) in forest.grid.iter().enumerate() {
    //     for (x, c) in row.iter().enumerate() {
    //         let p = Point{x,y};
    //         if reduced.0.contains_key(&p) {
    //             print!("O");
    //         } else {
    //             print!("{}", c);
    //         }
    //     }
    //     println!();
    // }


    println!("{:?}", reduced.longest_path(&start, &end, &mut HashSet::new()));
    println!("elapsed: {}", init.elapsed().as_secs_f32());
    // correct answer 6534
}
