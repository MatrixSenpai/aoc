#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use core::convert::TryInto;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Could not read in input");

    let directions: Vec<Movement> = input.split("\n")
        .map(|s| Movement::from(s))
        .collect();

    let mut horizontal_position: isize = 0;
    let mut vertical_position: isize = 0;
    let mut current_aim: isize = 0;

    for movement in directions.iter() {
        let distance: isize = movement.1.try_into().unwrap();
        match movement.0 {
            Direction::F => {
                horizontal_position += distance;
                vertical_position += distance * current_aim;
            },
            Direction::U => current_aim -= distance,
            Direction::D => current_aim += distance,
        }
    }

    println!(
        "Vertical: {}\nHorizontal: {}\nTotal Pos: {}",
        vertical_position, horizontal_position, vertical_position * horizontal_position
    );
}

#[derive(Debug)]
struct Movement(Direction, usize);
impl From<&str> for Movement {
    fn from(input: &str) -> Self {
        let split_input: Vec<&str> = input.split(" ").collect();

        let direction = Direction::from(split_input[0]);
        let length = split_input[1].parse::<usize>().unwrap();

        Movement(direction, length)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    F, U, D
}
impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "forward" => Self::F,
            "up" => Self::U,
            "down" => Self::D,
            // controlled input, should never happen
            _ => Self::F,
        }
    }
}