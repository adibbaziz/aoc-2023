use std::{fs::read_to_string, collections::HashMap};

fn build_graph() -> (Vec<char>, HashMap<String, (String, String)>) {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut iter = contents.lines().into_iter();
    let instructions = iter.next().unwrap().chars().collect::<Vec<char>>();
    iter.next();
    
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    iter.for_each(|line| {
        let (source, destination) = line.split_once(" = ").unwrap();
        let (left, right) = destination.trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();

        graph.insert(source.to_string(), (left.to_string(), right.to_string()));
    });

    (instructions, graph)
}

fn first_question() -> () {
    let (instructions, graph) = build_graph();

    let start = "AAA";
    let end = "ZZZ";

    let mut answer = 0;
    let mut current = start.to_string();
    let mut cursor = 0;
    while current != end {
        match instructions[cursor % instructions.len()] {
            'L' => {
                let (left, _) = graph.get(&current).unwrap();
                current = left.to_string();
                
            },
            'R' => {
                let (_, right) = graph.get(&current).unwrap();
                current = right.to_string();
            },
            _ => panic!("Invalid instruction")
        }
        cursor += 1;
        answer += 1;
    }

    println!("First question: {}", answer);
    ()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_of_list(numbers: &[u64]) -> Option<u64> {
    if numbers.is_empty() {
        None
    } else {
        let mut result = numbers[0];
        for &num in &numbers[1..] {
            result = (result * num) / gcd(result, num);
        }
        Some(result)
    }
}

fn second_question() -> () {
    let (instructions, graph) = build_graph();

    let starts = graph.keys().filter(|node| node.ends_with("A")).collect::<Vec<&String>>();

    
    // Array of zeroes of size starts.len
    let mut answers = vec![0; starts.len()];
    for i in 0..starts.len() {
        let mut current = starts[i].to_string();
        let mut cursor = 0;
        while !current.ends_with("Z") {
            match instructions[cursor % instructions.len()] {
                'L' => {
                    let (left, _) = graph.get(&current).unwrap();
                    current = left.to_string();
                    
                },
                'R' => {
                    let (_, right) = graph.get(&current).unwrap();
                    current = right.to_string();
                },
                _ => panic!("Invalid instruction")
            }
            cursor += 1;
            answers[i] += 1;
        }
    }

    let answer = lcm_of_list(&answers).unwrap();
    println!("Second question: {}", answer);
    ()
}

fn main() {
    first_question();
    second_question();
}
