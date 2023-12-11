use std::fs::read_to_string;
use std::collections::HashMap;

fn find_first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_digit(10))
}

fn find_last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_digit(10))
}

fn first_question() {
    let contents = read_to_string("data/01.txt").expect("Something went wrong reading the file");

    let mut s: u32 = 0;
    contents.lines().for_each(|line| {
        let first_digit = find_first_digit(line).unwrap().to_digit(10).unwrap();
        let last_digit = find_last_digit(line).unwrap().to_digit(10).unwrap();

        s += 10*first_digit + last_digit;
    });

    println!("Sum: {}", s);
}

fn find_first_and_last(s: &str) -> (Option<&str>, Option<&str>) {
    let NUMBERS: [&str; 18] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut first_match = None;
    let mut last_match = None;
    let mut first_index = usize::MAX;
    let mut last_index = 0;

    for &number in NUMBERS.iter() {
        if let Some(index) = s.find(number) {
            if index < first_index {
                first_index = index;
                first_match = Some(number);
            }
        }
        if let Some(index) = s.rfind(number) {
            if index >= last_index {
                last_index = index;
                last_match = Some(number);
            }
        }
    }

    (first_match, last_match)
}

fn second_question() {
    let contents = read_to_string("data/01.txt").expect("Something went wrong reading the file");


    let NUMBERS_MAP = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut s: u32 = 0;
    contents.lines().for_each(|line| {
        let (first, last) = find_first_and_last(line);

        let mut first_digit = 0;
        if let Some(&number) = NUMBERS_MAP.get(first.unwrap()) {
            first_digit = number;
        } else {
            first_digit = first.unwrap().parse::<u32>().unwrap();
        }

        let mut last_digit = 0;
        if let Some(&number) = NUMBERS_MAP.get(last.unwrap()) {
            last_digit = number;
        } else {
            last_digit = last.unwrap().parse::<u32>().unwrap();
        }


        s += 10*first_digit + last_digit;
    });

    println!("Sum: {}", s);
}

fn main() {
    first_question();
    second_question();
}