use crate::utils::read_lines;
use std::collections::HashMap;
use std::io::BufRead;

pub fn main() {
    let raw_line = read_lines("src/day01/input.txt");
    let num_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let calibration_val: u32 = raw_line
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let frwd = line.chars();
            let bwkd = line.chars().rev().collect::<String>();
            let mut charsets = HashMap::from([("frwd", (frwd, 0)), ("bkwd", (bwkd.chars(), 0))]);

            // run two while loops, first through all characters forwards and then backwards
            for charset in charsets.iter_mut() {
                // create buffer for chars we've observed as we iterate through the line
                let mut char_buffer = String::new();

                'char_iterator: loop {
                    match charset.1.0.next() {
                        Some(c) => {
                            // first digit we see we want to extract...
                            if c.is_digit(10) {
                                charset.1.1 = c.to_digit(10).unwrap();
                                break 'char_iterator;
                            } else {
                                // or the first digit spelled out we want to extract...
                                char_buffer.push(c);
                                for key in num_map.keys() {
                                    let mut search_string = *key;
                                    let bkwd_search_string = key.chars().rev().collect::<String>();

                                    // important to reverse order of search string if we're iterating backwards
                                    if *charset.0 == "bkwd" {
                                        search_string = bkwd_search_string.as_str();
                                    }
                                    if char_buffer.contains(search_string) {
                                        charset.1.1 = num_map[key];
                                        break 'char_iterator;
                                    }
                                }
                            }
                        }
                        None => {
                            println!("End of line reached without a digit, something's up!");
                            break;
                        }
                    }
                }
            }
            return [
                charsets["frwd"].1.to_string(),
                charsets["bkwd"].1.to_string(),
            ]
            .join("")
            .parse::<u32>()
            .unwrap();
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("{:?}", calibration_val);
}
