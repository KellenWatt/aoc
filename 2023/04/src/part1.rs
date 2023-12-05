use std::io::stdin;

use regex::Regex;

// #[macro_use]
use lazy_static::lazy_static;

lazy_static! {
    static ref CARD_PATTERN: Regex = Regex::new(r"Card +(?<id>\d+):(?<winners>(?: +\d+)+) \|(?<numbers>(?: +\d+)+)").unwrap();
}

struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_str(desc: &str) -> Card {
        println!("{}", desc);
        let caps = CARD_PATTERN.captures(desc).unwrap();
        let id = caps.name("id").unwrap().as_str().parse().unwrap();
        let winners = caps.name("winners").unwrap().as_str().split(' ').filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
        let numbers = caps.name("numbers").unwrap().as_str().split(' ').filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
        Card{id, winners, numbers}
    }

    fn win_count(&self) -> usize {
        self.numbers.iter().filter(|n| self.winners.contains(n)).count()
    }

    fn score(&self) -> u32 {
        let wins = self.win_count();
        if wins > 0 {
            1 << (wins - 1)
        } else {
            0
        }
    }
}

fn main() {
    let lines = stdin().lines();

    let mut cards = vec![];
    for line in lines {
        cards.push(Card::from_str(&line.unwrap()));
    }

    let mut total = 0;
    for card in cards.iter() {
        total += card.score();
    }
    println!("{}", total);
}
