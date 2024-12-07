use itertools::{repeat_n, Itertools};
use std::fs::read_to_string;

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
}
fn main() {
    let (results, sets_of_numbers) = read_input("./src/day 7/input.txt");
    let possible_operations = [Operation::Sum, Operation::Product];
    let mut sum = 0;

    for (result, numbers) in results.iter().zip(sets_of_numbers.iter()) {
        // permutation with repetition
        for operations in
            repeat_n(&possible_operations, numbers.len() - 1).multi_cartesian_product()
        {
            let mut test_result: i64 = numbers[0];

            for (number, operation) in numbers[1..].iter().zip(operations.iter()) {
                match operation {
                    Operation::Sum => test_result += number,
                    Operation::Product => test_result *= number,
                }
            }
            if test_result == *result {
                sum += result;
                break;
            };
        }
    }

    println!("Sum is: {}", sum);
}

fn read_input(filename: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let mut results: Vec<i64> = Vec::new();
    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let capture = line.split(":").collect::<Vec<&str>>();
        let result = capture[0].parse::<i64>().unwrap();
        let rest = capture[1].trim();

        results.push(result);
        numbers.push(
            rest.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }

    return (results, numbers);
}
