use std::collections::HashSet;

// in the whole code -1 represents an empty space as it's not a valid digit and vector expects a
// number
fn main() {
    let mut block_info = read_input("./src/day 9/input.txt");

    block_info = move_blocks(block_info);
    let blocks = convert_to_blocks(block_info);

    let answer = calculate_checksum(&blocks);
    println!("Answer is: {}", answer);
}

fn convert_to_blocks(block_info: Vec<(i64, usize)>) -> Vec<i64> {
    let mut blocks = vec![];
    for block in block_info {
        blocks.extend(vec![block.0; block.1])
    }

    return blocks;
}

fn calculate_checksum(blocks: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    for (i, id) in blocks.iter().enumerate() {
        if *id == -1 {
            continue;
        }
        sum = sum.checked_add(id * i as i64).unwrap();
    }
    return sum;
}

fn move_blocks(mut block_info: Vec<(i64, usize)>) -> Vec<(i64, usize)> {
    let mut checked = HashSet::new();
    let mut i = block_info.len() - 1;

    while i > 0 {
        let block = block_info[i];
        if block.0 == -1 || checked.contains(&block.0) {
            i -= 1;
            continue;
        }
        checked.insert(block.0);

        let mut insert_index = 0;
        let mut enough_space = false;

        for (j, empty_block) in block_info.iter().enumerate() {
            // find first empty block or exit if it's furthe than original place
            if empty_block.0 != -1 || j > i {
                continue;
            }

            if empty_block.1 >= block.1 {
                enough_space = true;
                insert_index = j;
                break;
            }
        }

        if !enough_space {
            i -= 1;
            continue;
        }

        block_info[insert_index].1 -= block.1;
        block_info[i] = (-1, block.1);
        block_info.insert(insert_index, block);
        // to mitigate inserting, we're not decreasing i (same as i+1-1)
    }

    return block_info;
}

fn read_input(filename: &str) -> Vec<(i64, usize)> {
    let mut input: Vec<i64> = vec![];
    let mut block_info: Vec<(i64, usize)> = vec![];

    for digit in std::fs::read_to_string(filename).unwrap().chars() {
        if digit == '\n' {
            break;
        }
        input.push(digit.to_string().parse::<i64>().unwrap());
    }

    for (i, digit) in input.iter().enumerate() {
        if i % 2 == 0 {
            block_info.push((i as i64 / 2, *digit as usize))
        } else {
            block_info.push((-1, *digit as usize))
        }
    }

    return block_info;
}
