use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use num_integer::gcd;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl FromStr for Point {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Point, Infallible> {
        let parts = s.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        let x = parts[0];
        let y = parts[1];
        Ok(Point{x, y})
    }   
}

impl Point {
    fn displacement(&self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn extrapolate(&self, v: &Vector) -> Point {
        Point{
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Vector {
    x: f64,
    y: f64,
}
impl FromStr for Vector {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Vector, Infallible> {
        let parts = s.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        let x = parts[0];
        let y = parts[1];
        let mut out = Vector{x, y};
        out.reduce();
        Ok(out)
    }
}

impl Vector {
    fn cross(&self, other: &Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn reduce(&mut self) {
        let div = gcd(self.x as i64, self.y as i64) as f64;
        self.x /= div;
        self.y /= div;
    }

    fn parallel(&self, other: &Vector) -> bool {
        self == other
    }
}

#[derive(Debug)]
struct HailStone {
    p: Point,
    v: Vector,
}

impl HailStone {
    fn line(&self) -> (f64, f64) {
        let q = self.p.extrapolate(&self.v);
        let m = (q.y - self.p.y) / (q.x - self.p.x);

        let y0 = self.p.y - m * self.p.x;
        (m, y0)
    }

    fn ahead(&self, p: &Point) -> bool {
        if self.v.x > 0.0 && self.p.x > p.x ||
           self.v.x < 0.0 && self.p.x < p.x ||
           self.v.y > 0.0 && self.p.y > p.y ||
           self.v.y < 0.0 && self.p.y < p.y {
            false
        } else {
            true
        }
    }

    fn collision_point(&self, other: &HailStone) -> Option<Point> {
        let (a, c) = self.line();
        let (b, d) = other.line();
        if a == b {
            return None;
        }
        let x = (d-c) / (a-b);
        let y = a*x + c;

        let p = Point{x, y};

        if !(self.ahead(&p) && other.ahead(&p)) {
            None
        } else {
            Some(p)
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
    n >= 200000000000000.0 && n <= 400000000000000.0
    // n >= 7.0 && n <= 27.0
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let stones: Vec<HailStone> = lines.enumerate().map(|(i, line)| {
        let h = line.parse().unwrap();
        // println!("stone {} - {:?}", i, h);
        h
    }).collect();

    // check if ..= needed if failing
    // let checked_range = 200000000000000u64..400000000000000u64;

    let mut total = 0;
    for (i, h) in stones[..(stones.len()-1)].iter().enumerate() {
        for (j, s) in stones[(i+1)..].iter().enumerate() {
            if let Some(Point{x, y}) = h.collision_point(s) {
                // println!("crossing of {} and {}", i, i+j+1);
                if in_range(x) && in_range(y) {
                    total += 1;
                }
            }
        }
    }
    
    println!("{}", total);
}
