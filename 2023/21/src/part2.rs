use std::io::stdin;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Stone,
    Empty(Option<usize>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self, g: &Garden) -> Vec<Point> {
        let mut out = vec![];
        if self.x > 0 {
            out.push(Point{x: self.x-1, y: self.y});
        }
        if self.y > 0 {
            out.push(Point{x: self.x, y: self.y-1});
        }
        if self.x < g.width-1 {
            out.push(Point{x: self.x+1, y: self.y});
        }
        if self.y < g.height-1 {
            out.push(Point{x: self.x, y: self.y+1});
        }
        out.iter().filter_map(|p| (g[p] != Cell::Stone).then_some(*p)).collect()
    }
}


struct Garden {
    space: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Garden {
    fn new(space: Vec<Vec<Cell>>) -> Garden {
        let height = space.len();
        let width = space[0].len();
        Garden{space, width, height}
    }
}

impl std::ops::Index<&Point> for Garden {
    type Output = Cell;
    fn index(&self, p: &Point) -> &Cell {
        let x = p.x.rem_euclid(self.width);
        let y = p.y.rem_euclid(self.height);
        &self.space[y][x]
    }
}

// shamelessly cribbing this from github.com/Philippe-Cholet in the hopes 
// that I one day understand how it works. Because this is magic to me.
impl Garden {

    // This makes sense. Sort of a dijkstras/BFS
    fn render_distances(&mut self, start: Point) {
        let mut queue = VecDeque::new();
        queue.push_back((0, start));
        while let Some((dist, p)) = queue.pop_front() {
            if let Cell::Empty(p_dist @ None) = &mut self.space[p.y as usize][p.x as usize] {
                *p_dist = Some(dist);
                for n in p.neighbors(self) {
                    if matches!(self[&n], Cell::Empty(None)) {
                        queue.push_back((dist+1, n));
                    }
                }
            }
        }
    }

    // This is the magic
    fn count_reachable(&self, start: Point, steps: u32) -> u64 {
        let size = self.width; // the input is assumed to be square;
        // I have no idea how these work or why they're here
        let in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c <= (size - 1) / 2 ||
                usize::max(size - 1 -  r, r) + c >= 3 * (size - 1) / 2
        };
        let in_corner_strict = |r, c| {
            usize::min(size - 1 - r, r) + c < (size - 1) / 2 ||
                usize::max(size - 1 -  r, r) + c > 3 * (size - 1) / 2
        };

        let whole_even = self.space.iter().flatten().filter(|cell| {
            matches!(cell, Cell::Empty(Some(d)) if *d % 2 == 0)
        }).count() as u64;
        let whole_odd = self.space.iter().flatten().filter(|cell| {
            matches!(cell, Cell::Empty(Some(d)) if *d % 2 == 1)
        }).count() as u64;

        let (center, other) = if steps % 2 == 0 {
            (whole_even, whole_odd)
        } else {
            (whole_odd, whole_even)
        };

        let middle = start.x;
        let q = (steps - middle as u32) / size as u32;
        let r = (steps - middle as u32) % size as u32;
        assert_eq!(r, 0); // not needed?
       
        // this is where the magic happens. I call it magic because there's a lot going on,
        // and absolutely no indication of why it works.
        center * (1 + 4 * (2..).step_by(2).take_while(|k| *k < q).map(u64::from).sum::<u64>())
            + other * 4 * (1..).step_by(2).take_while(|k| *k  < q).map(u64::from).sum::<u64>()
            + self.space.iter().enumerate().flat_map(|(r, col)| {
                col.iter().enumerate().map(move |(c, cell)| ((r, c), cell))
            }).map(|((r, c), cell)| {
                if let Cell::Empty(Some(d)) = cell {
                    if d % 2 == q as usize % 2 {
                        if in_corner(r, c) {
                            u64::from(q)
                        } else {
                            0
                        }
                    } else {
                        if in_corner_strict(r, c) {
                            3 * u64::from(q-1) + 2
                        } else {
                            4 * u64::from(q-1) + 4
                        }
                    }
                } else {
                    0
                }
            }).sum::<u64>()
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let start = Instant::now();
    let mut start_point = Point{x: 0, y: 0};
    let grid = lines.enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '.' => Cell::Empty(None),
                '#' => Cell::Stone,
                'S' => {
                    start_point = Point{x, y};
                    Cell::Empty(None)
                },
                _ => panic!("unexpected char '{}' at ({},{})", c, x, y)
            }
        }).collect()
    }).collect();
    let mut garden = Garden::new(grid);
    garden.render_distances(start_point);
    let total = garden.count_reachable(start_point, 26501365);
    println!("{}", total);

    // println!("\n{}", current.len());
    println!("elapsed: {}s", start.elapsed().as_secs_f32());
}
