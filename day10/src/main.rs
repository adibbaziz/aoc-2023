use core::panic;
use std::{fs::read_to_string, cmp::min, vec, collections::HashSet};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn next_move(cell: char, current_direction: Directions) -> Option<Directions> {
    match (cell, current_direction) {
        ('|', Directions::UP) => Some(Directions::UP),
        ('|', Directions::DOWN) => Some(Directions::DOWN),
        ('|', _) => None,
        ('-', Directions::LEFT) => Some(Directions::LEFT),
        ('-', Directions::RIGHT) => Some(Directions::RIGHT),
        ('F', Directions::UP) => Some(Directions::RIGHT),
        ('F', Directions::LEFT) => Some(Directions::DOWN),
        ('F', _) => None,
        ('7', Directions::UP) => Some(Directions::LEFT),
        ('7', Directions::RIGHT) => Some(Directions::DOWN),
        ('7', _) => None,
        ('L', Directions::DOWN) => Some(Directions::RIGHT),
        ('L', Directions::LEFT) => Some(Directions::UP),
        ('L', _) => None,
        ('J', Directions::DOWN) => Some(Directions::LEFT),
        ('J', Directions::RIGHT) => Some(Directions::UP),
        ('J', _) => None,
        ('.', _) => None,
        _ => None,
    }
    
}

fn find_start_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let (height, width) = (grid.len(), grid[0].len());
    let mut s_r = 0;
    let mut s_c = 0;
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == 'S' {
                s_r = r;
                s_c = c;
                break;
            }
        }
    }
    (s_r, s_c)
}

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let grid = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let (start_row, start_col) = find_start_position(&grid);

    let (height, width) = (grid.len(), grid[0].len());
    let mut distances = vec![vec![1000000000; width]; height];
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    for initial_direction in vec![Directions::UP, Directions::DOWN] {
        let mut current_direction = initial_direction.clone();
        let mut cursor = (start_row, start_col);
        let mut current_distance = 0;

        while cursor != (start_row, start_col) || current_distance == 0 {
            let (r, c) = cursor;
            distances[r][c] = min(current_distance, distances[r][c]);
            path.insert((r, c));

            let next_cursor = match current_direction {
                Directions::UP => (r - 1, c),
                Directions::DOWN => (r + 1, c),
                Directions::LEFT => (r, c - 1),
                Directions::RIGHT => (r, c + 1),
            };
            let next_cell = grid[next_cursor.0][next_cursor.1];

            if next_cell == 'S' {
                break;
            }

            let next_direction = next_move(next_cell, current_direction);
            if next_direction.is_none() {
                panic!("Invalid path");
            }
    
            current_direction = next_direction.unwrap();
            cursor = next_cursor;
            current_distance +=1;
        }
    }

    let max_distance = path.iter().map(|(r, c)| distances[*r][*c]).max().unwrap();

    println!("Question 1 answer: {}", max_distance);

    ()
}

fn find_path_cells(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    let (start_row, start_col) = find_start_position(&grid);

    let mut current_direction = Directions::UP;
    let mut cursor = (start_row, start_col);
    while !path.contains(&cursor) {
        let (r, c) = cursor;
        path.insert((r, c));

        let next_cursor = match current_direction {
            Directions::UP => (r - 1, c),
            Directions::DOWN => (r + 1, c),
            Directions::LEFT => (r, c - 1),
            Directions::RIGHT => (r, c + 1),
        };
        let next_cell = grid[next_cursor.0][next_cursor.1];

        if next_cell == 'S' {
            break;
        }

        let next_direction = next_move(next_cell, current_direction);
        if next_direction.is_none() {
            panic!("Invalid path");
        }

        current_direction = next_direction.unwrap();
        cursor = next_cursor;
    }

    path
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");
    let grid = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let path = find_path_cells(&grid);

    let (height, width) = (grid.len(), grid[0].len());

    let mut cells_inside_loop = 0;
    for r in 0..height {
        let mut c = 0;
        let mut inside_loop = false;

        while c < width {
            let mut pipes_on_path = Vec::new();
            while path.contains(&(r, c))  {
                pipes_on_path.push(grid[r][c]);
                if grid[r][c] == '|' {
                    inside_loop = !inside_loop;
                }
                c += 1;
            }
            match pipes_on_path.len() {
                0 => cells_inside_loop += 1,
                1 => inside_loop = !inside_loop,
                _ => {
                    let first_element = pipes_on_path.first().unwrap();
                    let last_element = pipes_on_path.last().unwrap();
                    if *first_element == 'L' && *last_element == 'J' {
                        return;
                    }
                    if *first_element == 'F' && *last_element == '7' {
                        return;
                    }

                }
            }

            if inside_loop {
                cells_inside_loop += 1;
            }
            c += 1;
        }
        
    }

    
    println!("Question 2 answer: {}", cells_inside_loop);

    ()
}

fn main() {
    first_question();
    second_question();
}
