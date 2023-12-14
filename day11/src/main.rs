use std::{fs::read_to_string, cmp};

fn find_all_pairs(grid: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let (height, width) = (grid.len(), grid[0].len());
    let mut galaxies = Vec::new();
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == '#' {
                galaxies.push((r, c));
            }
        }
    }

    let mut pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }

    pairs
}

fn find_empty_rows_and_cols(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let (height, width) = (grid.len(), grid[0].len());
    
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for r in 0..height {
        let mut empty = true;
        for c in 0..width {
            if grid[r][c] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_rows.push(r);
        }
    }

    for c in 0..width {
        let mut empty = true;
        for r in 0..height {
            if grid[r][c] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(c);
        }
    }

    (empty_rows, empty_cols)
}

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");
    let grid = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let all_pairs = find_all_pairs(&grid);
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&grid);

    let mut total_distance = 0;
    for ((r1, c1), (r2, c2)) in all_pairs {
        let mut abs_distance = (r1 as i32 - r2 as i32).abs() + (c1 as i32 - c2 as i32).abs();

        for r in &empty_rows {
            if cmp::min(r1, r2) < *r && *r < cmp::max(r1, r2) {
                abs_distance += 1;
            }
        }

        for c in &empty_cols {
            if cmp::min(c1, c2) < *c && *c < cmp::max(c1, c2) {
                abs_distance += 1;
            }
        }
        total_distance += abs_distance;
    }

    println!("Question 1 answer: {}", total_distance);

    ()
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");
    let grid = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let all_pairs = find_all_pairs(&grid);
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&grid);

    let mut total_distance = 0;
    for ((r1, c1), (r2, c2)) in all_pairs {
        let mut abs_distance = (r1 as i64 - r2 as i64).abs() + (c1 as i64 - c2 as i64).abs();

        for r in &empty_rows {
            if cmp::min(r1, r2) < *r && *r < cmp::max(r1, r2) {
                abs_distance += 1000000-1;
            }
        }

        for c in &empty_cols {
            if cmp::min(c1, c2) < *c && *c < cmp::max(c1, c2) {
                abs_distance += 1000000-1;
            }
        }
        total_distance += abs_distance;
    }


    println!("Question 2 answer: {}", total_distance);
    ()
}

fn main() {
    first_question();
    second_question();
}
