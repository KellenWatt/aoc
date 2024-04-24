use std::io::stdin;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn zero() -> Point {
        Point{x: 0, y: 0}
    }

    fn points_along(&self, p: &Segment) -> PointsAlong {
        PointsAlong{start: *self, path: *p, traveled: 0}
    }
    fn shift(&self, s: &Segment) -> Point {
        let (dx, dy) = match s.dir {
            Dir::N => (0, -s.len),
            Dir::E => (s.len, 0),
            Dir::S => (0, s.len),
            Dir::W => (-s.len, 0),
        };
        Point{x: self.x + dx, y: self.y + dy}
    }

    fn manhattan(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Clone, Copy)]
enum Dir {
    N, E, S, W,
}

#[derive(Clone, Copy)]
struct Segment {
    dir: Dir,
    len: i32,
}

impl Segment {
    fn from_str(s: &str) -> Segment {
        let (dir, len) = s.split_at(1);
        let dir = match dir {
            "U" => Dir::N,
            "R" => Dir::E,
            "D" => Dir::S,
            "L" => Dir::W,
            _ => panic!("invalid direction '{}'", dir)
        };
        let len = len.parse().unwrap();
        Segment{dir, len}
    }
}

struct PointsAlong {
    start: Point,
    path: Segment,
    traveled: i32,
}

impl Iterator for PointsAlong {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.traveled >= self.path.len {
            return None;
        }
        self.traveled += 1;
        let (x, y) = match self.path.dir {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        };
        let out = Point{x: self.start.x + x * self.traveled, y: self.start.y + y * self.traveled};
        Some(out)
    }
}

fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    
    let pipe1 = lines.next().unwrap().split(",").map(Segment::from_str).collect::<Vec<Segment>>();
    let pipe2 = lines.next().unwrap().split(",").map(Segment::from_str).collect::<Vec<Segment>>();

    let mut min_x = 100000000i32;
    let mut max_x = -100000000i32;
    let mut min_y = 100000000i32;
    let mut max_y = -100000000i32;

    let mut p1 = Point::zero();
    let mut p2 = Point::zero();

    for s in pipe1.iter() {
        p1 = p1.shift(s);
        if p1.x < min_x {
            min_x = p1.x;
        }
        if p1.x > max_x {
            max_x = p1.x;
        }
        if p1.y < min_y {
            min_y = p1.y;
        }
        if p1.y > max_y {
            max_y = p1.y;
        }
    }
    for s in pipe2.iter() {
        p2 = p2.shift(s);
        if p2.x < min_x {
            min_x = p2.x;
        }
        if p2.x > max_x {
            max_x = p2.x;
        }
        if p2.y < min_y {
            min_y = p2.y;
        }
        if p2.y > max_y {
            max_y = p2.y;
        }
    }

    let start = Point{x: -min_x, y: -min_y};
    p1 = start.clone();
    p2 = start.clone();

    let mut grid = vec![vec![0u8; (max_x - min_x) as usize + 1]; (max_y - min_y) as usize + 1];
    for s in pipe1.iter() {
        for p in p1.points_along(s) {
            grid[p.y as usize][p.x as usize] |= 1;
        }
        p1 = p1.shift(s);
    }
    let mut crossings = vec![];
    for s in pipe2.iter() {
        for p in p2.points_along(s) {
            if grid[p.y as usize][p.x as usize] > 0 {
                crossings.push(p.manhattan(&start));
            }
        }
        p2 = p2.shift(s);
    }

    let closest = crossings.iter().min().unwrap();
    println!("{}", closest);

}
