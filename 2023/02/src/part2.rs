#[macro_use]
extern crate lazy_static;
use std::io::stdin;

use regex::Regex;

lazy_static! {
    static ref game_pattern: Regex = Regex::new(r"Game (?<id>\d+): (?<contents>.+)").unwrap();
}


struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn from_str(line: &str) -> Game {
        let caps = game_pattern.captures(line).unwrap();
        let id = caps["id"].parse().unwrap();
        let mut game = Game{id, red: 0, green: 0, blue: 0};

        let contents = &caps["contents"];
        for grab in contents.split("; ") {
            for dice in grab.split(", ") {
                let (num, color) = dice.split_once(" ").unwrap();
                let num = num.parse().unwrap();
                let max = match color {
                    "red" => &mut game.red,
                    "green" => &mut game.green,
                    "blue" => &mut game.blue,
                    _ => todo!(),
                };
                if num > *max {
                    *max = num;
                }
            }
        }
        game
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() {
    let lines = stdin().lines();

    let mut total = 0;
    for line in lines {
        let g = Game::from_str(&line.unwrap());
        total += g.power();
    }
    println!("{}", total);
}
