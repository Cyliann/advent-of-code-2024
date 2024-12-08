#![feature(let_chains)]
use std::{collections::HashSet, fs::read_to_string};

use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Antenna {
    letter: char,
    x: usize,
    y: usize,
}

fn main() {
    let grid = read_input("./src/day 8/input.txt");
    let antennas = find_all_antennas(&grid);
    let mut antinodes = HashSet::new();

    for antenna in antennas {
        for (i, paired_antenna) in grid.iter().flatten().enumerate() {
            if *paired_antenna != antenna.letter {
                continue;
            }

            let current_x = i % grid[0].len();
            let current_y = i / grid.len();

            if current_x == antenna.x && current_y == antenna.y {
                continue;
            }

            let distance = (
                current_x as i32 - antenna.x as i32,
                current_y as i32 - antenna.y as i32,
            );

            if let Some(new_antinodes) = find_antinodes(&antenna, distance, &grid) {
                for antinode in new_antinodes {
                    if !antinodes.contains(&antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    println!("Antinodes on map: {}", antinodes.len());
}

fn find_antinodes(
    antenna: &Antenna,
    distance: (i32, i32),
    grid: &Vec<Vec<char>>,
) -> Option<Vec<(usize, usize)>> {
    let mut antinodes: Vec<(usize, usize)> = vec![];
    let mut pos = (antenna.x, antenna.y);

    for distance in [distance, (-distance.0, -distance.1)] {
        loop {
            let new_x = pos.0 as i32 + distance.0;
            let new_y = pos.1 as i32 + distance.1;

            if new_x < 0 || new_y < 0 {
                break;
            }

            if let Some(antinode_row) = grid.get(new_y as usize)
                && let Some(_) = antinode_row.get(new_x as usize)
            {
                antinodes.push((new_x as usize, new_y as usize));
                pos = (new_x as usize, new_y as usize);
            } else {
                break;
            };
        }
    }
    return Some(antinodes);
}

fn find_all_antennas(grid: &Vec<Vec<char>>) -> Vec<Antenna> {
    let re = Regex::new(r"([a-zA-Z0-9{1}])").expect("Invalid regex");
    let mut antennas: Vec<Antenna> = vec![];

    for (i, row) in grid.iter().enumerate() {
        let row_string = &row.iter().collect::<String>();
        let row_antennas: Vec<Antenna> = re
            .find_iter(&row_string)
            .map(|m| Antenna {
                letter: m.as_str().chars().nth(0).unwrap(),
                x: m.start(),
                y: i,
            })
            .collect();

        antennas.extend(row_antennas);
    }
    return antennas;
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        grid.push(line.chars().collect());
    }

    return grid;
}
