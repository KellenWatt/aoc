use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let grid = lines.map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut xmases = 0u64;

    let corners = [
        ['M', 'M', 'S', 'S'],
        ['S', 'M', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
        ['M', 'S', 'S', 'M'],
    ];

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            let mut found = false;
            for corn in corners.iter() {
                if is_x_mas(&grid, x, y, corn).is_some() {
                    xmases += 1;
                    found = true;
                    // print!("A");
                }                
            }
            print!("{}", if found {'A'} else {'.'});
        }
        println!();
    }
    println!("{}", xmases);
}


fn is_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize, corners: &[char; 4]) -> Option<()> {
    if grid.get(y)?.get(x)? != &'A' {
        return None;
    }
    if grid.get(y.wrapping_sub(1))?.get(x.wrapping_sub(1))? != &corners[0] {
        return None;
    }
    if grid.get(y.wrapping_sub(1))?.get(x+1)? != &corners[1] {
        return None;
    }
    if grid.get(y+1)?.get(x+1)? != &corners[2] {
        return None;
    }
    if grid.get(y+1)?.get(x.wrapping_sub(1))? != &corners[3] {
        return None;
    }
    Some(())
}


// fn xmas_in_dir(grid: &Vec<Vec<char>>, x: usize, y: usize, offset: &(isize, isize)) -> Option<()> {
//     for (i, c) in "XMAS".chars().enumerate() {
//         let offset = (offset.0 * i as isize, offset.1 * i as isize);
//         let o_x: usize = (offset.0 + x as isize).try_into().ok()?;
//         let o_y: usize = (offset.1 + y as isize).try_into().ok()?;
//         // println!("{} {} ({}, {})", i, c, o_x, o_y);
//         if grid.get(o_y)?.get(o_x)? != &c {
//             return None;
//         }
//     }
// 
//     Some(())
// }
