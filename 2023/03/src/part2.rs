use std::io::stdin;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

static NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Position {
    fn invalid() -> Position {
        Position{x: 0, y: 0}
    }
    fn is_valid(&self, dims: (u32, u32)) -> bool {
        let dims = (dims.0 as i32, dims.1 as i32);
        self.x >= 0 && self.y >= 0 && self.x < dims.0 && self.y < dims.1
    }

    fn translate(&self, x: i32, y: i32) -> Position {
        Position{x: self.x + x, y: self.y + y}
    }

    fn neighbors(&self, dims: (u32, u32)) -> Vec<Position> {
        NEIGHBOR_OFFSETS.iter().filter_map(|(x, y)| {
            let p = self.translate(*x, *y);
            if p.is_valid(dims) {
                Some(p)
            } else {
                None
            }
        }).collect()
    }
}

#[derive(Debug)]
struct Cell(i32, Position);

impl Cell {
    fn new(value: i32, p: Position) -> Cell {
        Cell(value, p)
    }
    
    fn empty(p: Position) -> Cell {
        Cell(0, p)
    }

    fn symbol(value: u8, p: Position) -> Cell {
        Cell(-(value as i32), p)
    }

    fn is_symbol(&self) -> bool {
        self.0 < 0
    }

    fn value(&self) -> i32 {
        self.0
    }
    
    fn pos(&self) -> &Position {
        &self.1
    }

    fn get_symbol(&self) -> Option<char> {
        if self.is_symbol() {
            Some(((-self.0) as u8) as char)
        } else {
            None
        }
    }
}

fn aggregate_digits(line: &[u8], row: i32) -> Vec<Cell> {
    let mut current_number = 0;
    let mut digits = 0;

    let mut output = vec![];
    for i in 0..(line.len()) {
        // output.push(Cell::empty());
        if line[i].is_ascii_digit() {
            current_number *= 10;
            current_number += (line[i] as i32) - 0x30;
            digits += 1;
            if i == line.len() - 1 {
                for j in (i-digits)..i {
                    output.push(Cell::new(current_number, Position{x: j as i32, y: row}))
                }
            }
        } else {
            for j in (i-digits)..i {
                output.push(Cell::new(current_number, Position{x: j as i32, y: row}))
            }
            current_number = 0;
            digits = 0;

            if line[i] != 0x2E { // period
                output.push(Cell::symbol(line[i], Position{x: i as i32, y: row}));
            } else {
                output.push(Cell::empty(Position{x: i as i32, y: row}));
            }
        }
    }
    output
}



fn main() {
    let lines: Vec<String> = stdin().lines().map(|l|l.unwrap()).collect();

    let mut grid: Vec<Vec<Cell>> = vec![];
    let mut total = 0;

    let rows = lines.len() as u32;
    let mut cols = 0;
    for (i,line) in lines.iter().enumerate() {
        let num_length = 0; 
        cols = line.len() as u32;
        let line = aggregate_digits(line.as_bytes(), i as i32);
        grid.push(line);
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.get_symbol().map(|c| c == '*').unwrap_or(false) {
                let mut seen = vec![];
                for p in cell.pos().neighbors((cols, rows)).iter() {
                    let v = grid[p.y as usize][p.x as usize].value();
                    if v != 0 && !seen.contains(&v) { // it's a good enough set (max 6 items)
                        seen.push(v)
                    }
                }
                if seen.len() != 2 {
                    continue;
                }
                total += seen[0] * seen[1];
            }
        }
    }
    println!("{}", total)
}
