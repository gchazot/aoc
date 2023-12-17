use std::collections::HashSet;
use crate::utils;

pub fn execute() {
    let example = utils::read_lines("src/day4/example.txt");
    let example_cards = Vec::from_iter(example.iter().map(|line|Card::from_text(&line)));

    assert_eq!(13, simple_wins(&example_cards));
    assert_eq!(30, correct_wins(&example_cards));

    let data = utils::read_lines("src/day4/mine.txt");
    let cards = Vec::from_iter(data.iter().map(|line|Card::from_text(&line)));

    assert_eq!(21959, simple_wins(&cards));
    assert_eq!(5132675, correct_wins(&cards));
}

fn simple_wins(cards: &Vec<Card>) -> u32 {
    cards.iter().map(Card::simple_score).sum::<u32>()
}

fn correct_wins(cards: &Vec<Card>) -> u32 {
    let mut copies = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let score = card.correct_score();
        for j in 0..score {
            if i+j+1 >= copies.len() {
                break;
            }
            copies[i+j+1] += copies[i];
        }
    }

    return copies.iter().sum();
}

#[derive(Debug)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn from_text(text: &str) -> Card {
        let (card_text, lists_text) = text.split_once(": ").unwrap();

        let card_id = card_text[5..].trim();
        let id = card_id.parse::<u32>().unwrap();

        let (winners_text, numbers_text) = lists_text.split_once(" | ").unwrap();

        let winners = HashSet::from_iter(winners_text.trim().split_whitespace().map(str::parse::<u32>).map(Result::unwrap));
        let numbers = HashSet::from_iter(numbers_text.trim().split_whitespace().map(str::parse::<u32>).map(Result::unwrap));

        return Card {id, winners, numbers};
    }

    fn simple_score(&self) -> u32 {
        let matches = self.winners.intersection(&self.numbers);
        match matches.count() {
            0 => 0,
            i=> 2u32.pow((i as u32)-1),
        }
    }

    fn correct_score(&self) -> usize {
        let matches = self.winners.intersection(&self.numbers);
        return matches.count();
    }
}
