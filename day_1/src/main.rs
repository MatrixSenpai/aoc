#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read in input");

    let depths: Vec<usize> = input.split("\n")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();


    let mut count_increase: u16 = 0;

    for index in 3..depths.len() {
        let b_index = index - 1;

        let prev_window: usize = depths[b_index - 2..=b_index].iter().sum();
        let curr_window: usize = depths[index - 2..=index].iter().sum();

        // println!("prev sum: {}\ncurr sum: {}", prev_window, curr_window);

        if curr_window > prev_window {
            count_increase += 1;
        }
    }

    println!("{}", count_increase);
}