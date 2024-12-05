use std::fs::read_to_string;

fn main() {
    let (rules, mut updates) = read_input("./src/day 5/input.txt");

    updates.retain(|update| !is_valid(update, &rules)); //get only invalid updates
    let fixed_updates: Vec<Vec<i32>> = updates
        .iter_mut()
        .map(|update| fix_validity(update, &rules))
        .collect(); //get only invalid updates

    let sum: i32 = fixed_updates
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum();

    println!("Sum is: {}", sum)
}

fn is_valid(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for tuple in rules.iter() {
        let (a, b) = tuple;

        if let Some(index_b) = update.iter().position(|x| x == b) {
            if let Some(index_a) = update.iter().position(|x| x == a) {
                if index_a > index_b {
                    return false;
                };
            }
        }
    }
    return true;
}

fn fix_validity(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    for tuple in rules.iter() {
        let (a, b) = tuple;

        if let Some(index_b) = update.iter().position(|x| x == b) {
            if let Some(index_a) = update.iter().position(|x| x == a) {
                if index_a > index_b {
                    update.remove(index_a);
                    update.insert(index_b, *a);
                };
            }
        }
    }
    // sometimes it's not fixed after one pass-through, so we recurse until it's valid
    if is_valid(update, rules) {
        return update.to_owned();
    }
    return fix_validity(update, rules);
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
            let a = capture[0].parse::<i32>().unwrap();
            let b = capture[1].parse::<i32>().unwrap();
            rules.push((a, b));
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
