use std::fs::read_to_string;
fn main() {
    let grid = read_input("./src/day 4/input.txt");
    let mut sum_of_words = 0;
    let words = ["XMAS", "SAMX"];

    for word in words {
        sum_of_words += get_rows(&grid).matches(word).count();
        sum_of_words += get_columns(&grid).matches(word).count();
        sum_of_words += get_diagonal_down(&grid).matches(word).count();
        sum_of_words += get_diagonal_up(&grid).matches(word).count();
    }

    println!("Words: {}", sum_of_words);
}

fn get_columns(grid: &Vec<Vec<char>>) -> String {
    let mut output = String::new();

    for column in 0..grid[0].len() {
        output.push_str(&grid.iter().map(|row| row[column]).collect::<String>());
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    return output;
}

fn get_rows(grid: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    for row in grid.iter() {
        output.push_str(&row.iter().collect::<String>());
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    return output;
}

fn get_diagonal_down(grid: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut starting_row = size_y;
    let mut starting_column = 0;

    while starting_row > 0 {
        starting_row -= 1;
        let mut row = starting_row;
        let mut column = starting_column;
        while row < size_y {
            output.push_str(&grid[row][column].to_string());
            row += 1;
            column += 1;
        }
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    starting_row = 0;
    starting_column = 1;

    while starting_column < size_x {
        let mut row = starting_row;
        let mut column = starting_column;
        while column < size_x {
            output.push_str(&grid[row][column].to_string());
            row += 1;
            column += 1;
        }
        starting_column += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    return output;
}

fn get_diagonal_up(grid: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut starting_row = 0;
    let mut starting_column = 0;

    while starting_row < size_y {
        let mut row = starting_row;
        let mut column = starting_column;
        loop {
            output.push_str(&grid[row][column].to_string());
            if row == 0 {
                break;
            }
            row -= 1;
            column += 1;
        }
        starting_row += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    starting_row = size_y - 1;
    starting_column = 1;

    while starting_column < size_x {
        let mut row = starting_row;
        let mut column = starting_column;
        while column < size_x {
            output.push_str(&grid[row][column].to_string());
            row -= 1;
            column += 1;
        }
        starting_column += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
    }
    return output;
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        grid.push(line.chars().collect());
    }

    return grid;
}
