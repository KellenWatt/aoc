use std::io::stdin;

struct Gift {
    sides: Vec<u32>,
}

impl Gift {
    fn new(dims: &str) -> Gift {
        let mut sides: Vec<_> = dims.split("x").map(|s| s.parse().unwrap()).collect();
        sides.sort();
        Gift{sides}
    }

    fn surface(&self) -> u32 {
        let a = self.sides[0];
        let b = self.sides[1];
        let c = self.sides[2];
        2*a*c + 2*a*b + 2*b*c
    }

    fn smallest_side(&self) -> u32 {
        self.sides[0] * self.sides[1]
    }
}

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let gifts = lines.map(|line| {
        Gift::new(&line)
    });

    let total = gifts.map(|g| {
        g.surface() + g.smallest_side()
    }).sum::<u32>();

    println!("{}", total);
}
