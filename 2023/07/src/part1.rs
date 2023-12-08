use std::cmp::Ordering;
use std::collections::{HashSet};
use regex::Regex;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TYPE {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hand {
    pub cards: String,
    pub bid: usize,
    pub label_type: TYPE,
    pub card_values: Vec<usize>,
}


const LABELS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

pub fn part1(lines: Vec<&str>) -> usize {
    let re = Regex::new(r"(\w{5})\s([0-9]+)").unwrap();

    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        let groups = re.captures(line).unwrap();

        let cards = groups.get(1).unwrap().as_str().to_string();
        let bid = groups.get(2).unwrap().as_str().parse().unwrap();

        let uniq_cards = cards.chars().into_iter().collect::<HashSet<_>>();

        let mut counted_cards: Vec<usize> = vec![];
        for card in uniq_cards {
            counted_cards.push(cards.chars().filter(|c| c == &card).count());
        }

        counted_cards.sort();

        let label_type = match counted_cards[..] {
            [1,1,1,1,1] => {TYPE::HighCard}
            [1,1,1,2] => {TYPE::OnePair}
            [1,2,2] => {TYPE::TwoPair}
            [1,1,3] => {TYPE::ThreeOfAKind}
            [2,3] => {TYPE::FullHouse}
            [1,4] => {TYPE::FourOfAKind}
            [5] => {TYPE::FiveOfAKind},
            _ => panic!()
        };

        let mut card_values: Vec<usize> = vec![];
        for card in cards.chars() {
            card_values.push(LABELS.iter().position(|&c| c == card).unwrap())
        }

        hands.push(Hand {
            cards,
            bid,
            label_type,
            card_values,
        });
    }

    hands.sort_by(|a, b| {
        match a.label_type.cmp(&b.label_type).reverse() {
            Ordering::Equal => a.card_values.cmp(&b.card_values),
            other => other,
        }
    });

    let mut total_winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        total_winnings += (rank + 1) * hand.bid;
    }

    return total_winnings;
}
