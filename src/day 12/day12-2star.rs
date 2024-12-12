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
    let mut corners = 0; // there is as many sides as there is corners, so we can just count the
                         // number of corners
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_front((x, y));

    while !queue.is_empty() {
        (x, y) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }
        let mut temp_sides = [false; 4];
        visited[y][x] = true;
        area += 1;

        for (i, dir) in directions.iter().enumerate() {
            let temp_x = x as i32 + dir.0;
            let temp_y = y as i32 + dir.1;
            if temp_y < 0 || temp_x < 0 {
                temp_sides[i] = true;
                continue;
            }

            let Some(row) = grid.get(temp_y as usize) else {
                temp_sides[i] = true;
                continue;
            };

            let Some(neigbor) = row.get(temp_x as usize) else {
                temp_sides[i] = true;
                continue;
            };

            if *neigbor != grid[y][x] {
                temp_sides[i] = true;
                continue;
            }

            if !visited[temp_y as usize][temp_x as usize] {
                // println!("Pushing {}", neigbor);
                queue.push_back((temp_x as usize, temp_y as usize));
            }
        }
        // iterate over walls and see which are next to each other (create corners)
        let mut temp_corners = 0;
        for (i, side) in temp_sides.iter().enumerate() {
            if *side && temp_sides[(i + 1) % 4] {
                temp_corners += 1;
            }
        }
        temp_corners += check_internal_corners(y, x, &temp_sides, &directions, &grid);
        corners += temp_corners;
    }
    println!("Region: {} Area: {} Sides: {}", grid[y][x], area, corners);
    return area * corners;
}

fn check_internal_corners(
    y: usize,
    x: usize,
    temp_sides: &[bool; 4],
    directions: &[(i32, i32); 4],
    grid: &Vec<Vec<char>>,
) -> u32 {
    // if there is no borders on the sides, but the diagonal is different, that means we're on the
    // inner corner. We can safely acces the grid, because if it was the border index (len or 0)
    // the would be a border on the sides
    let mut corners = 0;
    for (i, side) in temp_sides.iter().enumerate() {
        if !side && !temp_sides[(i + 1) % 4] {
            let (temp_x, temp_y) = (
                directions[i].0 + directions[(i + 1) % 4].0 + x as i32,
                directions[i].1 + directions[(i + 1) % 4].1 + y as i32,
            );
            if grid[temp_y as usize][temp_x as usize] != grid[y][x] {
                corners += 1;
            }
        }
    }
    return corners;
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in std::fs::read_to_string(path).expect("Wrong path").lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    return grid;
}
