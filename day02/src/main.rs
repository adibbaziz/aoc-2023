use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(draw: &str) -> Result<Self, Self::Err> {
        let splits = draw.split(", ");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for split in splits {
            let (number, color) = split.split_once(" ").unwrap();

            match color {
                "red" => red = number.parse().unwrap(),
                "green" => green = number.parse().unwrap(),
                "blue" => blue = number.parse().unwrap(),
                _ => (),
            }
        }

        Ok(Game { red, green, blue })
    }
}

fn first_question() -> u32 {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut answer = 0;
    let mut game_index = 1;

    for line in contents.lines() {
        let (_, game_line) = line.split_once(": ").unwrap();
        let draws = game_line.split("; ");

        let mut game_is_valid = true;
        for draw in draws.into_iter() {
            let current_draw = Game::from_str(draw).unwrap();

            game_is_valid = game_is_valid && current_draw.red <= max_red && current_draw.green <= max_green && current_draw.blue <= max_blue;

        }
        if game_is_valid {
            answer += game_index;
        }
        

        game_index += 1;
    }


    println!("Question 1 answer: {}", answer);
    answer
}


fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");


    let mut answer = 0;
    
    for line in contents.lines() {
        let (_, game_line) = line.split_once(": ").unwrap();
        let draws = game_line.split("; ");


        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

        for draw in draws.into_iter() {
            let current_draw = Game::from_str(draw).unwrap();

            max_red = max_red.max(current_draw.red);
            max_green = max_green.max(current_draw.green);
            max_blue = max_blue.max(current_draw.blue);
        }

        answer += max_red * max_green * max_blue;
    }

    println!("Question 2 answer: {}", answer);
    ()
}


fn main() {
    first_question();
    second_question();
}
