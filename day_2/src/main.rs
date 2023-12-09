#![allow(unused, dead_code)]

const TEST_CUBE_SET: CubeSet = CubeSet { red: 12, green: 13, blue: 14 };

fn main() {
    let values = (12_u32, 13_u32, 14_u32);

    let input = std::fs::read_to_string("day_2/src/input.txt").unwrap();

    let all_games = input.split("\n")
        .map(String::from)
        .map(Game::from)
        .collect::<Vec<Game>>();

    let total = all_games.iter()
        .filter_map(|game| if game.is_valid() { Some(game.id) } else { None })
        .fold(0, |a, n| a + n);

    println!("Day 1: {total}");

    let power_total = all_games.iter()
        .map(|game| game.min_set().power())
        .fold(0, |a, s| a + s);

    println!("Day 2: {power_total}");
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>
}
impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(CubeSet::is_valid)
    }

    fn min_set(&self) -> CubeSet {
        self.sets.iter()
            .fold(CubeSet::default(), |a, set| CubeSet {
                red: a.red.max(set.red),
                green: a.green.max(set.green),
                blue: a.blue.max(set.blue),
            })
    }
}
impl From<String> for Game {
    fn from(value: String) -> Self {
        let (game, sets) = value.split_once(": ").unwrap();

        let (_, id) = game.split_once(" ").unwrap();
        let id = id.parse::<u32>().unwrap();

        let sets = sets.split("; ").into_iter()
            .map(String::from)
            .map(CubeSet::from)
            .collect::<Vec<CubeSet>>();

        Self {
            id, sets
        }
    }
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}
impl CubeSet {
    fn load_red(mut self, value: u32) -> Self {
        self.red = value;
        self
    }
    fn load_green(mut self, value: u32) -> Self {
        self.green = value;
        self
    }
    fn load_blue(mut self, value: u32) -> Self {
        self.blue = value;
        self
    }

    fn is_valid(&self) -> bool {
        self.red <= TEST_CUBE_SET.red
        && self.green <= TEST_CUBE_SET.green
        && self.blue <= TEST_CUBE_SET.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
impl From<String> for CubeSet {
    fn from(value: String) -> Self {
        let mut set = Self::default();

        for value in value.split(", ") {
            let (count, color) = value.split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();

            match color {
                "red" => set = set.load_red(count),
                "green" => set = set.load_green(count),
                "blue" => set = set.load_blue(count),

                _ => unreachable!("unknown color!")
            }
        }

        set
    }
}