use std::io::{self, BufRead};

fn main() {
    let reports = read_input();
    let mut safe_reports_iter: Vec<Vec<i32>> = vec![];
    let mut safe_reports_2: Vec<Vec<i32>> = vec![];

    for report in reports {
        if let Some(report) = parse_report_iter(report.clone(), false) {
            safe_reports_iter.push(report)
        }

        if let Some(report) = parse_report_3(report.clone(), false) {
            safe_reports_2.push(report)
        }
    }

    let diff: Vec<Vec<i32>> = safe_reports_iter.into_iter().filter(|item| !safe_reports_2.contains(item)).collect();

    for report in diff {
        print!("{:?}", debug_report_2(report, false));
    }
    // print!("{:?}", diff);
}

fn parse_report_iter(report: Vec<i32>, dampener_used: bool) -> Option<Vec<i32>> {
    let rising = report[0] < report[1];

    for (i, num) in report.iter().enumerate() {
        if let Some(next) = report.get(i + 1) {
            let order = if rising { next > num } else { next < num };
            if (num - next).abs() < 1 || (num - next).abs() > 3 || !order {
                if dampener_used {
                    return None;
                }
                for (i, _)  in report.iter().enumerate() {
                    let mut report_clone = report.clone();
                    report_clone.remove(i);

                    if let Some(_) = parse_report_iter(report_clone, true) {
                        return Some(report);
                    }
                }
                return None;
            }
        } else {
            break;
        }
    }
    return Some(report);
}

fn parse_report_3(report: Vec<i32>, dampener_used: bool) -> Option<Vec<i32>> {
    let rising = report[0] < report[1];

    for (i, num) in report.iter().enumerate() {
        if let Some(next) = report.get(i + 1) {
            let order = if rising { next > num } else { next < num };
            if (num - next).abs() < 1 || (num - next).abs() > 3 || !order {
                if dampener_used {
                    return None;
                }

                let mut report_zero = report.clone();
                report_zero.remove(i);
                if let Some(_) = parse_report_3(report_zero, true) {
                    return Some(report);
                }

                let mut report_one = report.clone();
                report_one.remove(i+1);
                if let Some(_) = parse_report_3(report_one, true) {
                    return Some(report);
                }

                if i == 0 { return None; }
                let mut report_minus_one = report.clone();
                report_minus_one.remove(i-1);
                if let Some(_) = parse_report_3(report_minus_one, true) {
                    return Some(report);
                }
                return None;
            }
        } else {
            break;
        }
    }
    return Some(report);
}

fn debug_report_2(report: Vec<i32>, dampener_used: bool) -> Option<Vec<i32>> {
    let rising = report[0] < report[1];
    dbg!(&report);

    for (i, num) in report.iter().enumerate() {
        if let Some(next) = report.get(i + 1) {
            let order = if rising { next > num } else { next < num };
            dbg!(num);
            dbg!(next);
            dbg!(order);
            dbg!(dampener_used);
            dbg!(i);
            if (num - next).abs() < 1 || (num - next).abs() > 3 || !order {
                if dampener_used {
                    return None;
                }
                println!("Using dampener");
                let mut report_zero = report.clone();
                report_zero.remove(i);
                let mut report_one = report.clone();
                report_one.remove(i+1);
                if let Some(_) = debug_report_2(report_zero, true) {
                    return Some(report);
                }
                if let Some(_) = debug_report_2(report_one, true) {
                    return Some(report);
                }
                return None;
            }
        } else {
            break;
        }
    }
    return Some(report);
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
