use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
struct Mapper {
    sources: Vec<u64>,
    destinations: Vec<u64>,
    ranges: Vec<u64>,
}
impl Mapper {
    fn new() -> Mapper {
        Mapper {
            sources: Vec::new(),
            destinations: Vec::new(),
            ranges: Vec::new(),
        }
    }
}

fn create_mapper(contents: String) -> HashMap<&'static str, Mapper> {
    let mapper_sections = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

    let mut big_mapper: HashMap<&str, Mapper> = HashMap::new();
    for section in mapper_sections.iter() {
        big_mapper.insert(section, Mapper::new());
    }

    let mut lines_iter = contents.lines().into_iter();
    let mut current_section = "seed-to-soil";
    lines_iter.next();
    lines_iter.next();
    while let Some(current_line) = lines_iter.next() {
        if current_line.contains(" map:") {
            current_section = current_line.split_once(" ").unwrap().0;
            continue;
        }
        if current_line == "" {
            continue;
        }

        let current_line = current_line.split(" ").into_iter().map(|x| {
            let digi = x.parse::<u64>();
            if digi.is_ok() {
                return digi.unwrap();
            } else {
                panic!("{} is not a number", x)
            }
        }).collect::<Vec<u64>>();

        let current_mapper = big_mapper.get_mut(current_section).unwrap();
        current_mapper.sources.push(current_line[1]);
        current_mapper.destinations.push(current_line[0]);
        current_mapper.ranges.push(current_line[2]);

    }

    big_mapper

}

fn first_question() -> () {
    let contents = read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut seeds: Vec<u64> = Vec::new();
    let first_line = contents.lines().next().unwrap();
    first_line.split_once(": ").unwrap().1.split(" ").into_iter().for_each(|seed| {
        seeds.push(seed.parse::<u64>().unwrap());
    });

    let mapper_sections = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];

    let big_mapper = create_mapper(contents);

    for seed in seeds {
        
    }

    println!("{:?}", big_mapper);
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
