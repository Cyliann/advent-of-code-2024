use regex::Regex;

fn main() {
    let machines = read_input("./src/day 13/input.txt");
    let mut cost = 0;

    for (a, b, target) in machines {
        if let Some(solution) = solve_cramers(a, b, target) {
            cost += solution.0 * 3 + solution.1;
            println!("A: {} B: {}", solution.0, solution.1);
        }
    }
    println!("Cost: {}", cost);
}

fn solve_cramers(a: (u64, u64), b: (u64, u64), target: (u64, u64)) -> Option<(u64, u64)> {
    // Calculate determinant
    let det = (a.0 as f64) * (b.1 as f64) - (a.1 as f64) * (b.0 as f64);

    // If the determinant is zero, the system has no unique solution
    if det == 0.0 {
        return None;
    }

    // Apply Cramer's rule to find x and y
    let det_x = (target.0 as f64) * (b.1 as f64) - (target.1 as f64) * (b.0 as f64);
    let det_y = (a.0 as f64) * (target.1 as f64) - (a.1 as f64) * (target.0 as f64);

    let x = det_x / det;
    let y = det_y / det;

    let x: u64 = x.round() as u64;
    let y: u64 = y.round() as u64;
    if x * a.0 + y * b.0 == target.0 && x * a.1 + y * b.1 == target.1 {
        return Some((x, y));
    }
    return None;
}

fn read_input(path: &str) -> Vec<((u64, u64), (u64, u64), (u64, u64))> {
    let mut machines = vec![];
    let mut i = 0;
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut target = (0, 0);
    let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();

    for line in std::fs::read_to_string(path).unwrap().lines() {
        match i {
            0 => assign(&mut a, line, &re),
            1 => assign(&mut b, line, &re),
            2 => {
                assign(&mut target, line, &re);
                target.0 += 10000000000000;
                target.1 += 10000000000000;
                machines.push((a, b, target));
            }
            _ => {
                i = 0;
                continue;
            }
        }
        i += 1;
    }

    return machines;
}

fn assign(var: &mut (u64, u64), line: &str, re: &Regex) {
    let Some(capture) = re.captures(line) else {
        panic!("Regex didn't capture")
    };
    var.0 = capture[1].parse().unwrap();
    var.1 = capture[2].parse().unwrap();
}
