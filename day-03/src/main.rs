use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}

fn part_one() {
    let mut places: [isize; 12] = [0; 12];

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                for (index, c) in line.char_indices() {
                    if c == '1' {
                        places[index] += 1
                    } else if c == '0' {
                        places[index] -= 1
                    }
                }
            }
        }
    }

    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    for place in places {
        if place > 0 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!(
        "gamma: {}\nepsilon: {}\ntotal: {}",
        gamma,
        epsilon,
        (gamma * epsilon)
    );
}

fn part_two() {
    let mut input: Vec<String> = Vec::new();
    // let mut places: [isize; 12] = [0; 12];

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line) = line {
                input.push(line);
            }
        }
        let mut working_list = input.clone();
        for pos in 0..12 {
            let mut one_bits: Vec<String> = Vec::new();
            let mut zero_bits: Vec<String> = Vec::new();
            for value in working_list {
                if value.chars().nth(pos) == Some('1') {
                    one_bits.push(value.to_string())
                } else if value.chars().nth(pos) == Some('0') {
                    zero_bits.push(value.to_string())
                }
            }
            working_list = if one_bits.len() > zero_bits.len() {
                one_bits
            } else if one_bits.len() < zero_bits.len() {
                zero_bits
            } else {
                one_bits
            };
        }
        let oxygen_rating = isize::from_str_radix(working_list.first().unwrap(), 2).unwrap();
        println!("oxygen rating: {}", oxygen_rating);

        let mut working_list = input.clone();
        for pos in 0..12 {
            let mut one_bits: Vec<String> = Vec::new();
            let mut zero_bits: Vec<String> = Vec::new();
            for value in working_list {
                if value.chars().nth(pos) == Some('1') {
                    one_bits.push(value.to_string())
                } else if value.chars().nth(pos) == Some('0') {
                    zero_bits.push(value.to_string())
                }
            }
            working_list = if one_bits.len() > zero_bits.len() {
                zero_bits
            } else if one_bits.len() < zero_bits.len() {
                one_bits
            } else {
                zero_bits
            };
            if working_list.len() == 1 {
                break;
            }
        }
        let carbon_diozide_rating =
            isize::from_str_radix(working_list.first().unwrap(), 2).unwrap();
        println!("carbon dioxide rating: {}", carbon_diozide_rating);

        println!(
            "life support rating: {}",
            oxygen_rating * carbon_diozide_rating
        );
    }
}

fn main() {
    part_one();
    part_two();
}
