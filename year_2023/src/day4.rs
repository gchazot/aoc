use aoc_utils as utils;
use std::collections::HashSet;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let cards = Card::from_file("day4.txt");

    assert_eq!(21959, simple_wins(&cards));
    assert_eq!(5132675, correct_wins(&cards));
}

#[test]
fn test_simple_wins() {
    let example_cards = Card::from_file("day4-example.txt");

    assert_eq!(13, simple_wins(&example_cards));
    assert_eq!(30, correct_wins(&example_cards));
}

fn simple_wins(cards: &Vec<Card>) -> u32 {
    return cards.iter().map(Card::simple_score).sum::<u32>();
}

#[test]
fn test_correct_wins() {
    let example_cards = Card::from_file("day4-example.txt");
    assert_eq!(30, correct_wins(&example_cards));
}

fn correct_wins(cards: &Vec<Card>) -> u32 {
    let mut copies = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let score = card.correct_score();
        for j in 0..score {
            if i + j + 1 >= copies.len() {
                break;
            }
            copies[i + j + 1] += copies[i];
        }
    }

    return copies.iter().sum();
}

#[derive(Debug)]
struct Card {
    winners: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn from_file(filename: &str) -> Vec<Card> {
        let path = format!("input/{}", &filename);
        let lines = utils::read_lines(&path);
        return Vec::from_iter(lines.iter().map(|line| Card::from_text(&line)));
    }

    fn from_text(text: &str) -> Card {
        let (_card_text, lists_text) = text.split_once(": ").unwrap();
        let (winners_text, numbers_text) = lists_text.split_once(" | ").unwrap();

        let winners = HashSet::from_iter(
            winners_text
                .trim()
                .split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap),
        );
        let numbers = HashSet::from_iter(
            numbers_text
                .trim()
                .split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap),
        );

        return Card { winners, numbers };
    }

    fn simple_score(&self) -> u32 {
        let matches = self.winners.intersection(&self.numbers);
        match matches.count() {
            0 => 0,
            i => 2u32.pow((i as u32) - 1),
        }
    }

    fn correct_score(&self) -> usize {
        let matches = self.winners.intersection(&self.numbers);
        return matches.count();
    }
}
