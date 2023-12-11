use std::fs::read_to_string;


fn process_line_1(line: &str) -> i64 {
    let mut nums: Vec<i64> = Vec::new();
    for c in line.split(" ") {
        nums.push(c.parse::<i64>().unwrap());
    }


    let mut diffed = nums.clone();
    
    fn all_zeros(diff: &Vec<i64>) -> bool {
        diff.into_iter().fold(true, |acc, e| acc && e == &0)
    }

    let mut last_numbers = Vec::new();
    
    while !all_zeros(&diffed) && diffed.len() > 1 {
        last_numbers.push(diffed.last().unwrap().clone());

        let mut new_diffed: Vec<i64> = Vec::new();
        for i in 0..diffed.len() - 1 {
            new_diffed.push(diffed[i + 1] - diffed[i]);
        }
        diffed = new_diffed;
    }

    last_numbers.into_iter().fold(0, |acc, e| acc + e)
}

fn process_line_2(line: &str) -> i64 {
    let mut nums: Vec<i64> = Vec::new();
    for c in line.split(" ") {
        nums.push(c.parse::<i64>().unwrap());
    }


    let mut diffed = nums.clone();
    
    fn all_zeros(diff: &Vec<i64>) -> bool {
        diff.into_iter().fold(true, |acc, e| acc && e == &0)
    }

    let mut first_numbers = Vec::new();
    
    while !all_zeros(&diffed) && diffed.len() > 1 {
        first_numbers.push(diffed.first().unwrap().clone());

        let mut new_diffed: Vec<i64> = Vec::new();
        for i in 0..diffed.len() - 1 {
            new_diffed.push(diffed[i + 1] - diffed[i]);
        }
        diffed = new_diffed;
    }
    first_numbers.reverse();

    first_numbers.into_iter().fold(0, |acc, e| e - acc)
}

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut answer = 0;

    contents.lines().for_each(|line| {
        answer += process_line_1(line);
    });

    println!("Question 1 answer: {}", answer);
    ()
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut answer = 0;

    contents.lines().for_each(|line| {
        answer += process_line_2(line);
    });

    println!("Question 2 answer: {}", answer);
    ()
}

fn main() {
    first_question();
    second_question();
}
