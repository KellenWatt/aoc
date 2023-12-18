use std::io::stdin;
use std::collections::VecDeque;

enum Dir {
    Up, Right, Down, Left,
}

impl Dir {
    fn from(s: &str) -> Dir {
        match s {
            "U" => Dir::Up,
            "R" => Dir::Right,
            "D" => Dir::Down,
            "L" => Dir::Left,
            _ => panic!("unspecified direction")
        }
    }
}

struct Wall {
    len: isize,
    dir: Dir,
}

impl Wall {
    fn from_instruction(inst: &str) -> Wall {
        let parts: Vec<&str> = inst.split(" ").collect();
        // let dir = Dir::from(parts[0]);
        // let len = parts[1].parse().unwrap();
        // let color = parts[2][2..8].to_owned();
        let code = &parts[2][2..8];
        let len = isize::from_str_radix(&code[..5], 16).unwrap();
        let dir = match &code[5..6] {
            "3" => Dir::Up,
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            _ => panic!("not a real digit")
        };

        Wall{len, dir}
    }

    fn is_horizontal(&self) -> bool {
        match self.dir {
            Dir::Right | Dir::Left => true,
            _ => false,
        }
    }
}


// Shoelace formula for calculating the area of an arbitrary polygon
fn shoelace(points: &Vec<(isize, isize)>) -> u64 {
    let mut res = 0i64;
    for i in 0..(points.len()-1) { // -1 to make math faster (no mod check)
        let p = points[i];
        let q = points[i+1];
        // x(n) * y(n+1) - y(n) * x(n+1)
        res += (p.0*q.1 - p.1*q.0) as i64;
    }
    let vn = points.last().unwrap();
    let v1 = points[0];
    res += (vn.0*v1.1 - vn.1*v1.0) as i64;

    (res / 2) as u64
}

fn pit_area(walls: &Vec<Wall>, start: (isize, isize)) -> u64 {
    let mut vertices = vec![];
    let mut point = start;
    for wall in walls.iter() {
        let mut len = wall.len;
        let v = match wall.dir {
            Dir::Up => (point.0, point.1 - len),
            Dir::Right => (point.0 + len, point.1),
            Dir::Down => (point.0, point.1 + len),
            Dir::Left => (point.0 - len, point.1),
        };
        vertices.push(v);
        point = v;
    }
    
    let area = shoelace(&vertices);
    let perimeter = walls.iter().map(|w| w.len).sum::<isize>() as u64;
    area + perimeter/2 + 1
}

// This doesn't even remotely work in part 2. The scales are ridiculous
// fn visualize(grid: &Vec<Vec<Cell>>) {
//     for row in grid.iter() {
//         for c in row.iter() {
//             let c = match c {
//                 Cell::Wall(_) => '#',
//                 Cell::Pit => '-',
//                 Cell::Untouched => '.',
//             };
//             print!("{}", c);
//         }
//         println!("");
//     }
// }


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let walls: Vec<Wall> = lines.map(|line| {
        Wall::from_instruction(&line)
    }).collect();


    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;

    for Wall{len: l, dir: d} in walls.iter() {
        let l = *l as isize;
        match d {
            Dir::Up => {y -= l;},
            Dir::Right => {x += l;},
            Dir::Down => {y += l;},
            Dir::Left => {x -= l;},
        }
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }
    }

    // let max_width = max_x - min_x + 1;
    // let max_height = max_y - min_y + 1;
    let start = ((-min_x), (-min_y));

    println!("dimensions: {} x {}", max_x - min_x + 1, max_y - min_y + 1);

    // No.
    //visualize(&grid);
    
    let area = pit_area(&walls, start);

    println!("{}", area);

}
