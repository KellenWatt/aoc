use std::io::stdin;
use itertools::Itertools;
use std::time::Instant;

struct Valley {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Valley {
    fn new(rows: Vec<String>) -> Valley {
        let grid: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
        let rows = grid.iter().map(|r| line_to_num(r)).collect();
        let cols = (0..(grid[0].len())).map(|i| {
            let col = grid.iter().map(|row| row[i]).collect();
            line_to_num(&col)
        }).collect();
        Valley{rows, cols}
    }

    fn symmetries(&self) -> usize {
        let mut total = find_symmetry(&self.cols).unwrap_or(0);
        total += find_symmetry(&self.rows).unwrap_or(0) * 100;
        total
    }
}

fn line_to_num(line: &Vec<char>) -> u32 {
    line.iter().enumerate().fold(0, |acc, (i, c)| {
        acc | if *c == '#' {
            1 << i
        } else {
            0
        }
    })
}


fn find_symmetry(lines: &Vec<u32>) -> Option<usize> {
    for fold in 1..(lines.len()) {
        let size = if (lines.len() - fold) < fold {
            lines.len() - fold
        } else {
            fold
        };

        let check = &lines[(fold - size)..fold];
        let rest = &lines[fold..(fold+size)];
        if rest.iter().eq(check.iter().rev()) {
            return Some(fold);
        }
    }
    None
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let start = Instant::now();
    let valleys: Vec<_> = lines.group_by(|l| l.trim().len() == 0).into_iter()
        .filter(|(key, _)| !key)
        .map(|(_, group)| {
            Valley::new(group.collect())
        }).collect();

    let symmetries = valleys.iter().map(|v| {
        v.symmetries()
    });

    println!("{}", symmetries.sum::<usize>());
    println!("time: {}", start.elapsed().as_secs_f32());
}
