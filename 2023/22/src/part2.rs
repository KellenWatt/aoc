use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Point, Infallible> {
        let c = s.split(",").collect::<Vec<_>>();
        Ok(Point{x: c[0].parse().unwrap(), y: c[1].parse().unwrap(), z: c[2].parse().unwrap()})
    }
}

impl Point {
    fn below(&self) -> Option<Point> {
        // intentionally not putting safety check, better to panic in this case
        if self.z > 1 {
            let mut p = self.clone();
            p.z -= 1;
            Some(p)
        } else {
            None
        }
    }

    fn above(&self) -> Point {
        let mut p = self.clone();
        p.z += 1;
        p
    }

    fn step_between(&self, other: &Point) -> Point {
        if self.x != other.x {
            Point{x: 1, y: 0, z: 0}
        } else if self.y != other.y {
            Point{x: 0, y: 1, z: 0}
        } else if self.z != other.z {
            Point{x: 0, y: 0, z: 1}
        } else {
            Point{x: 0, y: 0, z: 0}
        }
    }
}

impl Point {
    fn id(&self) -> usize {
        self.x + self.y * 10 + self.z * 100
    }
}

struct PointIter {
    current: Point,
    end: Point,
    step: Point, // not a valid point, just a convenient representation
    done: bool,
}

impl Iterator for PointIter {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        let output = self.current;
        if !self.done {
            let x = output.x + self.step.x;
            let y = output.y + self.step.y;
            let z = output.z + self.step.z;
            self.current = Point{x, y, z};
            self.done = output == self.end;
            Some(output)
        } else {
            None
        }
    }
}

struct Block {
    start: Point,
    end: Point,
    id: Option<usize>,
}

impl Block {
    fn new(start: Point, end: Point) -> Block {
        // built on the assumption that only 1 dimension is different between the two
        if start.x != end.x {
            if start.x < end.x {
                Block{start, end, id: None}
            } else {
                Block{start: end, end: start, id: None}
            }
        } else if start.y != end.y {
            if start.y < end.y {
                Block{start, end, id: None}
            } else {
                Block{start: end, end: start, id: None}
            }
        } else if start.z != end.z {
            if start.z < end.z {
                Block{start, end, id: None}
            } else {
                Block{start: end, end: start, id: None}
            }
        } else {
            Block{start, end, id: None}
        }
    }

    fn set_id(&mut self, id: usize) {
        if self.id.is_none() {
            self.id = Some(id);
        }
    }

    fn points(&self) -> PointIter {
        PointIter{current: self.start, end: self.end, step: self.start.step_between(&self.end), done: false}
    }

    fn below(&self) -> Option<Vec<Point>> {
        let points: Vec<Option<Point>> = self.points().map(|p| p.below()).collect();
        if points.iter().any(|p| p.is_none()) {
            None
        } else {
            Some(points.into_iter().map(|p| p.unwrap()).collect())
        }
    }

    fn above(&self) -> Vec<Point> {
        self.points().map(|p| p.above()).collect()
    }

    fn drop(&mut self) {
        self.start = self.start.below().unwrap();
        self.end = self.end.below().unwrap();
    }
}

fn sort_blocks(blocks: &mut Vec<Block>) {
    blocks.sort_by(|b, c| {
        b.start.z.cmp(&c.start.z)
    });
}

impl FromStr for Block {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Block, Infallible> {
        let (start, end) = s.split_once("~").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Ok(Block::new(start, end))
    }
}

// to check if fully supporting:
// for each node at least partially supported by node 'id' (according to bottom-up dependency graph)
//   take the dependency graph from top-down:
//   if node 'id' appears as the only node in each branch of the top-down tree, then it is fully
//   supported
//     (if dependencies == 1 and node-below == id, don't push and continue
//      if dependencies == 0 || dependencies > 1 and id in nodes-below, return false
//      when queue is empty, return true
//     )
//   if node not fully supported by 'id', don't consider any above

fn fully_supported(id: usize, test: usize, top_down: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(test);

    while let Some(n) = queue.pop_front() {
        if seen.contains(&n) {
            continue;
        }
        seen.insert(n);

        let below = &top_down[&n];
        if below.len() == 1 && below.contains(&id) {
            continue;
        }
        if below.len() == 0 || below.len() > 1 && below.contains(&id) {
            return false;
        }
        for b in below {
            queue.push_back(*b);
        }
    }
    true
}


fn count_supporting(id: usize, bottom_up: &HashMap<usize, HashSet<usize>>, top_down: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut above = VecDeque::new();
    let mut queue = VecDeque::new();
    queue.push_back(id);
    while let Some(id) = queue.pop_front() {
        if above.contains(&id) {
            continue;
        }
        above.push_back(id);
        for s in bottom_up[&id].iter() {
            queue.push_back(*s);
        }
    }
    let _ = above.pop_front(); // get rid of root

    above.iter().filter(|n| fully_supported(id, **n, top_down)).count()
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut blocks: Vec<Block> = lines.map(|line| {
        line.parse().unwrap()
    }).collect();
    for (i, b) in blocks.iter_mut().enumerate() {
        b.set_id(i);
    }

    sort_blocks(&mut blocks);

    let mut space = [None; 30000];
    // let mut space = [None; 1000]; // test data size, for memory efficiency

    // positioning our blocks in space;
    for b in blocks.iter() {
        for p in b.points() {
            space[p.id()] = b.id.clone();
        }
    }

    // dropping all the blocks as far as they can go
    for b in blocks.iter_mut() {
        // let block = b;
        while let Some(below) = b.below() {
            if below.iter().all(|p| space[p.id()].is_none() || space[p.id()] == b.id) {
                for p in b.points() {
                    space[p.id()] = None;
                }
                for p in below {
                    space[p.id()] = b.id.clone();
                }
                b.drop();
            } else {
                break;
            }
        }    
    }

    let mut bottom_up_map = HashMap::new();
    for b in blocks.iter() {
        let mut above_me = HashSet::new();
        for p in b.above() {
            if space[p.id()].is_some() && space[p.id()] != b.id {
                above_me.insert(space[p.id()].unwrap());
            }
        }
        bottom_up_map.insert(b.id.unwrap(), above_me);
    }

    let mut top_down_map = HashMap::new();
    for b in blocks.iter() {
        let mut below_me = HashSet::new();
        let below = b.below().unwrap_or(vec![]);
        for p in below {
            if space[p.id()].is_some() && space[p.id()] != b.id {
                below_me.insert(space[p.id()].unwrap());
            }
        }
        top_down_map.insert(b.id.unwrap(), below_me);
    }

    // for each block, check the number of blocks below.
    // if equal to 1, add below id to set of "required"
    // output is total size - required

    let mut required = HashSet::new();
    for b in blocks.iter() {
        let mut supports = HashSet::new();
        if let Some(below) = b.below() {
            for p in below {
                if space[p.id()].is_some() && space[p.id()] != b.id { 
                    supports.insert(space[p.id()].unwrap());
                }
            }
        }

        if supports.len() == 1 {
            for id in supports { // this feels like a waste
                required.insert(id);
            }
        }
    }

    let above_each = required.iter().map(|id| {
        let res = count_supporting(*id, &bottom_up_map, &top_down_map);
        // println!("{} supports {}", id, res);
        res
    });

    // println!("space filled: {}", space.iter().filter_map(|p| *p).count());

    println!("{}", above_each.sum::<usize>());

}
