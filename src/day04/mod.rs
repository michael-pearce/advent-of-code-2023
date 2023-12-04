use crate::utils::read_lines;
use regex::Regex;
use std::io::BufRead;
// use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    num_matches: usize,
    points: u32,
}

#[allow(dead_code)]
pub fn main() {
    let re = Regex::new(r"\d{1,2}").unwrap();
    let raw_lines = read_lines("src/day04/input.txt");
    let initial_cards = raw_lines
        .unwrap()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let line = line.unwrap();
            let card_vals = line.split(":").collect::<Vec<&str>>()[1];
            let winning_n = extract_nums(&re, &String::from(card_vals.split("|").nth(0).unwrap()));
            let scratch_n = extract_nums(&re, &String::from(card_vals.split("|").nth(1).unwrap()));
            let mut num_matches: usize = 0;
            let mut points: u32 = 0;

            scratch_n.iter().for_each(|num| {
                if winning_n.contains(num) {
                    points = (2 as u32).pow(num_matches as u32);
                    num_matches += 1;
                }
            });

            return Card {
                id: id + 1,
                num_matches,
                points: points,
            };
        })
        .collect::<Vec<Card>>();

    let total_points = initial_cards.iter().fold(0, |acc, card| acc + card.points);
    println!("Total points: {}", total_points);

    let mut final_cards = initial_cards.clone();
    for card in initial_cards.iter() {
        flatten_copies(&card, &initial_cards, &mut final_cards);
    }
    println!("Final # of cards: {:?}", final_cards.len());
}

fn flatten_copies(card: &Card, initial_cards: &Vec<Card>, final_cards: &mut Vec<Card>) {
    match card.num_matches {
        n if n > 0 => {
            for child_card in initial_cards[card.id..card.id + card.num_matches].to_vec() {
                final_cards.push(child_card.clone());
                flatten_copies(&child_card, initial_cards, final_cards)
            }
        }
        _ => (),
    }
}

fn extract_nums(re: &Regex, text: &String) -> Vec<i32> {
    return re
        .captures_iter(&String::from(text))
        .map(|capture| {
            if let Some(matching_num) = capture.get(0) {
                return matching_num.as_str().parse::<i32>().unwrap();
            } else {
                return -1;
            }
        })
        .collect::<Vec<i32>>();
}
