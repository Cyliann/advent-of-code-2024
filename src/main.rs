use std::fs::read_to_string;

fn main() {
    let (rules, mut updates) = read_input("./src/day 5/input.txt");

    updates.retain(|update| is_valid(update, &rules));

    dbg!(updates);
}

fn is_valid(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (a, b) in rules {
        // println!("{}, {}", a, b);
        if let Some(index_b) = update.iter().position(|x| x == b) {
            if let Some(index_a) = update.iter().position(|x| x == a) {
                return index_a < index_b;
            }
        }
    }
    return false;
}

fn read_input(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut before_empty_line = true;

    for line in read_to_string(filename).unwrap().lines() {
        if line.to_string().is_empty() {
            before_empty_line = false;
            continue;
        }
        if before_empty_line {
            let capture = line.split("|").collect::<Vec<&str>>();
            let a = capture[0];
            let b = capture[1];
            rules.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
        } else {
            updates.push(
                line.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    return (rules, updates);
}
