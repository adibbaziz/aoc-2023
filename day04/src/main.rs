use std::{fs::read_to_string, collections::HashMap};

fn count_winning_cards(line: &str) -> i32 {
    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning_raw, sequence_raw) = numbers.split_once(" | ").unwrap();

    let winning = winning_raw
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let sequence = sequence_raw
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut occ = 0;
    for number in sequence {
        if winning.contains(&number) {
            occ += 1;
        }
    }

    occ
}

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut answer = 0;
    for line in contents.lines() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_raw, sequence_raw) = numbers.split_once(" | ").unwrap();

        let winning = winning_raw
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let sequence = sequence_raw
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();


        let mut occ = 0;
        for number in sequence {
            if winning.contains(&number) {
                if occ == 0 {
                    occ = 1;
                } else {
                    occ *= 2;
                }
            }
        }
        if occ > 0 {
            answer += occ;
        }

    }

    println!("Question 1: {}", answer);


    ()
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut total_card_multiplier: HashMap<usize, i32> = HashMap::new();
    for (idx, _)  in contents.lines().enumerate() {
        let card_number = idx+1;
        total_card_multiplier.insert(card_number, 1);
    }

    for (idx, line) in contents.lines().enumerate() {
        let card_number = idx+1;

        let n_wins = count_winning_cards(line);
        let current_multiplier = total_card_multiplier.get(&card_number).unwrap().clone();
        for i in 1..=n_wins {
            let k = card_number + (i as usize);
            let n = total_card_multiplier.get_mut(&k).unwrap();
            *n += current_multiplier;
        }
    }
    let mut total_cards = 0;
    for (_, n) in total_card_multiplier {
        total_cards += n;
    }

    println!("Question 2: {}", total_cards);
    ()
}

fn main() {
    first_question();
    second_question();
}
