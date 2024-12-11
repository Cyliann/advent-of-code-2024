use std::fs::read_to_string;

fn main() {
    let mut stones = read_input("./src/day 11/input.txt");
    dbg!(&stones);

    stones = blink(stones, 25);
    println!("{:?}", stones.len());
}

fn blink(mut stones: Vec<u64>, num_of_blinks: u64) -> Vec<u64> {
    for _ in 0..num_of_blinks {
        let mut i = 0;

        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;

                i += 1;
                continue;
            }

            if stone.to_string().len() % 2 == 0 {
                let whole = stone.to_string();
                let left = &whole[..whole.len() / 2];
                let right = &whole[whole.len() / 2..];

                stones.remove(i);
                stones.insert(i, right.parse().unwrap());
                stones.insert(i, left.parse().unwrap());

                i += 2;
                continue;
            }

            stones[i] = stone * 2024;
            i += 1;
        }
    }
    return stones;
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
