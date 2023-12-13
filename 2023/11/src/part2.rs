use std::io::stdin;
use std::ops::Sub;


type Row = Vec<char>;
type Grid = Vec<Row>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point(isize, isize);

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point(x as isize, y as isize)
    }
}

// impl Sub<&Point> for &Point {
//     type Output = Offset;
//     fn sub(self, other: &Point) -> Offset {
//         Offset(self.0 - other.0, self.1 - other.1)
//     }
// }

struct Offset(isize, isize);

impl Offset {
    fn magnitude(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}


fn row_is_void(row: &Row) -> bool {
    row.iter().filter(|c| **c == '#').count() == 0
}

fn col_is_void(i: usize, grid: &Grid) -> bool {
    grid.iter().filter(|row| row[i] == '#').count() == 0
}

fn empty_rows(grid: &Grid) -> Vec<isize> {
    (0..(grid.len() as isize)).filter(|i| grid[*i as usize].iter().all(|c| *c == '.')).collect()
}

fn empty_cols(grid: &Grid) -> Vec<isize> {
    (0..(grid[0].len() as isize)).filter(|j| grid.iter().all(|row| row[*j as usize] == '.')).collect()
}

fn make_adjusted_offset(start: &Point, end: &Point, rows: &Vec<isize>, cols: &Vec<isize>) -> Offset {
    let x_range = if start.0 < end.0 {
        (start.0)..(end.0)
    } else {
        (end.0)..(start.0)
    };
    let y_range = if start.1 < end.1 {
        (start.1)..(end.1)
    } else {
        (end.1)..(start.1)
    };

    let scale = 1000000;
    let scale = scale - 1;
    let mut y_mag = (start.0 - end.0).abs();
    for y in rows {
        if y_range.contains(y) {
            y_mag += scale;
        }
    }
    
    let mut x_mag = (start.1 - end.1).abs();
    for x in cols {
        if x_range.contains(x) {
            x_mag += scale;
        }
    }
    Offset(x_mag, y_mag)
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let space: Grid = lines.map(|line| line.chars().collect()).collect();
    // let space = expand_grid(space);

    let mut galaxies = vec![];

    for (y, row) in space.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push(Point::new(x, y));
            }
        }
    }

    let mut graph = vec![vec![0; galaxies.len()]; galaxies.len()];
    let rows = empty_rows(&space);
    let cols = empty_cols(&space);
    println!("empty rows: {:?}", rows);
    println!("empty cols: {:?}", cols);

    for (i, g) in galaxies.iter().enumerate() {
        for (j, h) in galaxies.iter().enumerate().skip(i+1) {
            graph[i][j] = make_adjusted_offset(g, h, &rows, &cols).magnitude();
        }
    }

    let total: usize = graph.iter().map(|row| row.iter().sum::<usize>()).sum();
    println!("{}", total);

}
