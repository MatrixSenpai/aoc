#![allow(dead_code, unused)]

use std::collections::HashMap;

const WORD_MAP: [(&'static str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let lines = std::fs::read_to_string("day_1/src/input.txt").unwrap();

    let values = lines.split("\n")
        .map(|l| l.to_string())
        .map(check_line)
        .collect::<Vec<i32>>();


    let total = values.into_iter().reduce(|a, n| a + n).unwrap();
    println!("{total}")
}

fn check_line(line: String) -> i32 {
    let mut updated_line = line;
    for (k, v) in WORD_MAP.iter() {
        updated_line = updated_line.replace(k, format!("{}{}{}", k, v, k).as_str());
    }
    println!("{}", updated_line);

    let items = updated_line.split("")
        .filter_map(|c| c.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let first = items.first().unwrap();
    let last = items.last().unwrap();

    (first * 10) + last
}