use crate::utils::read_lines;
use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
struct Node {
    label: String,
    left_dest: String,
    right_dest: String,
    round_dest: String,
    all_dests: Vec<String>,
    is_start: bool,
}

#[allow(dead_code)]
pub fn main(question: u32) {
    // process input
    let re = Regex::new(r"[0-9A-Z]{3}").unwrap();
    let raw_lines = read_lines("src/day08/input.txt").unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut move_str = String::from("");

    raw_lines.lines().for_each(|line| {
        let line = line.unwrap();
        if line.contains("=") {
            let label = line.split("=").nth(0).unwrap().trim();
            let siblings = re
                .find_iter(line.split("=").nth(1).unwrap().trim())
                .map(|m| m.as_str())
                .collect::<Vec<&str>>();
            nodes.insert(
                label.to_string(),
                Node {
                    label: label.to_string(),
                    left_dest: siblings[0].to_string(),
                    right_dest: siblings[1].to_string(),
                    round_dest: String::from(""),
                    all_dests: Vec::new(),
                    is_start: label.ends_with('A'),
                },
            );
        } else if !line.is_empty() {
            move_str = line.to_string();
        }
    });

    // for each node in the hashmap, either do 1 or 2 based on the question:
    // 1. calculate the final destination of each node after going through one full round of moves
    // 2. calculate the number of moves it takes to get to the first 'XXZ' node (often greater than one round) from each starting node
    let mut round_dests: HashMap<String, Vec<String>> = HashMap::new();

    nodes.keys().for_each(|key| {
        let mut curr_node = key.clone();
        round_dests.insert(key.to_string(), Vec::new());

        'outer: loop {
            for val in move_str.chars() {
                if val == 'L' {
                    round_dests.entry(key.to_string()).and_modify(|v| {
                        v.push(nodes[&nodes[&curr_node].left_dest].label.clone());
                    });
                    curr_node = nodes[&nodes[&curr_node].left_dest].label.clone();
                } else if val == 'R' {
                    round_dests.entry(key.to_string()).and_modify(|v| {
                        v.push(nodes[&nodes[&curr_node].right_dest].label.clone());
                    });
                    curr_node = nodes[&nodes[&curr_node].right_dest].label.clone();
                }
                if round_dests.get(key).unwrap().last().unwrap().ends_with('Z')
                    && nodes[key].is_start
                {
                    break 'outer;
                }
            }
            if !nodes[key].is_start {
                break 'outer;
            }
        }
    });
    nodes.iter_mut().for_each(|(key, node)| {
        let all_dests = round_dests[key].clone();
        node.round_dest = all_dests[move_str.len() - 1].clone();
        node.all_dests = all_dests;
    });

    let mut count = 0;
    if question == 1 {
        let mut curr_node = &nodes["AAA"];
        'outer: loop {
            count += 1;
            if curr_node.round_dest == "ZZZ" {
                break 'outer;
            } else {
                curr_node = &nodes[&curr_node.round_dest];
            }
        }
        count = count * move_str.len();
    } else {
        let start_nodes = nodes
            .clone()
            .iter()
            .filter_map(|(_, v)| v.is_start.then(|| v.all_dests.len()))
            .collect::<Vec<usize>>();

        count = start_nodes.iter().fold(1, |acc, min| {
            return lcm(acc, *min);
        });
    }
    println!("Number of moves: {:?}", count);
}
