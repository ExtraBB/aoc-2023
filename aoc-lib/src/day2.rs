use crate::Day;

pub struct Day2 {
    pub input: &'static str,
}

fn game_possible(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    let (red, green, blue) = parse_max_rgb(line);
    return red <= max_red && green <= max_green && blue <= max_blue;
}

fn game_fewest(line: &str) -> u32 {
    let (red, green, blue) = parse_max_rgb(line);
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

impl Day for Day2 {
    fn part1(&self) -> String {
        return self
            .input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(index, line)| {
                if game_possible(line, 12, 13, 14) {
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
