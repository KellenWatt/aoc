use std::io::stdin;
use std::str::FromStr;
use std::convert::Infallible;
use z3::ast::{Ast, Int};

// I have no idea what I'm doing with Z3, so this is paraphrased from 
// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/24.rs

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
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

#[derive(PartialEq, Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}
impl FromStr for Vector {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Vector, Infallible> {
        let parts = s.splitn(3, ", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<_>>();
        let x = parts[0];
        let y = parts[1];
        let z = parts[2];
        let mut out = Vector{x, y, z};
        Ok(out)
    }
}

#[derive(Debug)]
struct HailStone {
    p: Point,
    v: Vector,
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

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let stones: Vec<HailStone> = lines.enumerate().map(|(i, line)| {
        let h = line.parse().unwrap();
        // println!("stone {} - {:?}", i, h);
        h
    }).collect();


    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x0 = Int::new_const(&ctx, "x0");
    let y0 = Int::new_const(&ctx, "y0");
    let z0 = Int::new_const(&ctx, "z0");

    let dx0 = Int::new_const(&ctx, "Δx0");
    let dy0 = Int::new_const(&ctx, "Δy0");
    let dz0 = Int::new_const(&ctx, "Δz0");

    let zero = Int::from_i64(&ctx, 0);
    for (i, stone) in stones.iter().enumerate() {
        let x = Int::from_i64(&ctx, stone.p.x);
        let y = Int::from_i64(&ctx, stone.p.y);
        let z = Int::from_i64(&ctx, stone.p.z);
        
        let dx = Int::from_i64(&ctx, stone.v.x);
        let dy = Int::from_i64(&ctx, stone.v.y);
        let dz = Int::from_i64(&ctx, stone.v.z);

        let t = Int::new_const(&ctx, format!("t{}", i));
        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &dx * &t)._eq(&(&x0 + &dx0 * &t))));
        solver.assert(&((&y + &dy * &t)._eq(&(&y0 + &dy0 * &t))));
        solver.assert(&((&z + &dz * &t)._eq(&(&z0 + &dz0 * &t))));
    }
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let res = model.eval(&(&x0 + &y0 + &z0), true).unwrap();
    
    let summed_coords = res.as_i64().unwrap();
    
    println!("{}", summed_coords);
}
