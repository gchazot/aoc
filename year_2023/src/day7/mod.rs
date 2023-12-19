use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Index;
use std::slice::Iter;
use crate::utils;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let hands = Hand::from_file("mine.txt");
    assert_eq!(253910319, score(hands));
}


#[test]
fn test_score() {
    let hands = Hand::from_file("example.txt");
    assert_eq!(6440, score(hands));
}

type Score = u64;

fn score(mut hands: Vec<Hand>) -> Score {
    hands.sort();

    hands.iter()
        .enumerate()
        .map(|(i, hand)| (i as Score + 1) * hand.bid)
        .sum()
}

#[test]
fn test_hands() {
    let mut hands = Hand::from_file("example.txt");
    assert_eq!(5, hands.len());

    hands.sort();
    assert_eq!(5, hands.len());

    for (i, hand) in hands.iter().enumerate() {
        let hand_text = match i {
            0 => "32T3K",
            1 => "KTJJT",
            2 => "KK677 ",
            3 => "T55J5",
            4 => "QQQJA",
            _ => "Fail"
        };
        assert_eq!(hand, &Hand::from_text(hand_text));
    }
}


#[test]
fn test_hand() {
    let hand1 = Hand::from_text("32T3K");
    assert_eq!(5, hand1.len());
    assert_eq!(hand1[0], 3);
    assert_eq!(hand1[1], 2);
    assert_eq!(hand1[2], 10);
    assert_eq!(hand1[3], 3);
    assert_eq!(hand1[4], 13);

    assert_eq!(hand1.bid, 0);

    assert!(matches!(hand1.get_type(), HandType::OnePair));

    let hand2 = Hand::from_text("T55J5");
    assert!(matches!(hand2.get_type(), HandType::ThreeOfAKind));

    let hand3 = Hand::from_text("KK677");
    assert!(matches!(hand3.get_type(), HandType::TwoPairs));

    let hand4 = Hand::from_text("KTJJT 220");
    assert!(matches!(hand4.get_type(), HandType::TwoPairs));
    assert_eq!(220, hand4.bid);

    let hand5 = Hand::from_text("QQQJA 48");
    assert!(matches!(hand5.get_type(), HandType::ThreeOfAKind));
    assert_eq!(48, hand5.bid);

    assert!(hand1 < hand2);
    assert!(hand1 < hand3);
    assert!(hand1 < hand4);
    assert!(hand1 < hand5);
    assert!(hand4 < hand2);
    assert!(hand4 < hand3);
    assert!(hand4 < hand5);
    assert!(hand3 < hand2);
    assert!(hand3 < hand5);
    assert!(hand2 < hand5);

    let mut hands = vec![&hand1, &hand2, &hand3, &hand4, &hand5];
    hands.sort();
    let expected = vec![&hand1, &hand4, &hand3, &hand2, &hand5];
    assert_eq!(expected, hands)
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: Score,
}

impl Hand {
    fn from_file(filename: &str) -> Vec<Self> {
        let path = format!("src/day7/{}", &filename);
        utils::read_lines(&path)
            .iter()
            .map(|line|Hand::from_text(line))
            .collect()
    }

    fn from_text(hand: &str) -> Self {
        let cards_text= &hand[0..5];
        let cards = cards_text.chars().map(card_from_text).collect();

        let bid_text = &hand[5..].trim();
        let bid = bid_text.parse::<Score>().unwrap_or(0);

        Hand{cards, bid}
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn get_type(&self) -> HandType {
        let mut counts = HashMap::<Card, usize>::new();
        for &card_type in all_cards() {
            let count = self.cards.iter().filter(|&&card|card == card_type).count();
            counts.insert(card_type, count);
        }

        if counts.iter().any(|(_card, &count)|count == 5) {
            return HandType::FiveOfAKind;
        } else if counts.iter().any(|(_card, &count)|count == 4) {
            return HandType::FourOfAKind;
        } else if counts.iter().any(|(_card, &count)|count == 3) {
            if counts.iter().any(|(_card, &count)|count == 2) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }

        let pairs = counts.iter().filter(|(_card, &count)|count == 2).count();
        if pairs == 2 {
            return HandType::TwoPairs;
        } else if pairs == 1 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn get_ord_value(&self) -> (u8, u8, u8, u8, u8, u8) {
        let hand_type = hand_type_value(self.get_type());
        (hand_type, self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4])
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_ord_value().cmp(&other.get_ord_value())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_ord_value() == other.get_ord_value()
    }
}

impl Index<usize> for Hand {
    type Output = Card;
    fn index(&self, index: usize) -> &Self::Output {
        self.cards.index(index)
    }
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn hand_type_value(hand_type: HandType) -> u8 {
    match hand_type {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPairs => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0,
    }
}

type Card = u8;

fn all_cards() -> Iter<'static, Card> {
    static CARDS: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    CARDS.iter()
}

fn card_from_text(card: char) -> Card {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {panic!("Not a card: {card}")}
    }
}