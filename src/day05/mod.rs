use crate::utils::read_lines;
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
struct Map(u64, u64, u64, u64);

#[derive(Debug, Clone)]
struct Range {
    s_range: std::ops::Range<u64>,
    s_start: u64,
    s_end: u64,
    d_start: u64,
    d_end: u64,
}

#[derive(Debug, Clone)]
struct Seed {
    s_start: u64,
    s_end: u64,
}

#[allow(dead_code)]
pub fn main(question: u32) {
    // process input
    let raw_lines = read_lines("src/day05/input.txt");
    let lines = raw_lines
        .unwrap()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let map_lines = lines[2..lines.len()]
        .split(|line| line.contains("map"))
        .map(|l| {
            let l = l.to_vec();
            l.iter()
                .map(|s| {
                    s.split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect::<Vec<Vec<Vec<u64>>>>();

    // build maps
    let mut maps = Vec::new();
    let map_names: HashMap<usize, &str> = HashMap::from([
        (0, "soil"),
        (1, "fertilizer"),
        (2, "water"),
        (3, "light"),
        (4, "temperature"),
        (5, "humidity"),
        (6, "location"),
    ]);
    for x in map_lines.iter() {
        maps.push(build_map(x));
    }

    // generate starting seeds/ranges
    let individual_seeds = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let mut input_ranges: Vec<Seed> = Vec::new();
    if question == 1 {
        input_ranges = individual_seeds
            .iter()
            .map(|s| Seed {
                s_start: *s,
                s_end: *s,
            })
            .collect();
    }
    if question == 2 {
        input_ranges = individual_seeds
            .chunks(2)
            .map(|chunk| Seed {
                s_start: chunk[0],
                s_end: chunk[0] + chunk[1] - 1,
            })
            .collect();
    }

    // calculate final location ranges
    let inputs = input_ranges.clone();
    let mut location_ranges = vec![];
    for range in inputs {
        let mut input = vec![range];
        let mut output = vec![];
        // println!("initial seed range: {:?}", input);
        for (i, map) in maps.iter().enumerate() {
            map_range(
                map,
                map_names.get(&i).unwrap().to_string(),
                &mut input,
                &mut output,
            );
            input = output.clone();
        }
        // println!("output ranges: {:?}", output);
        // println!("-----------------------------");
        let output_nums = output.iter().map(|s| s.s_start).collect::<Vec<u64>>();
        let minimum = output_nums.iter().min().unwrap();
        location_ranges.push(minimum.clone());
    }
    println!("{:?}", location_ranges.iter().min().unwrap());
}

fn build_map(map_lines: &Vec<Vec<u64>>) -> Vec<Range> {
    let mut result: Vec<Range> = Vec::new();
    map_lines.iter().for_each(|set| {
        result.push(Range {
            d_start: set[0],
            d_end: set[0] + set[2],
            s_range: set[1]..set[1] + set[2],
            s_start: set[1],
            s_end: set[1] + set[2],
        });
    });
    return result;
}

fn map_range(
    rules: &Vec<Range>,
    map_name: String,
    queue: &mut Vec<Seed>,
    processed: &mut Vec<Seed>,
) {
    // processed is copied into the queue at the end of each iteration of map_range, so we clear it to avoid double counting
    processed.clear();

    while queue.len() > 0 {
        let mut curr_seed = queue.pop().unwrap();
        let mut curr_seed_consumed = false;

        for (k, range) in rules.iter().enumerate() {
            // case 1: seed is fully within source range, is consumed and we skip the remaining rules by moving to the next seed
            if range.s_range.contains(&curr_seed.s_start)
                && range.s_range.contains(&curr_seed.s_end)
            {
                processed.push(Seed {
                    s_start: range.d_start + (curr_seed.s_start - range.s_start),
                    s_end: range.d_start + (curr_seed.s_end - range.s_start),
                });

                // mark seed as consumed
                // break the loop since no need to continue iterating through the maps
                // println!(
                //     "{} map | {} rule | full overlap {:?} {:?} - curr seed range consumed",
                //     map_name, k, curr_seed, range.s_range
                // );
                curr_seed_consumed = true;
                break; // maybe not necessary
            }
            // case 2: seed's lower bound is outside the range, we process part and trim the remaining seed
            else if curr_seed.s_start < range.s_start
                && (range.s_range).contains(&curr_seed.s_end)
            {
                // add partially mapped seed to processed list
                processed.push(Seed {
                    s_start: range.d_start,
                    s_end: range.d_start + (curr_seed.s_end - range.s_start),
                });

                // trim the remaining seed range and push it back into the queue
                // println!(
                //     "{} map | {} rule | lower bound partial {:?} {:?} - curr seed range modified and partial range added to queue",
                //     map_name, k, curr_seed, range.s_range
                // );
                curr_seed.s_end = range.s_start - 1;
                curr_seed_consumed = false;
                // println!("new seed range: {:?}", curr_seed);
            }
            // case 3: seed's upper bound is outside the range, we process part and trim the remaining seed
            else if curr_seed.s_end > range.s_end && range.s_range.contains(&curr_seed.s_start) {
                // add partially mapped seed to processed list
                processed.push(Seed {
                    s_start: range.d_end - (range.s_end - curr_seed.s_start),
                    s_end: range.d_end,
                });

                // trim the remaining seed range and push it back into the queue
                // println!(
                //     "{} map | {} rule | upper bound partial {:?} {:?} - curr seed range modified and partial range added to queue",
                //     map_name, k, curr_seed, range.s_range
                // );
                curr_seed.s_start = range.s_end + 1;
                curr_seed_consumed = false;
                // println!("new seed range: {:?}", curr_seed);
            }
            // case 4: seed range is bigger than the map range, so we need to trim and return two seeds
            else if curr_seed.s_start < range.s_start && curr_seed.s_end > range.s_end {
                // add internal partially mapped seed to the processed list
                processed.push(Seed {
                    s_start: range.d_start,
                    s_end: range.d_end,
                });

                // trim the remaining seed range and push it back into the queue
                // println!(
                //     "{} map | {} rule | seed encompasses map range {:?} {:?} - partial mapped range processed and two new ranges created/added to queue",
                //     map_name, k, curr_seed, range.s_range
                // );

                // add lower excluded range to queue
                queue.push(Seed {
                    s_start: curr_seed.s_start,
                    s_end: range.s_start - 1,
                });
                // println!("new seed range: {:?}", queue.last().unwrap());

                // adjust curr seed to be only the upper excluded range
                curr_seed.s_start = range.s_end + 1;
                // println!("new seed range: {:?}", curr_seed);
                curr_seed_consumed = false;
            }
        }
        if !curr_seed_consumed {
            // if the seed range doesn't match any mapping, just add it to the processed list
            // println!(
            //     "{} map | no overlap {:?} -- seed value unchanged and added to processed list",
            //     map_name, curr_seed
            // );
            processed.push(curr_seed);
        }
    }
}
