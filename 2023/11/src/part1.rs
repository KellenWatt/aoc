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

impl Sub<&Point> for &Point {
    type Output = Offset;
    fn sub(self, other: &Point) -> Offset {
        Offset(self.0 - other.0, self.1 - other.1)
    }
}

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

fn expand_rows(grid: Grid) -> Grid {
    let mut output = vec![];
    for row in grid {
        if row_is_void(&row) {
            output.push(row.clone());
        }
        output.push(row)
    }
    output
}

fn expand_cols(mut grid: Grid) -> Grid {
    let empty_cols: Vec<usize> = (0..(grid[0].len())).rev().filter(|i| col_is_void(*i, &grid)).collect();
    for row in grid.iter_mut() { // may need &mut
        for i in empty_cols.iter() {
            row.insert(i+1, '.');
        }
    }
    grid
}

fn expand_grid(grid: Grid) -> Grid {
    let grid = expand_cols(grid);
    expand_rows(grid)
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let space: Grid = lines.map(|line| line.chars().collect()).collect();
    let space = expand_grid(space);

    let mut galaxies = vec![];

    for (y, row) in space.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push(Point::new(x, y));
            }
        }
    }

    let mut graph = vec![vec![0; galaxies.len()]; galaxies.len()];

    for (i, g) in galaxies.iter().enumerate() {
        for (j, h) in galaxies.iter().enumerate().skip(i+1) {
            graph[i][j] = (g - h).magnitude();
        }
    }
    // for (i, g) in galaxies.iter().enumerate() {
    //     println!("{: >2}: {:?}", i, g);
    // }
    // for line in graph.iter() {
    //     for c in line.iter() {
    //         print!("{: >3}", c);
    //     }
    //     println!("");
    // }

    let total: usize = graph.iter().map(|row| row.iter().sum::<usize>()).sum();
    println!("{}", total);

}
