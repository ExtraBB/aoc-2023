use aoc_lib::{day1::Day1, day2::Day2, Day};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No day provided");
    }

    if args.len() < 3 {
        panic!("No part provided");
    }

    let day: u8 = match args[1].trim().parse() {
        Ok(day) => day,
        Err(_) => panic!("Invalid day provided"),
    };

    let part: u8 = match args[2].trim().parse() {
        Ok(part) => part,
        Err(_) => panic!("Invalid part provided"),
    };

    execute(day, part);
}

fn execute(day_number: u8, part: u8) {
    match day_number {
        1 => execute_part(
            Day1 {
                input: include_str!("../../data/1.in"),
            },
            day_number,
            part,
        ),
        2 => execute_part(
            Day2 {
                input: include_str!("../../data/2.in"),
            },
            day_number,
            part,
        ),
        _ => panic!("Solution for day {day_number}, part {part} not implemented yet."),
    };
}

fn execute_part(day: impl Day, day_number: u8, part: u8) {
    let result = match part {
        1 => day.part1(),
        2 => day.part2(),
        _ => panic!("Solution for day {day_number}, part {part} not implemented yet."),
    };

    println!("Result: {result}");
}
