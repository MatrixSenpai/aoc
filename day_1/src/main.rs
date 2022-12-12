#![allow(unused, dead_code)]

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    let elves: Vec<Elf> = input
        .split("\n\n")
        .into_iter()
        .map(|s| s.to_string())
        .map(Elf::from)
        .collect();

    part_two(elves);
}

fn part_one(input: Vec<Elf>) {
    let mut elves = input;
    elves.sort();

    println!("elf with most: {}", elves.last().unwrap().0);
}

fn part_two(input: Vec<Elf>) {
    let mut elves = input;
    elves.sort();

    let start = elves.len() - 3;
    let mut acc = 0;
    for i in start..elves.len() {
        acc += elves[i].0;
        println!("e: {}", elves[i].0);
    }

    println!("top 3 elves: {acc}");
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Elf(u32);
impl From<String> for Elf {
    fn from(s: String) -> Self {
        let t = s.split("\n")
            .into_iter()
            .map(|s| s.parse::<u32>().unwrap())
            .reduce(|a, i| a + i)
            .unwrap();

        Self(t)
    }
}