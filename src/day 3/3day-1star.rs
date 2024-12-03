use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_input("./src/day 3/input.txt");
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Invalid regex");
    let mut sum = 0;

    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    println!("Sum is {}", sum);
}

fn read_input(filename: &str) -> String {
    let mut result = String::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push_str(&line.to_string());
    }

    return result;
}
