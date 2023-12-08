use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TYPE {
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
    cards: String,
    label_type: TYPE,
    bid: u32,
    rank: u32,
}


// enum Labels { L2, L3, L4, L5, L6, L7, L8, L9,LT, LJ, LQ, LK, LA }
const LABELS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9','T', 'J', 'Q', 'K', 'A'];
// const LABELS: HashMap<&str, u32> = HashMap::from([("2", 0), ("3", 1), ("4", 2), ("5", 3), ("6", 4), ("7", 5), ("8", 6), ("9", 7), ("T", 8), ("J", 9), ("Q", 10), ("K", 11), ("A", 12)]);

fn get_labels() -> HashMap<char, usize> {
    let mut label_map: HashMap<char, usize> = HashMap::new();

    for (l, label) in LABELS.iter().enumerate() {
        label_map.insert(*label, l);
    }
    label_map
}

pub fn part1(lines: Vec<&str>) -> u32 {
    let labels = get_labels();
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

        hands.push(Hand {
            cards,
            bid,
            label_type,
            rank: 0,
        });
    }

    hands.sort_by(|a, b| {
        match a.label_type.cmp(&b.label_type).reverse() {
            Ordering::Equal => a.cards.cmp(&b.cards).reverse(),
            other => other,
        }
    });

    let mut total_winnings = 0;
    let mut rank = 1;
    for mut hand in hands {
        hand.rank = rank;
        rank += 1;

        total_winnings += hand.rank * hand.bid;
    }


    return total_winnings;
}
