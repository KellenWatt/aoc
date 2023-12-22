use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use std::collections::{HashSet};

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


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let mut blocks: Vec<Block> = lines.map(|line| {
        let mut b: Block = line.parse().unwrap();
        b
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
    // for b in blocks.iter() {
    //     println!("{:?}~{:?}", b.start, b.end);
    //     for p in b.points() {
    //         println!("  {:?}", p);
    //     }
    // }
    // for (i, id) in space.iter().enumerate().filter_map(|(i, p)| if p.is_some() {Some((i,*p))} else {None}) {
    //     let x = i % 10;
    //     let y = (i / 10) % 10;
    //     let z = i / 100;
    //     println!("{},{},{}: {}", x, y, z, id.unwrap());
    // }


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
    // print layers
    // for z in 1..10 {
    //     for y in 0..10 {
    //         for x in 0..10 {
    //             let idx = x + y * 10 + z * 100;
    //             if let Some(id) = space[idx] {
    //                 print!("{}", id);
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }

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

    // println!("space filled: {}", space.iter().filter_map(|p| *p).count());

    let removable = blocks.len() - required.len();

    println!("required:");
    for id in required {
        println!("- block {}", id);
    }

    println!("{}", removable);
}
