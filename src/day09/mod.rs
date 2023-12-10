use crate::utils::read_lines;
use std::{collections::HashMap, io::BufRead};

#[allow(dead_code)]
pub fn main(question: u32) {
    // process input
    let mut map: HashMap<usize, Vec<Vec<i64>>> = HashMap::new();
    let raw_lines = read_lines("src/day09/input.txt").unwrap();
    let input = raw_lines
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    input.iter().enumerate().for_each(|(i, history)| {
        if question == 2 {
            let mut rev = history.clone();
            rev.reverse();
            map.insert(i, Vec::from([rev.clone()]));
            find_diff(rev.clone(), map.get_mut(&i).unwrap());
        } else {
            map.insert(i, Vec::from([history.clone()]));
            find_diff(history.clone(), map.get_mut(&i).unwrap());
        }
    });

    let next_vals = map.iter().map(|(_, v)| {
        let next = v
            .iter()
            .map(|vals| vals.last().unwrap().clone())
            .sum::<i64>();
        return next;
    });

    println!(
        "Sum of extrapolated values is: {}",
        next_vals.fold(0, |acc, x| acc + x)
    );
}

pub fn find_diff(arr: Vec<i64>, history: &mut Vec<Vec<i64>>) {
    let mut diff = vec![];
    for i in 0..arr.len() - 1 {
        diff.push(arr[i + 1] - arr[i])
    }
    history.push(diff.clone());
    if diff.iter().all(|x| *x == 0) {
        return;
    } else {
        find_diff(diff, history);
    }
}
