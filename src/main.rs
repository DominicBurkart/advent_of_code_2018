//#[macro_use(c)]
//extern crate cute;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

macro_rules! s {
    ( $v:expr ) => { &$v.to_string() };
}

macro_rules! args_contain {
    ( $args:expr, $v:expr ) => { $args.contains(s!($v)) };
}

fn get_input(day: &str) -> Vec<String> {
    let files = match day {
        "1" => vec!["day1.txt"],
        _ => panic!("No resources for the given day"),
    };
    return files.into_iter().map(
        |file_name| {
            let input_path = Path::new("input").join(file_name);
            fs::read_to_string(input_path).unwrap()
        }).collect();
}

fn parse_inputs(inputs: Vec<String>) -> Vec<Vec<String>> {
    inputs.iter().map(|file_string| {*file_string.clone().split("\n").collect()}).collect()
}

fn day1_parse_input(inputs: Vec<String>) -> Vec<i32> {
    let input_lines = parse_inputs(inputs).get(0).unwrap();
    input_lines.iter().map(
        |l| {
            l.parse::<i32>().unwrap()
        }).collect()
}

fn day1_part1(inputs: &Vec<i32>) -> i32 {
    return inputs.iter().sum();
}

fn day1_part2(frequency_changes: &Vec<i32>) -> i32 {
    let mut last = 0;
    let mut current = last;
    let mut i = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&current) {
        last = current;
        seen.insert(last);
        current = last + frequency_changes.get(i % frequency_changes.len()).unwrap();
        i += 1;
    }
    current
}

fn day2_part1(ids: Vec<String>) {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args_contain!(args, "1") {
        let parsed = &day1_parse_input(get_input("1"));
        println!("day 1 part 1: {}", day1_part1(parsed));
        println!("day 1 part 2: {}", day1_part2(parsed));
    } else {
        println!("Ran with args: {:?}", args);
    }
}