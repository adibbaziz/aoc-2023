use std::fs::read_to_string;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut iter = contents.lines().into_iter();
    let first_line = iter.next().unwrap();
    let second_line = iter.next().unwrap();

    let mut times: Vec<i32> = Vec::new();
    first_line.trim().split(" ").into_iter().for_each(|x| {
        match x.trim().parse::<i32>() {
            Ok(x) => times.push(x),
            Err(_) => (),
        };
    });
    let mut distances: Vec<i32> = Vec::new();
    second_line.trim().split(" ").into_iter().for_each(|x| {
        match x.trim().parse::<i32>() {
            Ok(x) => distances.push(x),
            Err(_) => (),
        };
    });

    (times, distances)
}

fn compute_distance<T>(hold_time: T, total_time: T) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    let distance_traveled = hold_time * (total_time - hold_time);
    distance_traveled
}

fn first_question() -> () {
    let (times, distances) = read_input();

    /*
     * total_time in miliseconds
     * remaining_time = total_time - hold_time
     * speed = hold_time 
     * distance_traveled(hold_time) = hold_time * (total_time - hold_time)
     * 
     * f(x) = a*x - x^2
     */


    let mut answer = 1;
    for (total_time, max_distance) in times.iter().zip(distances.iter()) {

        let mut number_of_ways = 0;
        for i in 0..*total_time {
            let distance = compute_distance(i, *total_time);
            if distance > *max_distance {
                number_of_ways += 1;
            }
        }

        answer *= number_of_ways;
    }


    println!("Question 1 answer: {}", answer);
}

fn second_question() -> () {
    let (times, distances) = read_input();

    let total_time = times.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
    let max_distance = distances.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();

    /*
     * a = total_time
     * f(x) = a*x - x^2
     * f'(x) = a - 2x
     * f'(x) = 0 if x = a/2
     * 
     * arg_max = a/2
     * f(x) > max_distance <=> arg_max - delta_x < x < arg_max + delta_x
     * 
     * we can find delta_x using binary search between 0 and arg_max
     */

    // let arg_max = total_time / 2; // total_time is a multiple of 2
    // let (mut lo, mut hi) = (0, arg_max);
    // while lo <= hi {
    //     let mid = (lo + hi) / 2;
    //     let distance = compute_distance(mid, total_time);
    //     if distance > max_distance {
    //         lo = mid + 1;
    //     } else {
    //         hi = mid;
    //     }
    // }

    let mut answer = 0;
    for i in 0..total_time {
        let distance = compute_distance(i, total_time);
        if distance > max_distance {
            answer += 1;
        }
    }


    println!("Question 2 answer: {}", answer);


}

fn main() {
    first_question();
    second_question();
}
