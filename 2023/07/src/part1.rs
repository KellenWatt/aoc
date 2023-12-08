use std::io::stdin;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Card, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Number(10)),
            "1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => Ok(Card::Number(s.parse().unwrap())),
            _ => Err("Not a valid card identifier"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: u32,
    score: HandType,
}

impl Hand {
    fn new(cards: &str, bet: u32) -> Hand {
        let cards: Vec<_> = cards.split("").filter(|s| !s.is_empty()).map(|c| c.parse().unwrap()).collect();
        
        let mut counts = HashMap::new();
        for card in cards.iter() {
            counts.entry(card).and_modify(|c| *c += 1).or_insert(1u32);
        }
        let types = counts.len();
        let max_count = counts.values().max().unwrap();


        let score = match (types, max_count) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            (5, 1) => HandType::HighCard,
            _ => panic!("unreachable"),
        };
        
        Hand{cards, bet, score}
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self.score < other.score {
            Some(Ordering::Less)
        } else if self.score > other.score {
            Some(Ordering::Greater)
        } else {
            let res = self.cards.iter().zip(other.cards.iter()).fold(Ordering::Equal, |res, (m, y)| {
                if res.is_eq() {
                    y.cmp(m)
                } else {
                    res
                }
            });
            Some(res)
        }
    }
}


fn main() {
    let lines = stdin().lines();

    let mut hands: Vec<_> = lines.map(|l| {
        let line = l.unwrap();
        let (cards, bet) = line.split_once(" ").unwrap();
        let bet = bet.parse().unwrap();
        Hand::new(cards, bet)
    }).collect();

    hands.sort();
    // for (i, hand) in hands.iter().rev().enumerate() {
    //     println!("{}: {:?}", i+1, hand);
    // }

    let total = hands.iter().rev().enumerate().fold(0, |total, (i, hand)| {
        total + (i as u32 + 1) * hand.bet
    });

    println!("{}", total)

}
