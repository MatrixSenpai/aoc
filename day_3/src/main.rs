#![allow(unused, dead_code)]

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part_one(input);
}

fn part_one(input: String) {
    let sacks = input.split("\n")
        .map(String::from)
        .map(Rucksack::from)
        .collect::<Vec<Rucksack>>();

    let sum = sacks.into_iter()
        .fold(0u32, |accumulator, sack| accumulator + sack.find_common() as u32);

    println!("{sum}");
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Rucksack {
    left: Vec<u8>,
    right: Vec<u8>,
}
impl Rucksack {
    fn convert_letter(value: &str) -> u8 {
        let letter = value.as_bytes().first().unwrap().to_owned();

        if letter >= 65 && letter <= 90 {
            letter - 38
        } else if letter >= 98 && letter <= 122 {
            letter - 96
        } else {
            0
        }
    }

    fn find_common(&self) -> u8 {
        for left_item in &self.left {
            for right_item in &self.right {
                if left_item == right_item && left_item > &0 {
                    return left_item.clone()
                }
            }
        }

        unimplemented!()
    }
}
impl From<String> for Rucksack {
    fn from(value: String) -> Rucksack {
        let compartment_size = value.len() / 2;
        let left = &*value[0..compartment_size].to_string();
        let right = &*value[compartment_size..value.len()].to_string();

        let left_transformed = left.split("")
            .filter(|s| !s.is_empty())
            .map(Rucksack::convert_letter)
            .collect();

        let right_transformed = right.split("")
            .filter(|s| !s.is_empty())
            .map(Rucksack::convert_letter)
            .collect();

        Rucksack {
            left: left_transformed,
            right: right_transformed,
        }
    }
}