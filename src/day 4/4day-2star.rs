use std::fs::read_to_string;
fn main() {
    let grid = read_input("./src/day 4/input.txt");
    let mut sum_of_words = 0;
    let words = ["MAS", "SAM"];

    let (rows, row_coords) = get_rows(&grid);
    let (columns, column_coords) = get_columns(&grid);
    let (diag1, diag1_coords) = get_diagonal_down(&grid);
    let (diag2, diag2_coords) = get_diagonal_up(&grid);

    let mut row_indices: Vec<usize> = vec![];
    let mut column_indices: Vec<usize> = vec![];
    let mut diag1_indices: Vec<usize> = vec![];
    let mut diag2_indices: Vec<usize> = vec![];

    for word in words {
        row_indices.extend(
            rows.clone()
                .match_indices(word)
                .into_iter()
                .map(|x| x.to_owned().0)
                .collect::<Vec<usize>>(),
        );

        column_indices.extend(
            columns
                .match_indices(word)
                .into_iter()
                .map(|x| x.to_owned().0)
                .collect::<Vec<usize>>(),
        );

        diag1_indices.extend(
            diag1
                .match_indices(word)
                .into_iter()
                .map(|x| x.to_owned().0)
                .collect::<Vec<usize>>(),
        );

        diag2_indices.extend(
            diag2
                .match_indices(word)
                .into_iter()
                .map(|x| x.to_owned().0)
                .collect::<Vec<usize>>(),
        );
    }
    // sum_of_words += match_rows_columns(row_indices, column_indices, &row_coords, &column_coords); // turns out I don't need to do this
    sum_of_words += match_diagonals(diag1_indices, diag2_indices, &diag1_coords, &diag2_coords);
    println!("{}", sum_of_words);
}

fn get_columns(grid: &Vec<Vec<char>>) -> (String, Vec<(usize, usize)>) {
    let mut output = String::new();
    let mut coords: Vec<(usize, usize)> = vec![];

    for column in 0..grid[0].len() {
        for row in 0..grid.len() {
            output.push_str(&grid[row][column].to_string());
            coords.push((column, row));
        }
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    return (output, coords);
}

fn get_rows(grid: &Vec<Vec<char>>) -> (String, Vec<(usize, usize)>) {
    let mut output = String::new();
    let mut coords: Vec<(usize, usize)> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            output.push_str(&column.to_string());
            coords.push((i, j));
        }
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    return (output, coords);
}

fn get_diagonal_down(grid: &Vec<Vec<char>>) -> (String, Vec<(usize, usize)>) {
    let mut output = String::new();
    let mut coords: Vec<(usize, usize)> = vec![];
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
            coords.push((column, row));
            row += 1;
            column += 1;
        }
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    starting_row = 0;
    starting_column = 1;

    while starting_column < size_x {
        let mut row = starting_row;
        let mut column = starting_column;
        while column < size_x {
            output.push_str(&grid[row][column].to_string());
            coords.push((column, row));
            row += 1;
            column += 1;
        }
        starting_column += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    return (output, coords);
}

fn get_diagonal_up(grid: &Vec<Vec<char>>) -> (String, Vec<(usize, usize)>) {
    let mut output = String::new();
    let mut coords: Vec<(usize, usize)> = vec![];
    let size_x = grid[0].len();
    let size_y = grid.len();

    let mut starting_row = 0;
    let mut starting_column = 0;

    while starting_row < size_y {
        let mut row = starting_row;
        let mut column = starting_column;
        loop {
            output.push_str(&grid[row][column].to_string());
            coords.push((column, row));
            if row == 0 {
                break;
            }
            row -= 1;
            column += 1;
        }
        starting_row += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    starting_row = size_y - 1;
    starting_column = 1;

    while starting_column < size_x {
        let mut row = starting_row;
        let mut column = starting_column;
        while column < size_x {
            output.push_str(&grid[row][column].to_string());
            coords.push((column, row));
            row -= 1;
            column += 1;
        }
        starting_column += 1;
        output.push_str(" "); // to separate rows from each other, so XM+AS=XM_AS instead of XMAS
        coords.push((0, 0));
    }
    return (output, coords);
}

fn match_rows_columns(
    row_indices: Vec<usize>,
    column_indices: Vec<usize>,
    row_coords: &Vec<(usize, usize)>,
    column_coords: &Vec<(usize, usize)>,
) -> usize {
    let row_coords = row_indices
        .iter()
        .map(|x| row_coords[x.to_owned() + 1])
        .collect::<Vec<(usize, usize)>>();

    let column_coords = column_indices
        .iter()
        .map(|x| column_coords[x.to_owned() + 1])
        .collect::<Vec<(usize, usize)>>();

    return row_coords
        .iter()
        .filter(|x| column_coords.contains(x))
        .count();
}

fn match_diagonals(
    diag1_indices: Vec<usize>,
    diag2_indices: Vec<usize>,
    diag1_coords: &Vec<(usize, usize)>,
    diag2_coords: &Vec<(usize, usize)>,
) -> usize {
    let diag1_coords = diag1_indices
        .iter()
        .map(|x| diag1_coords[x.to_owned() + 1])
        .collect::<Vec<(usize, usize)>>();

    let diag2_coords = diag2_indices
        .iter()
        .map(|x| diag2_coords[x.to_owned() + 1])
        .collect::<Vec<(usize, usize)>>();

    return diag1_coords
        .iter()
        .filter(|x| diag2_coords.contains(x))
        .count();
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        grid.push(line.chars().collect());
    }

    return grid;
}
