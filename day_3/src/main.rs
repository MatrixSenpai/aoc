#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Could not read in input");

    part_2(input);
}

fn part_1(input: String) {
    let binary_numbers: Vec<[bool; 12]> = input.split("\n")
        .map(map_input_line)
        .collect();

    let totals_map: [bool; 12] = [
        most_common_at_index(&binary_numbers, 0),
        most_common_at_index(&binary_numbers, 1),
        most_common_at_index(&binary_numbers, 2),
        most_common_at_index(&binary_numbers, 3),
        most_common_at_index(&binary_numbers, 4),
        most_common_at_index(&binary_numbers, 5),
        most_common_at_index(&binary_numbers, 6),
        most_common_at_index(&binary_numbers, 7),
        most_common_at_index(&binary_numbers, 8),
        most_common_at_index(&binary_numbers, 9),
        most_common_at_index(&binary_numbers, 10),
        most_common_at_index(&binary_numbers, 11),
    ];

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (index, b) in totals_map.iter().enumerate() {
        gamma += 2_usize.pow(11 - index as u32) * *b as usize;
        epsilon += 2_usize.pow(11 - index as u32) * !*b as usize;
    }

    println!("{}", gamma * epsilon);
}
fn part_2(input: String) {
    let binary_numbers: Vec<[bool; 12]> = input.split("\n")
        .map(map_input_line)
        .collect();

    let oxygen_rating = oxygen_rating(&binary_numbers);
    let co2_rating = co_rating(&binary_numbers);

    println!("oxy: {}\nco2: {}", oxygen_rating, co2_rating);
    println!("{}", oxygen_rating * co2_rating);
}

fn map_input_line(input: &str) -> [bool; 12] {
    let mut output: [bool; 12] = [false; 12];
    let mut split = input.split("")
        .filter(|i| !i.is_empty())
        .collect::<Vec<&str>>();

    for (index, split) in split.iter().enumerate() {
        output[index] = *split == "1";
    }

    output
}

fn most_common_at_index(total: &Vec<[bool; 12]>, index: usize) -> bool {
    let mut t_count = 0;
    let mut f_count = 0;

    for line in total {
        match line[index] {
            true => t_count += 1,
            false => f_count += 1,
        };
    }

    // NOTE: for part 1, this should be simply >, for part 2 this should be >=
    t_count >= f_count
}

fn oxygen_rating(list: &Vec<[bool; 12]>) -> usize {
    let mut list = list.clone();

    for index in 0..12 {
        let most_common = most_common_at_index(&list, index);
        list = list.into_iter()
            .filter(|i| i[index] == most_common)
            .collect();

        if list.len() == 1 { break }
    }

    let mut result: usize = 0;
    for (index, b) in list[0].iter().enumerate() {
        result += 2_usize.pow(11 - index as u32) * *b as usize;
    }

    result
}

fn co_rating(list: &Vec<[bool; 12]>) -> usize {
    let mut list = list.clone();

    for index in 0..12 {
        let least_common = !most_common_at_index(&list, index);
        list = list.into_iter()
            .filter(|i| i[index] == least_common)
            .collect();

        if list.len() == 1 { break }
    }

    let mut result: usize = 0;
    for(index, b) in list[0].iter().enumerate() {
        result += 2_usize.pow(11 - index as u32) * *b as usize;
    }

    result
}