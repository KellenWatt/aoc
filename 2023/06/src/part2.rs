use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();

    let first_line = lines.next().unwrap().unwrap();
    let time = first_line.split(" ").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let target = second_line.split(" ").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>().join("").parse::<u64>().unwrap();

    let options = (1..time).map(|press| press * (time - press)).filter(|d| d > &target).count();
    println!("{}" , options);

}
