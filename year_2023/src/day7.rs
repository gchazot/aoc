use aoc_utils as utils;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Index;
use std::slice::Iter;

#[test]
fn test_mine() {
    execute();
}

pub fn execute() {
    let hands = Hand::from_file("day7.txt", false);
    assert_eq!(253910319, score(hands));

    let hands_with_jokers = Hand::from_file("day7.txt", true);
    assert_eq!(254083736, score(hands_with_jokers));
}

#[test]
fn test_score() {
    let hands = Hand::from_file("day7-example.txt", false);
    assert_eq!(6440, score(hands));

    let hands_with_jokers = Hand::from_file("day7-example.txt", true);
    assert_eq!(5905, score(hands_with_jokers));
}

type Score = u64;

fn score(mut hands: Vec<Hand>) -> Score {
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as Score + 1) * hand.bid)
        .sum()
}

#[test]
fn test_hands() {
    let mut hands = Hand::from_file("day7-example.txt", false);
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
            _ => "Fail",
        };
        assert_eq!(hand, &Hand::from_text(hand_text, false));
    }
}

#[test]
fn test_hand() {
    let hand1 = Hand::from_text("32T3K", false);
    assert_eq!(5, hand1.cards.len());
    assert_eq!(hand1[0], 3);
    assert_eq!(hand1[1], 2);
    assert_eq!(hand1[2], 10);
    assert_eq!(hand1[3], 3);
    assert_eq!(hand1[4], 13);

    assert_eq!(hand1.bid, 0);

    assert!(matches!(hand1.get_type(), HandType::OnePair));

    let hand2 = Hand::from_text("T55J5", false);
    assert!(matches!(hand2.get_type(), HandType::ThreeOfAKind));

    let hand3 = Hand::from_text("KK677", false);
    assert!(matches!(hand3.get_type(), HandType::TwoPairs));

    let hand4 = Hand::from_text("KTJJT 220", false);
    assert!(matches!(hand4.get_type(), HandType::TwoPairs));
    assert_eq!(220, hand4.bid);

    let hand5 = Hand::from_text("QQQJA 48", false);
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

#[test]
fn test_hand_with_jokers() {
    let hand1 = Hand::from_text("32T3K", true);
    assert!(matches!(hand1.get_type(), HandType::OnePair));

    let hand2 = Hand::from_text("T55J5", true);
    assert!(matches!(hand2.get_type(), HandType::FourOfAKind));

    let hand3 = Hand::from_text("KK677", true);
    assert!(matches!(hand3.get_type(), HandType::TwoPairs));

    let hand4 = Hand::from_text("KTJJT", true);
    assert!(matches!(hand4.get_type(), HandType::FourOfAKind));

    let hand5 = Hand::from_text("QQQJA", true);
    assert!(matches!(hand5.get_type(), HandType::FourOfAKind));

    let hand6a = Hand::from_text("J2AAA", true);
    assert!(matches!(hand6a.get_type(), HandType::FourOfAKind));

    let hand6b = Hand::from_text("2JAAA", true);
    assert!(matches!(hand6b.get_type(), HandType::FourOfAKind));

    assert!(hand6a < hand6b);

    fn hand_type(hand_text: &str) -> HandType {
        let hand = Hand::from_text(hand_text, true);
        hand.get_type()
    }

    assert!(matches!(hand_type("32T3K"), HandType::OnePair));
    assert!(matches!(hand_type("T55J5"), HandType::FourOfAKind));
    assert!(matches!(hand_type("KK677"), HandType::TwoPairs));
    assert!(matches!(hand_type("KTJJT"), HandType::FourOfAKind));
    assert!(matches!(hand_type("QQQJA"), HandType::FourOfAKind));

    assert!(matches!(hand_type("23456"), HandType::HighCard));

    assert!(matches!(hand_type("J3456"), HandType::OnePair));
    assert!(matches!(hand_type("J3356"), HandType::ThreeOfAKind));
    assert!(matches!(hand_type("J3336"), HandType::FourOfAKind));
    assert!(matches!(hand_type("J3333"), HandType::FiveOfAKind));

    assert!(matches!(hand_type("JJ456"), HandType::ThreeOfAKind));
    assert!(matches!(hand_type("JJ446"), HandType::FourOfAKind));
    assert!(matches!(hand_type("JJ444"), HandType::FiveOfAKind));

    assert!(matches!(hand_type("JJJ56"), HandType::FourOfAKind));
    assert!(matches!(hand_type("JJJ55"), HandType::FiveOfAKind));

    assert!(matches!(hand_type("JJJJ6"), HandType::FiveOfAKind));

    assert!(matches!(hand_type("JJJJJ"), HandType::FiveOfAKind));
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: Score,
    jokers: bool,
}

impl Hand {
    fn from_file(filename: &str, jokers: bool) -> Vec<Self> {
        let path = format!("input/{}", &filename);
        utils::read_lines(&path)
            .iter()
            .map(|line| Hand::from_text(line, jokers))
            .collect()
    }

    fn from_text(hand: &str, jokers: bool) -> Self {
        let cards_text = &hand[0..5];
        let cards = cards_text
            .chars()
            .map(|card| card_from_text(card, jokers))
            .collect();

        let bid_text = &hand[5..].trim();
        let bid = bid_text.parse::<Score>().unwrap_or(0);

        Hand { cards, bid, jokers }
    }

    fn get_type(&self) -> HandType {
        let mut counts = HashMap::<Card, usize>::new();
        for &card_type in all_cards(self.jokers) {
            let count = self.cards.iter().filter(|&&card| card == card_type).count();
            counts.insert(card_type, count);
        }

        let joker = card_from_text('J', self.jokers);

        let mut counts_of_counts: Vec<usize> = counts
            .iter()
            .filter(|(&card, &count)| card != joker && count > 0)
            .map(|(_card, &count)| count)
            .collect();

        counts_of_counts.sort();
        counts_of_counts.reverse();

        let num_jokers = counts[&joker];

        if self.jokers {
            if counts_of_counts.is_empty() {
                counts_of_counts.push(0);
            }
            counts_of_counts[0] += num_jokers;
        } else if num_jokers > 0 {
            counts_of_counts.push(num_jokers);
        }

        counts_of_counts.sort();
        counts_of_counts.reverse();

        if counts_of_counts == vec![5] {
            return HandType::FiveOfAKind;
        } else if counts_of_counts == vec![4, 1] {
            return HandType::FourOfAKind;
        } else if counts_of_counts == vec![3, 2] {
            return HandType::FullHouse;
        } else if counts_of_counts == vec![3, 1, 1] {
            return HandType::ThreeOfAKind;
        } else if counts_of_counts == vec![2, 2, 1] {
            return HandType::TwoPairs;
        } else if counts_of_counts == vec![2, 1, 1, 1] {
            return HandType::OnePair;
        } else if counts_of_counts == vec![1, 1, 1, 1, 1] {
            return HandType::HighCard;
        }
        panic!("Unknown hand type: {:?}", self)
    }

    fn get_ord_value(&self) -> (u8, u8, u8, u8, u8, u8) {
        let hand_type = hand_type_value(self.get_type());
        (
            hand_type,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4],
        )
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

fn all_cards(jokers: bool) -> Iter<'static, Card> {
    static CARDS: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    static CARDS_WITH_JOKER: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 12, 13, 14];

    if jokers {
        CARDS_WITH_JOKER.iter()
    } else {
        CARDS.iter()
    }
}

fn card_from_text(card: char, jokers: bool) -> Card {
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
        'J' => {
            if jokers {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            panic!("Not a card: {card}")
        }
    }
}
