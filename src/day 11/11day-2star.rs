use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let stones = read_input("./src/day 11/input.txt");

    let map = blink(stones, 75);
    let mut sum: u64 = 0;
    for val in map.values() {
        sum = sum + val;
    }
    println!("{:?}", sum);
}

fn blink(stones: Vec<u64>, num_of_blinks: u64) -> HashMap<u64, u64> {
    let mut memo = HashMap::new();
    let mut to_insert;
    let mut to_remove;

    for stone in stones.iter() {
        if let Some(val) = memo.get_mut(stone) {
            *val += 1 as u64;
        } else {
            memo.insert(*stone, 1 as u64);
        }
    }

    for j in 0..num_of_blinks {
        to_insert = vec![];
        to_remove = vec![];
        for (key, value) in memo.iter() {
            if *key == 0 {
                to_insert.push((1, *value));
                to_remove.push(*key);
                continue;
            }
            let length = key.checked_ilog10().unwrap() + 1;
            if length % 2 == 0 {
                let mask = (10 as u64).pow(length / 2);
                let left = *key / mask;
                let right = *key % mask;

                to_insert.push((left, *value));
                to_insert.push((right, *value));
                to_remove.push(*key);

                continue;
            }

            to_insert.push((*key * 2024, *value));
            to_remove.push(*key);
        }

        for key in to_remove.iter() {
            memo.remove(key);
        }
        for (key, value) in to_insert {
            if let Some(val) = memo.get_mut(&key) {
                *val += value;
                continue;
            }
            memo.insert(key, value);
        }
        println!("Blink: {}", j + 1);
    }
    return memo;
}

fn read_input(path: &str) -> Vec<u64> {
    let mut stones = vec![];

    for line in read_to_string(path).unwrap().lines() {
        stones.extend(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    return stones;
}

