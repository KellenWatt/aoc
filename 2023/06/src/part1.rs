use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();

    let first_line = lines.next().unwrap().unwrap();
    let times = first_line.split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap());
    let second_line = lines.next().unwrap().unwrap();
    let targets = second_line.split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| s.parse::<u32>().unwrap());

    let options = times.map(|time| {
        (1..time).map(move |press| press * (time - press))
    }).zip(targets).map(|(dists, target)| {
        dists.filter(|d| d > &target).count()
    }).collect::<Vec<usize>>();

    println!("{}", options.iter().product::<usize>());

}
