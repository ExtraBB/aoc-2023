use std::collections::HashMap;

use crate::Day;

pub struct Day3 {
    pub input: &'static str,
}

struct Location {
    x: usize,
    y: usize,
}

struct Part {
    location: Location,
    length: usize,
    number: String,
}

struct Schematic {
    parts: Vec<Part>,
    symbol_map: HashMap<usize, char>,
    width: usize,
}

fn parse_schematic(input: &str) -> Schematic {
    let mut parts: Vec<Part> = vec![];
    let mut symbol_map: HashMap<usize, char> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();

    for (y, line) in lines.iter().enumerate() {
        let mut current_part: Vec<char> = vec![];
        let mut start: usize = 0;
        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if start == 0 {
                    start = x;
                }
                current_part.push(char);
            } else {
                if current_part.len() > 0 {
                    parts.push(Part {
                        location: Location { x: start, y: y },
                        length: current_part.len(),
                        number: current_part.iter().collect(),
                    });
                    start = 0;
                    current_part.clear();
                }

                if char != '.' {
                    symbol_map.insert(y * width + x, char);
                }
            }
        }

        if current_part.len() > 0 {
            parts.push(Part {
                location: Location { x: start, y: y },
                length: current_part.len(),
                number: current_part.iter().collect(),
            });
        }
    }
    return Schematic {
        parts,
        symbol_map,
        width,
    };
}

fn valid_part(part: &Part, validator: &dyn Fn(usize, usize) -> bool) -> bool {
    let min_x: usize = ((part.location.x as i32) - 1).max(0) as usize;
    let max_x: usize = (part.location.x + part.length) as usize;
    let min_y: usize = ((part.location.y as i32) - 1).max(0) as usize;
    let max_y: usize = (part.location.y + 1) as usize;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if validator(x, y) {
                return true;
            }
        }
    }

    return false;
}

fn valid_part_2(part: &Part, validator: &dyn Fn(usize, usize) -> bool) -> bool {
    let min_x: usize = part.location.x;
    let max_x: usize = part.location.x + part.length;

    for x in min_x..max_x {
        if validator(x, part.location.y) {
            return true;
        }
    }

    return false;
}

fn find_gear_parts(location: &usize, schematic: &Schematic) -> u32 {
    let gear_x = (location % schematic.width) as i32;
    let gear_y = (location / schematic.width) as i32;

    let adjacent_parts = schematic
        .parts
        .iter()
        .filter(|part| {
            valid_part_2(part, &|x, y| {
                return ((x as i32 - gear_x) as i32).abs() <= 1
                    && ((y as i32 - gear_y) as i32).abs() <= 1;
            })
        })
        .collect::<Vec<&Part>>();

    if adjacent_parts.len() == 2 {
        return adjacent_parts[0].number.parse::<u32>().unwrap()
            * adjacent_parts[1].number.parse::<u32>().unwrap();
    }

    return 0;
}

impl Day for Day3 {
    fn part1(&self) -> String {
        let schematic = parse_schematic(self.input);

        return schematic
            .parts
            .iter()
            .filter(|part| {
                valid_part(part, &|x, y| {
                    schematic
                        .symbol_map
                        .contains_key(&(y * schematic.width + x))
                })
            })
            .map(|part| part.number.parse::<u32>().unwrap())
            .sum::<u32>()
            .to_string();
    }

    fn part2(&self) -> String {
        let schematic = parse_schematic(self.input);
        return schematic
            .symbol_map
            .iter()
            .filter(|(_, char)| **char == '*')
            .map(|(location, _)| find_gear_parts(location, &schematic))
            .sum::<u32>()
            .to_string();
    }
}

#[cfg(test)]
mod day3tests {
    use crate::{day3::Day3, Day};

    #[test]
    fn test_part1() {
        let day = Day3 {
            input: include_str!("../../data/3.in"),
        };
        let result = day.part1();
        assert_eq!(result, "543867");
    }

    #[test]
    fn test_part2() {
        let day = Day3 {
            input: include_str!("../../data/3.in"),
        };
        let result = day.part2();
        assert_eq!(result, "63542");
    }
}
