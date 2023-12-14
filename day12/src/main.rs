use std::fs::read_to_string;

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    ()
}

fn second_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    ()
}

fn main() {
    first_question();
    second_question();
}
