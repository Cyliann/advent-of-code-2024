use std::io::{self, BufRead};
fn main() {
    // Initialize two arrays to hold the input values
    let mut l_list = Vec::with_capacity(1000);
    let mut r_list = Vec::with_capacity(1000);

    (l_list, r_list) = read_input(l_list, r_list);

    let mut similarity: i32 = 0;

    for li in l_list.iter() {
        similarity += li * r_list.iter().filter(|item| li == *item).count() as i32;
    }

    print!("{:?}", similarity)
}

fn read_input(mut l_list: Vec<i32>, mut r_list: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // Read input from the standard input line by line
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    for _ in 0..1000 {
        if let Some(Ok(line)) = lines.next() {
            // Split the line by three spaces
            let parts: Vec<&str> = line.split("   ").collect();
            if parts.len() == 2 {
                if let (Ok(l), Ok(r)) = (
                    parts[0].trim().parse::<i32>(),
                    parts[1].trim().parse::<i32>(),
                ) {
                    l_list.push(l);
                    r_list.push(r);
                } else {
                    eprintln!("Error parsing integers in line: {}", line);
                }
            } else {
                eprintln!("Error: Line does not contain exactly two integers separated by three spaces: {}", line);
            }
        } else {
            eprintln!("Error reading line.");
        }
    }
    return (l_list, r_list);
}
