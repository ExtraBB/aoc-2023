use regex::Regex;

use crate::Day;

pub struct Day2 {
    pub input: &'static str,
}

fn game_possible(game: &str, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    return parse_color(game, "red") <= max_red
        && parse_color(game, "green") <= max_green
        && parse_color(game, "blue") <= max_blue;
}

fn game_fewest(game: &str) -> u32 {
    return parse_color(game, "red") * parse_color(game, "green") * parse_color(game, "blue");
}

fn parse_color(game: &str, color: &str) -> u32 {
    let re = Regex::new(&format!(r"(\d+) {}", color)).unwrap();
    return re
        .captures_iter(game)
        .map(|capture| capture.extract())
        .map(|(_, [num])| num.parse::<u32>().unwrap())
        .max()
        .unwrap_or(0);
}

impl Day for Day2 {
    fn part1(&self) -> String {
        return self
            .input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(index, line)| {
                if line.split(";").all(|game| game_possible(game, 12, 13, 14)) {
                    return (index + 1) as u32;
                }

                return 0;
            })
            .sum::<u32>()
            .to_string();
    }

    fn part2(&self) -> String {
        return self.input.lines().map(game_fewest).sum::<u32>().to_string();
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
