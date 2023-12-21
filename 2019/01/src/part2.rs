use std::io::stdin;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let fuel = lines.map(|l| {
        let mut mass = l.parse::<i64>().unwrap();
        let mut fuel = 0;
        loop {
            mass = mass / 3 - 2;
            if mass <= 0 {
                break;
            }
            fuel += mass;
        }
        fuel
    }).sum::<i64>();
 
    // let mut fuel = 0;
    // loop {
    //     println!("{}", mass);
    //     mass = mass / 3 - 2;
    //     if mass <= 0 {
    //         break;
    //     }
    //     fuel += mass;
    // }
    println!("{}", fuel);

}
