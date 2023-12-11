use std::{fs::read_to_string, collections::{HashSet, HashMap}};

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    row: i32,
    start_col: i32,
    end_col: i32,
}

impl Number {
    pub fn neighbor_cells(&self) -> Vec<(i32, i32)> {
        let delta_h: [i32; 3] = [0, 1, -1];
        let delta_w: [i32; 3] = [0, 1, -1];

        let mut neighbors: HashSet<(i32, i32)> = HashSet::new();
        for c in self.start_col..(self.end_col+1) {
            for dh in delta_h {
                for dw in delta_w {
                    if dh == 0 && dw == 0  { continue; }
                    let r = self.row + dh;
                    let c = c + dw;

                    neighbors.insert((r, c));
                }
            }
            neighbors.remove(&(self.row, c));
        }

        neighbors.into_iter().collect::<Vec<(i32, i32)>>()
    }
}


fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (height, width) = (grid.len(), grid[0].len());

    let mut symbols_positions: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..height {
        for c in 0..width {
            if is_symbol(grid[r][c]) {
                symbols_positions.insert((r, c));
            }
        }
    }


    
    let mut answer = 0;
    for r in 0..height {
        let mut c = 0;
        while c < width {
            if !grid[r][c].is_digit(10) {
                c += 1;
                continue;
            }

            let mut number_str = "".to_string();
            while c < width && grid[r][c].is_digit(10) {
                number_str += &grid[r][c].to_string();
                c += 1;
            }
            if number_str.len() == 0 { continue; }
            let number = Number {
                value: number_str.parse::<i32>().unwrap(),
                row: r as i32,
                start_col: (c - number_str.len()) as i32,
                end_col: (c-1) as i32,
            };

            for (r, c) in number.neighbor_cells() {
                if symbols_positions.contains(&(r as usize, c as usize)) {
                    answer += number.value;
                    break;
                }
            }
        }

    }

    println!("Question 1: {}", answer);
    ()
}

fn find_all_number(grid: &Vec<Vec<char>>) -> Vec<Number> {
    let (height, width) = (grid.len(), grid[0].len());

    let mut all_number: Vec<Number> = Vec::new();
    for r in 0..height {
        let mut c = 0;
        while c < width {
            if !grid[r][c].is_digit(10) {
                c += 1;
                continue;
            }

            let mut number_str = "".to_string();
            while c < width && grid[r][c].is_digit(10) {
                number_str += &grid[r][c].to_string();
                c += 1;
            }
            if number_str.len() == 0 { continue; }
            let number = Number {
                value: number_str.parse::<i32>().unwrap(),
                row: r as i32,
                start_col: (c - number_str.len()) as i32,
                end_col: (c-1) as i32,
            };
            all_number.push(number);
        }
    }

    all_number
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (height, width) = (grid.len(), grid[0].len());

    let all_numbers = find_all_number(&grid);

    let mut asterisk_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut asterisk_neighbors: HashMap<(usize, usize), Vec<Number>> = HashMap::new();
    for r in 0..height {
        for c in 0..width {
            asterisk_neighbors.insert((r, c), Vec::new());
            if grid[r][c] != '*' { continue; }
            asterisk_positions.insert((r, c));
        }
    }

    for number in all_numbers {
        for (r, c) in number.neighbor_cells() {
            if asterisk_positions.contains(&(r as usize, c as usize)) {
                let nei = asterisk_neighbors.get_mut(&(r as usize, c as usize)).unwrap();
                nei.push(number.clone());
            }
        }
    }

    let mut answer = 0;
    for (r, c) in asterisk_neighbors.keys().into_iter() {
        let neighbors = asterisk_neighbors.get(&(*r, *c)).unwrap();
        if neighbors.len() != 2 { continue; }

        answer += neighbors[0].value * neighbors[1].value;
    }

    println!("Question 2: {}", answer);
    ()
}

fn main() {
    first_question();
    second_question();
}
