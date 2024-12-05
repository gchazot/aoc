use aoc_utils as utils;
use std::cmp::max;

#[test]
fn test_mine() {
    execute()
}

pub fn execute() {
    let bag_content = CubeHand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = utils::read_lines("src/day2/mine.txt");
    assert_eq!(2317, sum_possibles(&bag_content, &input));
    assert_eq!(74804, sum_powers(&input));
}

#[test]
fn test_sum_possibles() {
    let bag_content = CubeHand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let example = utils::read_lines("src/day2/example.txt");
    assert_eq!(8, sum_possibles(&bag_content, &example));
}

fn sum_possibles(bag: &CubeHand, games: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in games {
        let game = Game::from_text(line.as_str());
        if game.is_possible(bag) {
            total += game.id;
        }
    }
    return total;
}

#[test]
fn test_sum_powers() {
    let example = utils::read_lines("src/day2/example.txt");
    assert_eq!(2286, sum_powers(&example));
}

fn sum_powers(games: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for line in games {
        let game = Game::from_text(line.as_str());
        total += game.min_hand().power();
    }
    return total;
}

struct Game {
    id: u32,
    hands: Vec<CubeHand>,
}

impl Game {
    fn from_text(text: &str) -> Game {
        let (game_id, hands_text) = text.split_once(": ").unwrap();
        assert!(game_id.starts_with("Game "));

        let id = game_id[5..].parse::<u32>().unwrap();

        let mut hands = Vec::new();
        for hand_text in hands_text.split("; ") {
            hands.push(CubeHand::from_text(hand_text));
        }

        return Game { id, hands };
    }

    fn is_possible(&self, bag_content: &CubeHand) -> bool {
        for hand in &self.hands {
            if !bag_content.contains(&hand) {
                return false;
            }
        }
        return true;
    }

    fn min_hand(&self) -> CubeHand {
        let [mut red, mut green, mut blue] = [0u32; 3];

        for hand in &self.hands {
            red = max(red, hand.red);
            green = max(green, hand.green);
            blue = max(blue, hand.blue);
        }
        return CubeHand { red, green, blue };
    }
}

struct CubeHand {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeHand {
    fn from_text(text: &str) -> CubeHand {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        let colors = text.split(", ");
        for color in colors {
            let parsed = color.split_once(" ");
            match parsed.unwrap() {
                (n, "red") => red = n.parse::<u32>().unwrap(),
                (n, "green") => green = n.parse::<u32>().unwrap(),
                (n, "blue") => blue = n.parse::<u32>().unwrap(),
                _ => println!("Couldn't parse hand '{}'", color),
            }
        }
        return CubeHand { red, green, blue };
    }

    fn contains(&self, other: &CubeHand) -> bool {
        return self.red >= other.red && self.green >= other.green && self.blue >= other.blue;
    }

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}
