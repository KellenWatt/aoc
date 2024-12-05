use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let grid = lines.map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut xmases = 0u64;

    let offsets = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            let mut any_xmas = false;
            for off in offsets.iter() {
                // println!("{:?}", off);
                if xmas_in_dir(&grid, x, y, off).is_some() {
                    any_xmas = true;
                    xmases += 1;
                }
            }

            // print!("{}", if any_xmas {"X"} else {"."});
        }
        // println!();
    }

    println!("{}", xmases);
}

fn xmas_in_dir(grid: &Vec<Vec<char>>, x: usize, y: usize, offset: &(isize, isize)) -> Option<()> {
    for (i, c) in "XMAS".chars().enumerate() {
        let offset = (offset.0 * i as isize, offset.1 * i as isize);
        let o_x: usize = (offset.0 + x as isize).try_into().ok()?;
        let o_y: usize = (offset.1 + y as isize).try_into().ok()?;
        // println!("{} {} ({}, {})", i, c, o_x, o_y);
        if grid.get(o_y)?.get(o_x)? != &c {
            return None;
        }
    }

    Some(())
}
