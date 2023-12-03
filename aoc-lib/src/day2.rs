use crate::Day;

pub struct Day2 {
    pub input: &'static str,
}

fn parse_max_rgb(line: &str) -> (u32, u32, u32, u32) {
    let (game_title, games) = line.split_once(": ").unwrap();
    let (_, game_number) = game_title.split_once(" ").unwrap();
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

    return (game_number.parse::<u32>().unwrap(), red, green, blue);
}

fn calculate_score(input: &str, score_fn: fn((u32, u32, u32, u32)) -> u32) -> u32 {
    return input.lines().map(parse_max_rgb).map(score_fn).sum::<u32>();
}

impl Day for Day2 {
    fn part1(&self) -> String {
        return calculate_score(self.input, |(index, r, g, b)| {
            if r <= 12 && g <= 13 && b <= 14 {
                index
            } else {
                0
            }
        })
        .to_string();
    }

    fn part2(&self) -> String {
        return calculate_score(self.input, |(_, r, g, b)| r * g * b).to_string();
    }
}

#[cfg(test)]
mod day2tests {
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
