use crate::Day;

pub struct Day2 {
    pub input: &'static str,
}

fn calculate_part1_score(line: (usize, &str)) -> u32 {
    let (red, green, blue) = parse_max_rgb(line.1);
    return if red <= 12 && green <= 13 && blue <= 14 {
        (line.0 + 1) as u32
    } else {
        0
    };
}

fn calculate_part2_score(line: (usize, &str)) -> u32 {
    let (red, green, blue) = parse_max_rgb(line.1);
    return red * green * blue;
}

fn parse_max_rgb(line: &str) -> (u32, u32, u32) {
    let (_, games) = line.split_once(": ").unwrap();
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for game in games.split("; ") {
        for item in game.split(", ") {
            match item.split_once(" ").unwrap() {
                (num, "red") => red = red.max(num.parse::<u32>().unwrap()),
                (num, "green") => green = green.max(num.parse::<u32>().unwrap()),
                (num, "blue") => blue = blue.max(num.parse::<u32>().unwrap()),
                _ => panic!("invalid color"),
            }
        }
    }

    return (red, green, blue);
}

fn run(input: &str, f: fn((usize, &str)) -> u32) -> String {
    return input
        .lines()
        .into_iter()
        .enumerate()
        .map(f)
        .sum::<u32>()
        .to_string();
}

impl Day for Day2 {
    fn part1(&self) -> String {
        return run(self.input, calculate_part1_score);
    }

    fn part2(&self) -> String {
        return run(self.input, calculate_part2_score);
    }
}

#[cfg(test)]
mod day1tests {
    use crate::{day2::Day2, Day};

    #[test]
    fn test_part1() {
        let day = Day2 {
            input: include_str!("../../data/2.in"),
        };
        let result = day.part1();
        assert_eq!(result, "2268");
    }

    #[test]
    fn test_part2() {
        let day = Day2 {
            input: include_str!("../../data/2.in"),
        };
        let result = day.part2();
        assert_eq!(result, "63542");
    }
}
