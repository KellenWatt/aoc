use std::io::stdin;
use regex::Regex;
use std::ops::RangeInclusive;

const LINE_REGEX: &'static str = r"(?<method>t(?:urn o(?:ff|n)|oggle)) (?<start>\d{1,3},\d{1,3}) through (?<end>\d{1,3},\d{1,3})";


#[derive(Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn from(s: &str) -> Point {
        let (x, y) = s.split_once(",").unwrap();
        Point(x.parse().unwrap(), y.parse().unwrap())
    }
    
    fn to(self, to: Point) -> Rect {
        Rect{x_range: self.0..=to.0, y_range: self.1..=to.1}
    }
}

struct Rect {
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
}

impl Rect {
    fn apply(&self, grid: &mut Vec<Vec<i32>>, action: &str) {
        for row in grid[self.x_range.clone()].iter_mut() {
            for c in row[self.y_range.clone()].iter_mut() {
                *c += match action {
                    "turn on" => 1,
                    "turn off" =>  {
                        if *c == 0 {0} else {-1}
                    }
                    "toggle" => 2,
                    _ => panic!("unreachable")
                }
            }
        }
    }
}

fn main() {
    let instruction = Regex::new(LINE_REGEX).unwrap();

    let lines = stdin().lines().map(|l| l.unwrap());

    let mut grid = vec![vec![0; 1000]; 1000];

    for line in lines {
        let caps = instruction.captures(&line).unwrap();
        let method = caps.name("method").unwrap().as_str();
        let start = caps.name("start").unwrap().as_str();
        let end = caps.name("end").unwrap().as_str();

        let rect = Point::from(start).to(Point::from(end));
        rect.apply(&mut grid, method);
    }

    let total: i32 = grid.iter().map(|row| row.iter().sum::<i32>()).sum();
    println!("{}", total);

}
