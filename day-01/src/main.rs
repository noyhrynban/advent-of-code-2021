use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut previous_line: Option<i32>;
        let mut current_line: Option<i32> = None;

        let mut counter: i32 = 0;

        for line in lines {
            if let Ok(depth) = line {
                previous_line = current_line;
                current_line = Some(depth.parse().unwrap());
                if previous_line != None {
                    if current_line.unwrap() > previous_line.unwrap() {
                        counter += 1
                    }
                }
            }
        }
        println!("increases: {}", counter)
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut previous_sum: Option<i32>;
        let mut current_sum: Option<i32> = None;

        let mut part_1: Option<i32>;
        let mut part_2: Option<i32> = None;
        let mut part_3: Option<i32> = None;

        let mut counter: i32 = 0;

        for line in lines {
            if let Ok(depth) = line {
                part_1 = part_2;
                part_2 = part_3;
                part_3 = Some(depth.parse().unwrap());
                if part_1.is_some() && part_2.is_some() && part_3.is_some() {
                    previous_sum = current_sum;
                    current_sum = Some(part_1.unwrap() + part_2.unwrap() + part_3.unwrap());
                    if previous_sum.is_some() && (current_sum.unwrap() > previous_sum.unwrap()) {
                        counter += 1
                    }
                }
            }
        }
        println!("increases (window): {}", counter)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
