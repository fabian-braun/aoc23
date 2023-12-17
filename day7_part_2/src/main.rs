use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let file_context = read_to_string("input_day7").unwrap();
    let count: usize = file_context
        .lines()
        .map(|line| HandBid::from_str(line).unwrap())
        .sorted()
        .map(|hand_bid| {
            println!("{:?}", hand_bid);
            hand_bid.bid
        })
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid)
        .sum();

    println!("Part I solution: {}", count);
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Hand {
    typ: Typ,
    labels: [Label; 5],
}

impl Hand {
    fn new(hand: [Label; 5]) -> Self {
        let mut counts = hand.iter().counts();
        let jokers = counts.remove(&Label::J).unwrap_or(0usize);
        let typ = if jokers == 0 {
            if counts.len() == 5 {
                Typ::HighCard
            } else if counts.len() == 4 {
                Typ::OnePair
            } else if counts.len() == 3 {
                if *counts.values().max().unwrap() == 3usize {
                    Typ::ThreeOfKind
                } else {
                    Typ::TwoPair
                }
            } else if counts.len() == 2 {
                if *counts.values().max().unwrap() == 4usize {
                    Typ::FourOfKind
                } else {
                    Typ::FullHouse
                }
            } else {
                Typ::FiveOfKind
            }
        } else if jokers == 1 {
            if counts.len() == 4 {
                // (1,1,1,1) + 1J
                Typ::OnePair
            } else if counts.len() == 3 {
                // (2,1,1,0) + 1J
                Typ::ThreeOfKind
            } else if counts.len() == 2 {
                // (2,2,0,0) | (3,1,0,0) + 1J
                if *counts.values().next().unwrap() == 2usize {
                    Typ::FullHouse
                } else {
                    Typ::FourOfKind
                }
            } else {
                Typ::FiveOfKind
            }
        } else if jokers == 2 {
            if counts.len() == 3 {
                // (1,1,1) + 2J
                Typ::ThreeOfKind
            } else if counts.len() == 2 {
                // (2,1,0) + 2J
                Typ::FourOfKind
            } else {
                // (3,0,0) + 2J
                Typ::FiveOfKind
            }
        } else if jokers == 3 {
            if counts.len() == 2 {
                // (1,1) + 3J
                Typ::FourOfKind
            } else {
                // (2,0) + 3J
                Typ::FiveOfKind
            }
        } else {
            Typ::FiveOfKind
        };
        Self { typ, labels: hand }
    }

    fn typ(&self) -> Typ {
        self.typ
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct HandBid {
    labels: Hand,
    bid: usize,
}

impl FromStr for HandBid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let bid = usize::from_str(bid).unwrap();
        let hand = hand_from_str(hand)?;
        Ok(Self { labels: hand, bid })
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum Typ {
    // ascending order for deriving Ord
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash)]
enum Label {
    // ascending order for deriving Ord
    J,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    T,
    Q,
    K,
    A,
}

fn hand_from_str(s: &str) -> Result<Hand, String> {
    let mut result: [Label; 5] = [Label::A; 5];
    s.chars()
        .take(5)
        .map(|c| match c {
            '2' => Label::L2,
            '3' => Label::L3,
            '4' => Label::L4,
            '5' => Label::L5,
            '6' => Label::L6,
            '7' => Label::L7,
            '8' => Label::L8,
            '9' => Label::L9,
            'T' => Label::T,
            'J' => Label::J,
            'Q' => Label::Q,
            'K' => Label::K,
            'A' => Label::A,
            invalid => {
                panic!("Invalid char {}", invalid)
            }
        })
        .enumerate()
        .for_each(|(i, lbl)| {
            result[i] = lbl;
        });
    Ok(Hand::new(result))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
