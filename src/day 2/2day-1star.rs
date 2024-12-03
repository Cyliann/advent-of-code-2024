use std::io::{self, BufRead};

fn main() {
    let reports = read_input();
    let mut safe_reports = 0;

    for report in reports {
        safe_reports += parse_report(report)
    }

    print!("{:?}", safe_reports);
}

fn parse_report(report: Vec<i32>) -> i32 {
    let rising = report[0] < report[1];

    for (i, num) in report.iter().enumerate() {
        if let Some(next) = report.get(i + 1) {
            let order = if rising { next > num } else { next < num };

            if num == next || (num - next).abs() > 3 || !order {
                return 0;
            }
        } else {
            break;
        }
    }
    return 1;
}

fn read_input() -> Vec<Vec<i32>> {
    // Read input from the standard input line by line
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                break;
            }
            let parts = line
                .split(" ")
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();

            reports.push(parts);
        } else {
            eprintln!("Error reading line.");
        }
    }
    return reports;
}
