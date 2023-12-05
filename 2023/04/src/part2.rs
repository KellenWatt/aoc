use std::io::stdin;

use regex::Regex;

// #[macro_use]
use lazy_static::lazy_static;

lazy_static! {
    static ref CARD_PATTERN: Regex = Regex::new(r"Card +(?<id>\d+):(?<winners>(?: +\d+)+) \|(?<numbers>(?: +\d+)+)").unwrap();
}

struct Card {
    id: usize,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_str(desc: &str) -> Card {
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

    let mut cards_gotten: Vec<u32> = vec![1; cards.len()];

    for card in cards.iter() {
        let wins = card.win_count();
        let start = card.id;
        let current = cards_gotten[start - 1];
        for i in start..(start+wins) {
            let count = cards_gotten.get_mut(i);
            if count.is_some() {
                *count.unwrap() += current;
            }
        }
        // cards_gotten[card.id-1] = (start..(start+wins)).map(|i| cards_gotten.get(i).unwrap_or(0)).sum();
    }
    println!("{}", cards_gotten.iter().sum::<u32>())
}
