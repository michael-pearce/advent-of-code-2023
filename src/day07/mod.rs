use crate::utils::read_lines;
use counter::Counter;
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
struct Game {
    hand: String,
    joker_hand: String,
    bid: u64,
}

#[allow(dead_code)]
pub fn main(question: u32) {
    // define card strength
    let q1_card_strength: HashMap<char, i32> = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', 9),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);
    let mut q2_card_strength = q1_card_strength.clone();
    *q2_card_strength.get_mut(&'J').unwrap() = -1;

    // process input
    let raw_lines = read_lines("src/day07/input.txt").unwrap();
    let mut games = raw_lines
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let halves = line.split(" ").collect::<Vec<&str>>();
            let hand = halves[0].to_string();
            let bid = halves[1].parse::<u64>().unwrap();

            return Game {
                hand: hand.clone(),
                joker_hand: hand.clone(),
                bid: bid,
            };
        })
        .collect::<Vec<Game>>();

    if question == 2 {
        sub_jokers(&mut games);
        sort_games(&mut games, &q2_card_strength, question);
    } else {
        sort_games(&mut games, &q1_card_strength, question);
    }

    let total_winnings = games.iter().enumerate().fold(0, |acc, (rank, game)| {
        let winnings = game.bid * (rank as u64 + 1);
        return acc + winnings;
    });

    println!("total winnings: {}", total_winnings)
}

fn sort_games(games: &mut Vec<Game>, card_strength: &HashMap<char, i32>, question: u32) {
    games.sort_by(|a, b| {
        let a_counts = a.hand.chars().collect::<Counter<_>>();
        let b_counts = b.hand.chars().collect::<Counter<_>>();

        if a_counts.len() < b_counts.len() {
            // having less unique cards is always strictly better
            return std::cmp::Ordering::Greater;
            // this branch has two conditions
            // 1. different hands but with the same number of unique cards (easy)
            // 2. identical hands but with different card values (hard)
        } else if a_counts.len() == b_counts.len() {
            let a_max = a_counts.values().max().unwrap();
            let b_max = b_counts.values().max().unwrap();
            // condition 1: higher max is strictly better for unique hands
            if a_max != b_max {
                return a_max.cmp(&b_max);
            } else {
                // condition 2: iterate through char pairs one at a time and compare strength, returning the first non-equal comparison
                let mut pairs = a.hand.chars().zip(b.hand.chars());
                if question == 2 {
                    pairs = a.joker_hand.chars().zip(b.joker_hand.chars());
                }
                for (a_char, b_char) in pairs {
                    match card_strength[&a_char].cmp(&card_strength[&b_char]) {
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => continue,
                    }
                }
                println!(
                    "This final return should never be reached, but is here to satisfy the compiler :)"
                );
                return std::cmp::Ordering::Equal;
            }
        } else {
            // having more unique cards is always strictly worse
            return std::cmp::Ordering::Less;
        }
    });
}

fn sub_jokers(games: &mut Vec<Game>) {
    games.iter_mut().for_each(|game| {
        let mut counts = game.hand.chars().collect::<Counter<_>>();
        if counts.contains_key(&'J') {
            let n_j = counts.remove(&'J').unwrap();
            // no replacement necessary if there are 5 jokers
            if n_j < 5 {
                game.hand = game.hand.replace(
                    'J',
                    counts
                        .iter()
                        .find_map(|(k, v)| {
                            if v == counts.values().max().unwrap() {
                                return Some(k);
                            } else {
                                return None;
                            }
                        })
                        .unwrap()
                        .to_string()
                        .as_str(),
                );
            }
        }
    });
}
