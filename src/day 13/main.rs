use eqsolver::multivariable::MultiVarNewtonFD;
use nalgebra::{vector, ComplexField, Vector2};
use regex::Regex;

fn main() {
    let machines = read_input("./src/day 13/input.txt");
    let mut cost = 0;

    for (a, b, target) in machines {
        if let Some(solution) = solve(a, b, target) {
            cost += solution.0 * 3 + solution.1;
            println!("A: {} B: {}", solution.0, solution.1);
        }
    }
    println!("Cost: {}", cost);
}

fn solve(a: (u64, u64), b: (u64, u64), target: (u64, u64)) -> Option<(u64, u64)> {
    let f = |v: Vector2<f64>| {
        vector![
            v[0] * a.0 as f64 + v[1] * b.0 as f64 - target.0 as f64,
            v[0] * a.1 as f64 + v[1] * b.1 as f64 - target.1 as f64
        ]
    };

    let solution = MultiVarNewtonFD::new(f).solve(vector![1., 1.]).unwrap();

    let a_sol: u64 = solution[0].round() as u64;
    let b_sol: u64 = solution[1].round() as u64;
    if a_sol * a.0 + b_sol * b.0 == target.0 && a_sol * a.1 + b_sol * b.1 == target.1 {
        return Some((a_sol, b_sol));
    }
    return None;
}

fn solve_cramers(a: (u64, u64), b: (u64, u64), target: (u64, u64)) -> Option<(u64, u64)> {
    if target.0 * b.1 < b.0 * target.1
        || a.0 * b.1 <= b.0 * a.1
        || a.0 * target.1 < target.0 * a.1
        || a.0 * b.1 <= b.0 * a.1
    {
        return None;
    }
    let sol_a = (target.0 * b.1 - b.0 * target.1) / (a.0 * b.1 - b.0 * a.1);
    let sol_b = (a.0 * target.1 - target.0 * a.1) / (a.0 * b.1 - b.0 * a.1);
    dbg!(sol_a);
    dbg!(sol_b);
    dbg!(target);

    return Some((sol_a, sol_b));
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
