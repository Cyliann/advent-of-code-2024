use std::fs::read_to_string;

fn main() {
    let mut stones = read_input("/home/cylian/code/rust/advent-of-code-2024/src/day 11/input.txt");

    stones = blink(stones, 75);
    println!("{:?}", stones.len());
}

fn blink(mut stones: Vec<u64>, num_of_blinks: u64) -> Vec<u64> {
    for j in 0..num_of_blinks {
        let mut new_vec = stones.clone();
        // let mut i = 0;
        // let len = stones.len();

        for (i, stone) in stones.iter().enumerate() {
            // let stone = stones[i];
            if *stone == 0 {
                new_vec[i] = 1;

                // i += 1;
                continue;
            }

            let length = get_length(*stone);
            if length % 2 == 0 {
                let mask = (10 as u64).pow(length / 2);
                let left = stone / mask;
                let right = stone % mask;

                new_vec[i] = left;
                new_vec.push(right);

                // i += 1;
                continue;
            }

            new_vec[i] = stone * 2024;
            // i += 1;
        }
        stones = new_vec;
        println!("Blink: {} Number of stones {:?}", j + 1, stones.len());
    }
    return stones;
}

// faster than parsing to string
fn get_length(mut number: u64) -> u32 {
    let mut length = 1;
    if number >= 10000000000000000 {
        length += 16;
        number /= 10000000000000000;
    }
    if number >= 100000000 {
        length += 8;
        number /= 100000000;
    }
    if number >= 10000 {
        length += 4;
        number /= 10000;
    }
    if number >= 100 {
        length += 2;
        number /= 100;
    }
    if number >= 10 {
        length += 1;
    }
    return length;
}

fn read_input(path: &str) -> Vec<u64> {
    let mut stones = vec![];

    for line in read_to_string(path).unwrap().lines() {
        stones.extend(
            line.split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    return stones;
}
