use crate::utils::read_lines;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Race {
    max_time: u64,
    max_dist: u64,
}

#[allow(dead_code)]
pub fn main(question: u64) {
    // process input
    let raw_lines = read_lines("src/day06/input.txt").unwrap();
    let mut times = vec![];
    let mut distances = vec![];
    process_input(raw_lines, &mut times, &mut distances, question);
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(i, j)| {
            return Race {
                max_time: *i,
                max_dist: *j,
            };
        })
        .collect::<Vec<Race>>();

    // calculate error margin
    let mut error_margin = 1;
    for race in races {
        let mut count = 0;
        for time_held in 0..race.max_time {
            if (time_held * race.max_time - time_held.pow(2)) > race.max_dist {
                count += 1;
            }
        }
        error_margin *= count;
    }

    println!("error margin: {}", error_margin);
}

fn process_input(
    input: BufReader<File>,
    times: &mut Vec<u64>,
    distances: &mut Vec<u64>,
    question: u64,
) {
    input.lines().enumerate().for_each(|(i, line)| {
        let line = line.unwrap();
        let vals = line.split(":").nth(1).unwrap();
        let mut _parsed_val: Vec<u64> = vec![];

        if question == 2 {
            let mut val_str = vals.to_string();
            val_str.retain(|c| !c.is_whitespace());
            _parsed_val = vec![val_str.parse::<u64>().unwrap()];
        } else {
            _parsed_val = vals
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        }
        if i == 0 {
            *times = _parsed_val;
        } else {
            *distances = _parsed_val;
        }
    });
}
