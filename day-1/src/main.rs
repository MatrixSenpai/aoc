#![allow(unused, dead_code)]

use std::collections::HashMap;
use std::ops::AddAssign;
use anyhow::Result;

fn main() -> Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let input = std::fs::read_to_string("./day-1/input.txt")?;

    let lists = input.split("\n").into_iter()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(String::from)
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        }).collect::<Vec<Vec<isize>>>();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for mut item in lists.into_iter() {
        right_list.append(&mut item.split_off(1));
        left_list.append(&mut item);
    }

    // let total = part_one(left_list, right_list);
    let total = part_two(left_list, right_list);
    log::info!("Total is {total}");

    Ok(())
}

fn part_one(mut left_list: Vec<isize>, mut right_list: Vec<isize>) -> isize {
    left_list.sort();
    right_list.sort();

    left_list.into_iter().zip(right_list.into_iter())
        .map(|(l, r)| l - r)
        .fold(0, |acc, n| n.abs() + acc)
}

fn part_two(mut left_list: Vec<isize>, mut right_list: Vec<isize>) -> isize {
    let mut sim_score: HashMap<isize, usize> = HashMap::new();

    for number in left_list.into_iter() {
        let count = right_list.iter()
            .filter(|i| number.eq(i))
            .count();

        let score = (number as usize) * count;

        if sim_score.contains_key(&number) {
            sim_score.get_mut(&number).unwrap().add_assign(score);
        } else {
            sim_score.insert(number, score);
        }
    }

    sim_score.into_iter()
        .fold(0, |acc, (_, v)| v + acc) as isize
}