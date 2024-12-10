use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let map = read_input("./src/day 10/input.txt");
    let heads = find_heads(&map);
    let mut trail_scores = 0;

    for head in heads {
        trail_scores += find_trails(head, &map);
    }

    println!("Score is: {}", trail_scores);
}

// depth first search
fn find_trails(head: (usize, usize), map: &Vec<Vec<u32>>) -> u32 {
    let mut score = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((head.0, head.1, 0 as u32));

    while let Some(tile) = queue.pop_front() {
        if tile.2 == 9 {
            score += 1;
            continue;
        }

        for neighbor in find_neighbors(&tile, map) {
            if visited.insert(neighbor) {
                queue.push_front(neighbor);
            }
        }
    }

    return score;
}

fn find_neighbors(root: &(usize, usize, u32), map: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut neighbors = vec![];

    for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let x = root.0 as i32 + direction.0;
        let y = root.1 as i32 + direction.1;

        if x < 0 || y < 0 {
            continue;
        }
        if let Some(row) = map.get(y as usize) {
            if let Some(tile) = row.get(x as usize) {
                if *tile as i32 - root.2 as i32 == 1 {
                    neighbors.push((x as usize, y as usize, *tile));
                }
            }
        }
    }
    return neighbors;
}

fn find_heads(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut heads = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if *item == 0 {
                heads.push((x, y));
            }
        }
    }

    return heads;
}

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        map.push(
            line.chars()
                .map(|c| c.to_digit(10).expect("That's not a digit"))
                .collect::<Vec<u32>>(),
        );
    }

    return map;
}
