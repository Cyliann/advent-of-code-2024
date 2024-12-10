use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let map = read_input("./src/day 10/input.txt");
    let heads = find_heads(&map);
    let mut trail_scores = 0;
    let mut trail_ratings = 0;

    for head in heads {
        let (score, rating) = find_trails(head, &map);
        trail_scores += score;
        trail_ratings += rating;
    }

    println!("Score is: {}", trail_scores);
    println!("Rating is: {}", trail_ratings);
}

// depth first search
fn find_trails(head: (usize, usize), map: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut score = 0;
    let mut queue = VecDeque::new();
    let mut visited_ends = HashSet::new();
    let mut rating: u32 = 0; // we always have at least one way
    let mut prev_tile = (head.0, head.1, 0 as u32);

    queue.push_back(prev_tile); //push head onto the stack

    while let Some(tile) = queue.pop_front() {
        // if our tile is smaller than the previous one, it means that we started a new subroute
        // this also takes care of the first route, since 0 == 0
        if prev_tile.2 >= tile.2 {
            rating += 1;
        }
        prev_tile = tile;

        if tile.2 == 9 {
            if visited_ends.insert(tile) {
                score += 1;
            }
            continue;
        }
        let neighbors = find_neighbors(&tile, map);

        if neighbors.len() == 0 {
            rating -= 1;
            continue;
        }

        for neighbor in neighbors {
            queue.push_front(neighbor);
        }
    }

    // the path isn't valid. we need to reset rating.
    if score == 0 {
        rating -= 1;
    }

    return (score, rating);
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
                .map(|c| c.to_digit(10).unwrap_or(11)) //.expect("That's not a digit"))
                .collect::<Vec<u32>>(),
        );
    }

    return map;
}
