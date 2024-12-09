// in the whole code -1 represents an empty space as it's not a valid digit and vector expects a
// number
fn main() {
    let mut blocks = read_input("./src/day 9/input.txt");

    blocks = move_blocks(blocks);
    assert!(!blocks.contains(&-1));

    let answer = calculate_checksum(&blocks);
    println!("Answer is: {}", answer);
}

fn calculate_checksum(blocks: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    for (i, id) in blocks.iter().enumerate() {
        sum = sum.checked_add(id * i as i64).unwrap();
    }
    return sum;
}

fn move_blocks(mut blocks: Vec<i64>) -> Vec<i64> {
    for i in 0..blocks.len() {
        if let Some(_) = blocks.get(i) {
            if blocks[i] != -1 {
                continue;
            }

            let mut last = blocks.pop().unwrap();
            // make sure we're not swapping empty space with empty space
            while last == -1 {
                last = blocks.pop().unwrap();
            }

            blocks[i] = last;
        } else {
            break;
        }
    }
    return blocks;
}

fn read_input(filename: &str) -> Vec<i64> {
    let mut input: Vec<i64> = vec![];
    let mut blocks: Vec<i64> = vec![];

    for digit in std::fs::read_to_string(filename).unwrap().chars() {
        if digit == '\n' {
            break;
        }
        input.push(digit.to_string().parse::<i64>().unwrap());
    }

    for (i, digit) in input.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*digit {
                blocks.push(i as i64 / 2);
            }
        } else {
            for _ in 0..*digit {
                blocks.push(-1);
            }
        }
    }

    return blocks;
}
