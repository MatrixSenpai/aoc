#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Engine {
    game_over: bool,
    call_sequence: Vec<usize>,
    index: usize,
    players: Vec<Board>,
}
impl Engine {
    fn new(call_sequence: Vec<usize>, players: Vec<Board>) -> Self {
        Engine {
            call_sequence, players,
            game_over: false,
            index: 0,
        }
    }

    fn cycle(&mut self) {
        let next_in_sequence = self.call_sequence[self.index];
        self.index += 1;

        for (index, player) in self.players.iter_mut().enumerate() {
            if player.has_won { continue; }

            player.call(&next_in_sequence);

            if player.check_win() {
                let score = player.get_score(&next_in_sequence);
                println!("Player {} won with score {}", index, score);

                player.has_won = true;
            }
        }
    }

    fn should_continue(&self) -> bool {
        // self.index < self.call_sequence.len() && !self.game_over
        for player in self.players.iter() {
            if player.has_won == false {
                return true;
            }
        }

        false
    }
}
impl TryFrom<String> for Engine {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // first split gets call sequence and each board
        let sequence_boards_split = value.split("\n\n").collect::<Vec<&str>>();
        if sequence_boards_split.len() < 2 {
            return Err(String::from("Unexpectedly short input"));
        }

        let sequence_split = sequence_boards_split[0].split(",")
            .map(|i| i.parse().unwrap())
            .collect::<Vec<usize>>();

        let boards: Vec<Board> = sequence_boards_split[1..].iter()
            .map(|board| {
                let board = String::from(*board);
                let board = Board::try_from(board)
                    .expect("Could not convert board to string");

                board
            })
            .collect();

        Ok(Engine::new(sequence_split, boards))
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<Cell>>,
    has_won: bool,
}
impl Board {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        Board { cells, has_won: false }
    }

    fn call(&mut self, number: &usize) {
        self.cells.iter_mut().enumerate().for_each(|(_y, row)| {
            for (_x, cell) in row.iter_mut().enumerate() {
                if cell.eq(&number) {
                    cell.set_called();
                }
            }
        });
    }

    fn check_win(&self) -> bool {
        // horizontal check
        for row in self.cells.iter() {
            let mut row_flag = false;
            for cell in row.iter() {
                if !cell.called {
                    row_flag = false;
                    break;
                } else { row_flag = true; }
            }

            if row_flag { return true; }
        }

        // vertical check
        for index in 0..self.cells.len() {
            let mut col_flag = false;
            for row in self.cells.iter() {
                if !row[index].called {
                    col_flag = false;
                    break;
                } else { col_flag = true; }
            }

            if col_flag { return true; }
        }

        // diagonal check
        // ok, apparently diagonals do NOT count. leaving this in here for s&g
        // if self.cells[2][2].called {
        //     if (
        //         self.cells[0][0].called && self.cells[1][1].called && self.cells[3][3].called && self.cells[4][4].called
        //     ) || (
        //         self.cells[0][4].called && self.cells[1][3].called && self.cells[3][1].called && self.cells[4][0].called
        //     ) {
        //         return true;
        //     }
        // }

        return false;
    }

    fn get_score(&self, last_called: &usize) -> usize {
        let mut score: usize = 0;
        for row in self.cells.iter() {
            for cell in row.iter() {
                if !cell.called {
                    score += cell.number;
                }
            }
        }

        score * *last_called
    }
}
impl TryFrom<String> for Board {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let rows: Vec<&str> = value.split("\n").collect();

        let parsed = rows.into_iter()
            .map(|row| {
                row.split_whitespace()
                    .map(|c| Cell::try_from(String::from(c)).unwrap())
                    .collect()
            })
            .collect();

        Ok(Board::new(parsed))
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Cell {
    number: usize,
    called: bool,
}
impl Cell {
    fn new(number: usize, called: bool) -> Self {
        Cell { number, called }
    }

    fn set_called(&mut self) {
        self.called = true;
    }

    fn eq(&mut self, number: &usize) -> bool {
        self.number == *number
    }
}
impl TryFrom<String> for Cell {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: usize = value.parse().unwrap();
        Ok(Cell::new(value, false))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Could not read in input");

    part_1(input);
}

fn part_1(input: String) {
    let mut engine = Engine::try_from(input)
        .expect("Could not parse the input into a valid engine");
    // println!("{:#?}", engine);

    while engine.should_continue() {
        engine.cycle();
    }

    println!("Game over");
}
fn part_2(input: String) {}