use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two()
}

fn part_one() {
    let mut depth: usize = 0;
    let mut distance: usize = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                let submarine_vec: SubmarineVector = SubmarineVector {
                    direction: match parse_submarine_direction(tokens[0]) {
                        Ok(dir) => dir,
                        Err(bad_string) => panic!("Bad line in input:\n{}", bad_string),
                    },
                    distance: match tokens[1].parse() {
                        Ok(d) => d,
                        Err(e) => panic!("Bad input. Couldn't parse integer from line: {}", e),
                    },
                };

                match submarine_vec.direction {
                    SubmarineDirection::Up => depth -= submarine_vec.distance,
                    SubmarineDirection::Down => depth += submarine_vec.distance,
                    SubmarineDirection::Forward => distance += submarine_vec.distance,
                }
            }
        }
    }
    println!(
        "Net depth: {}, net forward: {}, net travel: {}",
        depth,
        distance,
        depth * distance
    )
}

fn part_two() {
    let mut aim: isize = 0;
    let mut depth: usize = 0;
    let mut distance: usize = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                let submarine_vec: SubmarineVector = SubmarineVector {
                    direction: match parse_submarine_direction(tokens[0]) {
                        Ok(dir) => dir,
                        Err(bad_string) => panic!("Bad line in input:\n{}", bad_string),
                    },
                    distance: match tokens[1].parse() {
                        Ok(d) => d,
                        Err(e) => panic!("Bad input. Couldn't parse integer from line: {}", e),
                    },
                };

                match submarine_vec.direction {
                    SubmarineDirection::Up => aim -= submarine_vec.distance as isize,
                    SubmarineDirection::Down => aim += submarine_vec.distance as isize,
                    SubmarineDirection::Forward => {
                        distance += submarine_vec.distance;
                        depth = (depth as isize + (aim * submarine_vec.distance as isize)) as usize
                    }
                }
            }
        }
    }
    println!(
        "Net depth: {}, net forward: {}, net travel: {}",
        depth,
        distance,
        depth * distance
    )
}

enum SubmarineDirection {
    Up,
    Down,
    Forward,
}

fn parse_submarine_direction(s: &str) -> Result<SubmarineDirection, &str> {
    match s {
        "up" => Ok(SubmarineDirection::Up),
        "down" => Ok(SubmarineDirection::Down),
        "forward" => Ok(SubmarineDirection::Forward),
        _ => Result::Err(s),
    }
}

struct SubmarineVector {
    direction: SubmarineDirection,
    distance: usize,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}
