use std::fs::read_to_string;
mod cards1;
mod cards2;

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut hands = contents.lines().map(|x| x.parse::<cards1::Hand>().unwrap()).collect::<Vec<cards1::Hand>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut answer: u64 = 0;
    for (i, card) in hands.iter().enumerate() {
        answer += (card.bid as u64) * ((i+1) as u64);
    }


    println!("Question 1 answer: {}", answer);
    ()
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut hands = contents.lines().map(|x| x.parse::<cards2::Hand>().unwrap()).collect::<Vec<cards2::Hand>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut answer: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        answer += (hand.bid as u64) * ((i+1) as u64);
    }

    println!("Question 2 answer: {}", answer);
    ()
}

fn main() {
    first_question();
    second_question();
}
