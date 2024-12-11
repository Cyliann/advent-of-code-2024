use std::fs::read_to_string;

fn main() {
    let mut stones = read_input("./src/day 11/input.txt");

    stones = blink(stones, 25);
    println!("{:?}", stones.len());
}

fn blink(mut stones: Vec<u64>, num_of_blinks: u64) -> Vec<u64> {
    for _ in 0..num_of_blinks {
        let mut temp = vec![];

        for stone in stones.iter_mut() {
            if *stone == 0 {
                *stone = 1;

                continue;
            }

            let length = stone.checked_ilog10().unwrap() + 1;
            if length % 2 == 0 {
                let mask = (10 as u64).pow(length / 2);
                let left = *stone / mask;
                let right = *stone % mask;

                *stone = right;
                temp.push(left);

                continue;
            }

            *stone = *stone * 2024;
        }

        for item in temp {
            stones.push(item);
        }
    }
    return stones;
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

