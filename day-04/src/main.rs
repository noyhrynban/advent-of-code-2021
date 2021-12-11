use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Copy)]
struct BoardPosition {
    value: usize,
    called: bool,
}

struct Board {
    fields: [[BoardPosition; 5]; 5],
    wins: bool,
}

impl Board {
    fn call(&mut self, value: usize) {
        for row in 0..5 {
            for col in 0..5 {
                if self.fields[row][col].value == value {
                    self.fields[row][col].called = true;
                    self.wins = self.check_for_win();
                    return;
                }
            }
        }
    }

    fn check_for_win(&self) -> bool {
        if self.wins {
            return true;
        }
        let f = &self.fields;
        if f[0][0].called && f[0][1].called && f[0][2].called && f[0][3].called && f[0][4].called {
            return true;
        }
        if f[1][0].called && f[1][1].called && f[1][2].called && f[1][3].called && f[1][4].called {
            return true;
        }
        if f[2][0].called && f[2][1].called && f[2][2].called && f[2][3].called && f[2][4].called {
            return true;
        }
        if f[3][0].called && f[3][1].called && f[3][2].called && f[3][3].called && f[3][4].called {
            return true;
        }
        if f[4][0].called && f[4][1].called && f[4][2].called && f[4][3].called && f[4][4].called {
            return true;
        }

        if f[0][0].called && f[1][0].called && f[2][0].called && f[3][0].called && f[4][0].called {
            return true;
        }
        if f[0][1].called && f[1][1].called && f[2][1].called && f[3][1].called && f[4][1].called {
            return true;
        }
        if f[0][2].called && f[1][2].called && f[2][2].called && f[3][2].called && f[4][2].called {
            return true;
        }
        if f[0][3].called && f[1][3].called && f[2][3].called && f[3][3].called && f[4][3].called {
            return true;
        }
        if f[0][4].called && f[1][4].called && f[2][4].called && f[3][4].called && f[4][4].called {
            return true;
        }

        false
    }

    fn sum_of_uncalled(&self) -> usize {
        let mut sum = 0;

        for r in 0..5 {
            for c in 0..5 {
                if !self.fields[r][c].called {
                    sum += self.fields[r][c].value
                }
            }
        }
        sum
    }
}

fn read_board(file: &mut std::io::Lines<BufReader<std::fs::File>>) -> Option<Board> {
    let mut board: Board = Board {
        fields: [[BoardPosition {
            value: 0,
            called: false,
        }; 5]; 5],
        wins: false,
    };
    for r in 0..5 {
        let row: Vec<usize> = file
            .next()?
            .ok()?
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for (c, value) in row.iter().enumerate() {
            board.fields[r][c] = BoardPosition {
                value: *value,
                called: false,
            }
        }
    }
    file.next(); // throw away empty line
    Some(board)
}

fn part_one() {
    let mut file = read_lines("input.txt").unwrap();

    let draw_numbers: Vec<usize> = file
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    file.next(); // throw away empty line

    let mut boards: Vec<Board> = Vec::new();

    while let Some(new_board) = read_board(&mut file) {
        boards.push(new_board)
    }

    for draw in draw_numbers {
        for board in boards.iter_mut() {
            board.call(draw);
            if board.check_for_win() {
                println!("winning score: {}", draw * board.sum_of_uncalled());
                return;
            };
        }
    }
}

fn part_two() {
    // let the squid win
    let mut file = read_lines("input.txt").unwrap();

    let draw_numbers: Vec<usize> = file
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    file.next(); // throw away empty line

    let mut boards: Vec<Board> = Vec::new();

    while let Some(new_board) = read_board(&mut file) {
        boards.push(new_board)
    }

    for draw in draw_numbers {
        for board in boards.iter_mut() {
            board.call(draw);
        }

        if boards.len() == 1 && boards.first().unwrap().wins {
            println!(
                "last winning score: {}",
                draw * boards.first().unwrap().sum_of_uncalled()
            );
            return;
        }
        boards.retain(|b| !b.wins);
    }
}

fn main() {
    part_one();
    part_two();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}
