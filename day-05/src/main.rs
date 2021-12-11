use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical_or_horizontal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn slope(&self) -> f32 {
        (self.end.y as f32 - self.start.y as f32) / (self.end.x as f32 - self.start.x as f32)
    }
    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if self.is_horizontal() {
            let (lesser, greater) = if self.start.x < self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };
            for x in lesser..=greater {
                points.push(Point { x, y: self.start.y });
            }
        } else if self.is_vertical() {
            let (lesser, greater) = if self.start.y < self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };
            for y in lesser..=greater {
                points.push(Point { x: self.start.x, y });
            }
        } else {
            let (lesser_x, greater_x, starting_y) = if self.start.x < self.end.x {
                (self.start.x, self.end.x, self.start.y)
            } else {
                (self.end.x, self.start.x, self.end.y)
            };
            let slope = self.slope() as isize;
            let (mut x, mut y) = (lesser_x, starting_y);
            while x <= greater_x {
                points.push(Point { x, y });
                x += 1;
                y = ((y as isize) + slope) as usize;
            }
        }

        points
    }
}

fn part_one() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut vent_lines: Vec<Line> = Vec::new();
        let mut vent_floor = Box::new([[0; 1000]; 1000]);

        for line in lines.flatten() {
            let digits: Vec<usize> = line
                .split("->")
                .flat_map(|x| x.split(','))
                .map(|x| x.trim())
                .map(|x| x.parse().unwrap())
                .collect();
            vent_lines.push(Line {
                start: Point {
                    x: digits[0],
                    y: digits[1],
                },
                end: Point {
                    x: digits[2],
                    y: digits[3],
                },
            })
        }

        for line in vent_lines {
            if line.is_vertical_or_horizontal() {
                for p in line.points() {
                    vent_floor[p.x][p.y] += 1
                }
            }
        }

        let mut total_over_2: usize = 0;
        for row in 0..1000 {
            for col in 0..1000 {
                if vent_floor[row][col] > 1 {
                    total_over_2 += 1
                }
            }
        }
        println!("total danger spots: {}", total_over_2)
    }
}

fn part_two() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut vent_lines: Vec<Line> = Vec::new();
        let mut vent_floor = Box::new([[0; 1000]; 1000]);

        for line in lines.flatten() {
            let digits: Vec<usize> = line
                .split("->")
                .flat_map(|x| x.split(','))
                .map(|x| x.trim())
                .map(|x| x.parse().unwrap())
                .collect();
            vent_lines.push(Line {
                start: Point {
                    x: digits[0],
                    y: digits[1],
                },
                end: Point {
                    x: digits[2],
                    y: digits[3],
                },
            })
        }

        for line in vent_lines {
            for p in line.points() {
                vent_floor[p.x][p.y] += 1
            }
        }

        let mut total_over_2: usize = 0;
        for row in 0..1000 {
            for col in 0..1000 {
                if vent_floor[row][col] > 1 {
                    total_over_2 += 1
                }
            }
        }
        println!("total danger spots: {}", total_over_2)
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
