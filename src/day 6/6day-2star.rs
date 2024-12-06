use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn rotate(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
        }
    }

    fn walk(&mut self, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut visited: Vec<(usize, usize)> = vec![];
        loop {
            let in_front = in_front(&self);
            visited.push(self.pos);

            if out_of_bounds(&in_front, map[0].len(), map.len()) {
                break;
            }

            // there's an obstacle
            if map[in_front.0 as usize][in_front.1 as usize] == '#' {
                self.rotate();
                continue;
            }

            // there's nothing in front. self can go forward
            self.advance(&in_front);
        }

        return visited;
    }

    fn advance(&mut self, in_front: &(i32, i32)) {
        self.pos = (in_front.0 as usize, in_front.1 as usize);
    }

    fn is_stuck(&mut self, map: &mut Vec<Vec<char>>) -> bool {
        let mut states = HashSet::new();

        loop {
            if states.contains(&(self.pos, self.dir)) {
                return true;
            }
            states.insert((self.pos, self.dir));
            let in_front = in_front(&self);

            if out_of_bounds(&in_front, map[0].len(), map.len()) {
                break;
            }

            // there's an obstacle
            if map[in_front.0 as usize][in_front.1 as usize] == '#' {
                self.rotate();
                continue;
            }

            // there's nothing in front. self can go forward
            self.advance(&in_front);
        }

        return false;
    }
}

fn main() {
    let map = read_input("./src/day 6/input.txt");
    let guard = get_guard(&map);
    let mut sum = 0;
    // let path = guard.clone().walk(&map);

    for row in 0..map.len() {
        for column in 0..map[row].len() {
            // if guard is standing there
            if (row, column) == guard.pos {
                continue;
            }

            let mut test_map = map.clone();
            let mut test_guard = guard.clone();
            test_map[row][column] = '#';

            if test_guard.is_stuck(&mut test_map) {
                sum += 1;
            }
        }
    }

    println!("Number of stuck loops: {}", sum);
}

fn get_guard(map: &Vec<Vec<char>>) -> Guard {
    let guard_absolutte_position = map
        .iter()
        .flatten()
        .position(|&guard| guard == '^' || guard == 'v' || guard == '<' || guard == '>')
        .unwrap();

    let guard_position = (
        guard_absolutte_position / map[0].len(),
        guard_absolutte_position % map[0].len(),
    );

    let direction = match map[guard_position.0][guard_position.1] {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '>' => Direction::Right,
        '<' => Direction::Left,
        _ => panic!("Direction is wrong"),
    };

    return Guard {
        pos: guard_position,
        dir: direction,
    };
}

fn in_front(guard: &Guard) -> (i32, i32) {
    let offset = match guard.dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
    };

    return (guard.pos.0 as i32 + offset.0, guard.pos.1 as i32 + offset.1);
}

fn out_of_bounds(in_front: &(i32, i32), x_size: usize, y_size: usize) -> bool {
    return in_front.0 < 0
        || in_front.0 >= x_size as i32
        || in_front.1 < 0
        || in_front.1 >= y_size as i32;
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        grid.push(line.chars().collect());
    }

    return grid;
}
