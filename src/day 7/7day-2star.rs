use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use std::time::Instant;
use std::{
    fs::read_to_string,
    sync::{Arc, Mutex},
    u64,
};

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Concatenation,
}

fn main() {
    let now = Instant::now();
    let (results, sets_of_numbers) = read_input("./src/day 7/input.txt");
    let possible_operations = vec![Operation::Sum, Operation::Product, Operation::Concatenation];
    let sum = Arc::new(Mutex::new(0));

    results
        .par_iter()
        .zip_eq(sets_of_numbers.par_iter())
        .for_each(|(result, numbers)| {
            if permutate(&possible_operations, result, numbers) {
                let mut num = sum.lock().unwrap();
                *num += result;
            }
        });

    println!("Sum is: {}", sum.lock().unwrap());
    println!("Took: {:.2?}", now.elapsed());
}

fn permutate(possible_operations: &Vec<Operation>, result: &u64, numbers: &Vec<u64>) -> bool {
    // permutation with repetition
    for operations in repeat_n(possible_operations, numbers.len() - 1).multi_cartesian_product() {
        let mut test_result: u64 = numbers[0];

        for (number, operation) in numbers[1..].iter().zip(operations.iter()) {
            if test_result > *result {
                return false;
            }
            match operation {
                // both sum and product can overflow, therefore the checked mul and add
                Operation::Sum => {
                    test_result = match test_result.checked_mul(*number) {
                        Some(value) => value,
                        None => return false,
                    }
                }
                Operation::Product => {
                    test_result = match test_result.checked_add(*number) {
                        Some(value) => value,
                        None => return false,
                    }
                }
                Operation::Concatenation => {
                    // we start from test_numbers[1], so we have to offset i by 1
                    if let Ok(concatenated) = format!("{}{}", test_result, number).parse::<u64>() {
                        test_result = concatenated;
                    } else {
                        return false;
                    };
                }
            }
        }
        if test_result == *result {
            return true;
        };
    }
    return false;
}

fn read_input(filename: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut results: Vec<u64> = Vec::new();
    let mut numbers: Vec<Vec<u64>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let capture = line.split(":").collect::<Vec<&str>>();
        let result = capture[0].parse::<u64>().unwrap();
        let rest = capture[1].trim();

        results.push(result);
        numbers.push(
            rest.split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    return (results, numbers);
}
