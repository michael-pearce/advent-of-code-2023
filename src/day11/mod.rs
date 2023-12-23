use crate::utils::{print_grid, read_lines};
use std::{
    cmp::{max, min},
    collections::HashMap,
    io::BufRead,
    vec,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    row: u64,
    col: u64,
}

#[allow(dead_code)]
pub fn main(question: u32) {
    // process input
    let raw_lines = read_lines("src/day11/input.txt").unwrap();
    let ref_grid = raw_lines
        .lines()
        .map(|line| {
            return line.unwrap().chars().collect::<Vec<char>>();
        })
        .collect::<Vec<Vec<char>>>();

    // print_grid(&ref_grid);

    // determine which row/cols will expand
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    for (r, r_val) in ref_grid.iter().enumerate() {
        if r_val.iter().all(|c| *c == '.') {
            empty_rows.push(r);
        }
    }
    for c in 0..ref_grid[0].len() {
        if ref_grid.iter().map(|r| r[c]).all(|c| c == '.') {
            empty_cols.push(c);
        }
    }

    // store hashmap of galaxies
    let mut galaxies: HashMap<usize, Coord> = HashMap::new();
    for (r, r_val) in ref_grid.iter().enumerate() {
        for (c, c_val) in r_val.iter().enumerate() {
            if c_val == &'#' {
                galaxies.insert(
                    galaxies.len(),
                    Coord {
                        row: r as u64,
                        col: c as u64,
                    },
                );
            }
        }
    }

    // calculate distances between galaxies
    let mut pairs: HashMap<String, u64> = HashMap::new();
    let mut mult: u64 = 2;
    if question == 2 {
        mult = 1_000_000;
    }
    for (id, coord) in galaxies.iter() {
        for (id2, coord2) in galaxies.iter() {
            // trick to use sorted list of ids as unique key
            let mut id = [id, id2];
            id.sort();
            let str = format!("{:?}", id);

            // only find distance on unique pairs of galaxies
            if id[0] != id[1] && !pairs.contains_key(&str) {
                let l_bound_r = min(coord.row, coord2.row);
                let u_bound_r = max(coord.row, coord2.row);
                let l_bound_c = min(coord.col, coord2.col);
                let u_bound_c = max(coord.col, coord2.col);

                let exp_r = empty_rows
                    .iter()
                    .filter(|r| **r > l_bound_r as usize && **r < u_bound_r as usize)
                    .count();
                let exp_c = empty_cols
                    .iter()
                    .filter(|c| **c > l_bound_c as usize && **c < u_bound_c as usize)
                    .count();

                pairs.insert(
                    str,
                    coord2.row.abs_diff(coord.row)
                        + coord2.col.abs_diff(coord.col)
                        + (exp_r as u64 * mult)
                        - exp_r as u64
                        + (exp_c as u64 * mult)
                        - exp_c as u64,
                );
            }
        }
    }
    let total_length = pairs.iter().fold(0, |acc, pair| acc + pair.1);
    println!("total length is: {:?}", total_length);
}

// -------- initial approach below to use BFS totally unnecessary, but leaving for future reference if I want to implement again --------

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// struct GalaxyPair {
//     id: String,
//     steps: i32,
// }

// fn is_valid(point: &Coord, grid_dims: &Coord) -> bool {
//     if point.row >= 0 && point.row < grid_dims.row && point.col >= 0 && point.col < grid_dims.col {
//         return true;
//     } else {
//         return false;
//     }
// }

// fn bfs(
//     init: Coord,
//     grid_dims: &Coord,
//     final_grid: &Vec<Vec<char>>,
//     pairs: &mut HashSet<GalaxyPair>,
// ) {
//     let mut queue: VecDeque<Coord> = VecDeque::from(vec![init.clone()]);
//     let mut visited: HashMap<Coord, i32> = HashMap::from([(init.clone(), 0)]);
//     let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

//     while queue.len() > 0 {
//         let curr = queue.pop_front().unwrap();
//         for dir in dirs.iter() {
//             let next = Coord {
//                 row: curr.row + dir.0,
//                 col: curr.col + dir.1,
//             };
//             if is_valid(&next, grid_dims) {
//                 if !visited.contains_key(&next) {
//                     visited.insert(next.clone(), 1 + visited[&curr]);
//                     let mut x_vals = vec![init.row, next.row];
//                     let mut y_vals = vec![init.col, next.col];
//                     x_vals.sort();
//                     y_vals.sort();
//                     let pair = GalaxyPair {
//                         id: format!("{:?},{:?}", x_vals, y_vals),
//                         steps: 1 + visited[&curr],
//                     };
//                     if final_grid[next.row as usize][next.col as usize] == '#'
//                         && !pairs.contains(&pair)
//                     {
//                         pairs.insert(pair);
//                     } else {
//                         queue.push_back(next.clone());
//                     }
//                 }
//             }
//         }
//     }
// }
