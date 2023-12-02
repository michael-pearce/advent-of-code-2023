use crate::utils::read_lines;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Round(i32, String);

#[allow(dead_code)]
pub fn main(question: u32) {
    let raw_lines = read_lines("src/day02/input.txt");
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut valid_games_sum = 0;
    let mut sum_power = 0;

    // parse data into structure for easier processing
    let games: Vec<Vec<Round>> = raw_lines
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let rounds: Vec<Round> = line.split(":").collect::<Vec<&str>>()[1]
                .split(";")
                .map(|part| {
                    part.split(",")
                        .map(|slice| {
                            let slice = slice.trim().split(" ").collect::<Vec<&str>>();
                            let color = slice[0].parse::<i32>().unwrap();
                            let val = String::from(slice[1]);
                            return Round(color, val);
                        })
                        .collect::<Vec<Round>>()
                })
                .flat_map(|inner_vec| inner_vec)
                .collect::<Vec<Round>>();
            return rounds;
        })
        .collect();

    // solve problems
    if question == 1 {
        for (id, game) in games.iter().enumerate() {
            let mut game_valid = true;
            for round in game.iter() {
                if round.0 > limits[round.1.as_str()] {
                    game_valid = false;
                    break;
                }
            }
            if game_valid {
                valid_games_sum += id + 1;
            }
        }
        println!("Sum of valid IDs: {}", valid_games_sum);
    } else {
        for game in games.iter() {
            let mut min_cubes: HashMap<&str, i32> =
                HashMap::from([("red", -1), ("green", -1), ("blue", -1)]);

            for round in game.iter() {
                let color = round.1.as_str();
                if round.0 > min_cubes[color] {
                    min_cubes.insert(color, round.0);
                }
            }
            sum_power += min_cubes.values().fold(1, |acc, val| acc * val);
        }
        println!("Sum of powers: {}", sum_power);
    }
}
