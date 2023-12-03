use crate::utils::read_lines;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
    val: Option<char>,
}

#[allow(dead_code)]
pub fn main() {
    let raw_lines = read_lines("src/day03/input.txt");
    let grid = raw_lines
        .unwrap()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // part 1
    let mut full_num_buffer: Vec<char> = vec![];
    let mut full_num_is_adj: bool = false;
    let mut part_nums: Vec<i32> = vec![];

    // part 2
    let mut gears: HashMap<String, Vec<i32>> = HashMap::new();
    let mut char_is_adj = false;
    let mut adj_coord = Coord {
        x: -1,
        y: -1,
        val: None,
    };

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, item)| {
            let coord = Coord {
                x: x as i32,
                y: y as i32,
                val: Some(*item),
            };
            // add number to buffer if digit detected
            if item.is_alphanumeric() {
                full_num_buffer.push(*item);
                let temp_adj_coord;
                (char_is_adj, temp_adj_coord) = is_symbol_adj(&grid, coord);
                if char_is_adj && !full_num_is_adj {
                    full_num_is_adj = true;
                    adj_coord = temp_adj_coord;
                }
            // if not digit, check if buffer is full and was adjacent to a symbol. if not, clear buffer and reset flags
            } else {
                if full_num_is_adj {
                    let num = full_num_buffer
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    if adj_coord.val.unwrap_or('0') == '*' {
                        let unique_pos = [
                            String::from("x:"),
                            adj_coord.x.to_string(),
                            String::from("y:"),
                            adj_coord.y.to_string(),
                        ]
                        .join("");
                        let gear = gears.get_mut(unique_pos.as_str());
                        match gear {
                            Some(refs) => {
                                refs.push(num);
                            }
                            None => {
                                gears.insert(unique_pos, vec![num]);
                            }
                        }
                    }
                    part_nums.push(num);
                }
                full_num_buffer.clear();
                full_num_is_adj = false;
                char_is_adj = false;
                adj_coord = Coord {
                    x: -1,
                    y: -1,
                    val: None,
                };
            }
        })
    });

    let gears_exactly_2: Vec<&Vec<i32>> = gears
        .iter()
        .filter_map(|(_, v)| match v.len() {
            2 => Some(v),
            _ => None,
        })
        .collect();

    let total = gears_exactly_2.iter().fold(0, |acc, gears| {
        acc + gears.iter().fold(1, |acc, part_nos| acc * part_nos)
    });

    println!(
        "sum of part nums: {:?}",
        part_nums.iter().fold(0, |acc, x| acc + x)
    );
    println!("sum of gear ratios: {:?}", total);
}

fn is_symbol_adj(grid: &Vec<Vec<char>>, coord: Coord) -> (bool, Coord) {
    for y in [-1, 0, 1].iter() {
        for x in [-1, 0, 1].iter() {
            // ensure coord is within grid bounds and isn't checking itself
            if (coord.y + y) >= 0
                && (coord.y + y) < grid.len() as i32
                && (coord.x + x) >= 0
                && (coord.x + x) < grid[0].len() as i32
                && (y != &0 || x != &0)
            {
                let new_y = (coord.y + y) as usize;
                let new_x = (coord.x + x) as usize;
                if !grid[new_y][new_x].is_alphanumeric() && grid[new_y][new_x] != '.' {
                    return (
                        true,
                        Coord {
                            x: new_x as i32,
                            y: new_y as i32,
                            val: Some(grid[new_y][new_x]),
                        },
                    );
                }
            }
        }
    }
    return (
        false,
        Coord {
            x: -1,
            y: -1,
            val: None,
        },
    );
}
