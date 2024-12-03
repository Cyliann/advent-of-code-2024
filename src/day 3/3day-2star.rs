use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_input("./src/day 3/input.txt");
    let re =
        Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)|(do\(\))|(don't\(\))").expect("Invalid regex");
    let mut sum = 0;
    let mut todo = true;

    for (_, [capture]) in re.captures_iter(&input).map(|c| c.extract()) {
        match capture {
            "do()" => todo = true,
            "don't()" => todo = false,
            _ => {
                if todo {
                    sum += multiply(capture)
                }
            }
        }
    }

    println!("Sum is {}", sum);
}

fn multiply(capture: &str) -> i32 {
    let parts = capture.split(",").collect::<Vec<&str>>();
    let a = parts[0].parse::<i32>().unwrap();
    let b = parts[1].parse::<i32>().unwrap();

    return a * b;
}

fn read_input(filename: &str) -> String {
    let mut result = String::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push_str(&line.to_string());
    }

    return result;
}
