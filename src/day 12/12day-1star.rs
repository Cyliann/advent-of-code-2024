use std::collections::VecDeque;

fn main() {
    let grid = read_input("./src/day 12/input.txt");
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut cost = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            cost += visit(x, y, &grid, &mut visited);
        }
    }
    println!("Cost: {}", cost);
}

fn visit(mut x: usize, mut y: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> u32 {
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = VecDeque::new();
    queue.push_front((x, y));

    while !queue.is_empty() {
        (x, y) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        area += 1;

        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let temp_x = x as i32 + dir.0;
            let temp_y = y as i32 + dir.1;
            if temp_y < 0 || temp_x < 0 {
                perimeter += 1;
                continue;
            }

            let Some(row) = grid.get(temp_y as usize) else {
                perimeter += 1;
                continue;
            };

            let Some(neigbor) = row.get(temp_x as usize) else {
                perimeter += 1;
                continue;
            };

            if *neigbor != grid[y][x] {
                perimeter += 1;
                continue;
            }

            if !visited[temp_y as usize][temp_x as usize] {
                // println!("Pushing {}", neigbor);
                queue.push_back((temp_x as usize, temp_y as usize));
            }
        }
    }
    // println!(
    //     "Region: {} Area: {} Perimeter: {}",
    //     grid[y][x], area, perimeter
    // );
    return area * perimeter;
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in std::fs::read_to_string(path).expect("Wrong path").lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    return grid;
}
