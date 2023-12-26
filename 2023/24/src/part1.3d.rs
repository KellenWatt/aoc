use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use num_integer::gcd;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Point {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Point, Infallible> {
        let parts = s.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        let x = parts[0];
        let y = parts[1];
        let z = parts[2];
        Ok(Point{x, y, z})
    }   
}

impl Point {
    fn displacement(&self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}
impl FromStr for Vector {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Vector, Infallible> {
        let parts = s.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        let x = parts[0];
        let y = parts[1];
        let z = parts[2];
        let mut out = Vector{x, y, z};
        out.reduce();
        Ok(out)
    }
}

impl Vector {
    fn cross(&self, other: &Vector) -> Vector {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector{x,y,z}
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn reduce(&mut self) {
        let div = gcd(self.x as i64, gcd(self.y as i64, self.z as i64)) as f64;
        self.x /= div;
        self.y /= div;
        self.z /= div;
    }

    fn parallel(&self, other: &Vector) -> bool {
        self == other
    }

    fn parallel_2d(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct HailStone {
    p: Point,
    v: Vector,
}

impl HailStone {
    // source: https://mikespivey.wordpress.com/2016/10/06/how-do-you-tell-whether-two-lines-intersect-in-3d/
    // Two vectors are guaranteed to intersect if they are non-parallel and on the same Euclidean
    // plane.
    fn will_collide(&self, other: &HailStone) -> bool {
        if self.v == other.v {
            return false;
        }
        let normal = self.v.cross(&other.v);
        let disp = self.p.displacement(&other.p);

        let res = normal.dot(&disp);

        res.abs() < 0.000000001
    }

    fn collision_point(&self, other: &HailStone) -> Option<Point> {
        if !self.will_collide(other) {
            return None;
        }

        let left = self.v.cross(&other.v);
        let right = other.p.displacement(&self.p).cross(&other.v);

        let a = right.mag() / left.mag();
        if a < 0.0 {
            return None;
        }

        let out = Point {
            x: self.p.x + a * self.v.x,
            y: self.p.y + a * self.v.y,
            z: self.p.z + a * self.v.z,
        };
        Some(out)
    }

    fn collision_point_2d(&self, other: &HailStone) -> Option<Point> {
        if self.v.parallel(self.other.v) {
            return None;
        }

    }
}

impl FromStr for HailStone {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<HailStone, Infallible> {
        let (p, v) = s.split_once(" @ ").unwrap();
        let p = p.parse().unwrap();
        let v = v.parse().unwrap();
        Ok(HailStone{p, v})
    }
}

fn in_range(n: f64) -> bool {
    // n >= 200000000000000.0 && n <= 400000000000000.0
    n >= 7.0 && n <= 27.0
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let stones: Vec<HailStone> = lines.enumerate().map(|(i, line)| {
        let h = line.parse().unwrap();
        println!("stone {} - {:?}", i, h);
        h
    }).collect();

    // check if ..= needed if failing
    // let checked_range = 200000000000000u64..400000000000000u64;

    let mut total = 0;
    for (i, h) in stones[..(stones.len()-1)].iter().enumerate() {
        for (j, s) in stones[(i+1)..].iter().enumerate() {
            if h.will_collide(s) {
                println!("{} and {} can collide", i, j+i+1)
            }
            if let Some(Point{x, y, ..}) = h.collision_point(s) {
                println!("crossing of {} and {}", i, i+j+1);
                if in_range(x) && in_range(y) {
                    total += 1;
                }
            }
        }
    }
    
    println!("{}", total);
}
