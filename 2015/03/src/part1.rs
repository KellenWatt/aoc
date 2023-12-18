use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let instrs: Vec<_> = stdin().lines().map(|l| l.unwrap()).next().unwrap().chars().collect();

    let mut seen = HashSet::new();

    let mut santa: (i32, i32) = (0,0);
    let mut robot: (i32, i32) = (0,0);
    seen.insert(santa);
    for c in instrs.chunks(2) {
        santa = match c[0] {
            '^' => (santa.0, santa.1+1),
            '>' => (santa.0+1, santa.1),
            'v' => (santa.0, santa.1-1),
            '<' => (santa.0-1, santa.1),
            _ => panic!("unreachable")
        };
        seen.insert(santa);
        robot = match c[1] {
            '^' => (robot.0, robot.1+1),
            '>' => (robot.0+1, robot.1),
            'v' => (robot.0, robot.1-1),
            '<' => (robot.0-1, robot.1),
            _ => panic!("unreachable")
        };
        seen.insert(robot);
    }

    println!("{}", seen.len());
}
