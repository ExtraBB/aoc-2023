use crate::Day;

pub struct Day1 {
    pub input: &'static str,
}

fn process_line(line: &str) -> i32 {
    let mut first: i32 = -1;
    let mut last: i32 = -1;
    for char in line.chars() {
        match char.to_digit(10) {
            Some(digit) => {
                if first == -1 {
                    first = digit as i32
                }
                last = digit as i32
            }
            None => continue,
        }
    }

    return first * 10 + last;
}

fn preprocess_line(line: &str) -> String {
    return line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "f4ur")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne");
}

impl Day for Day1 {
    fn part1(&self) -> String {
        return self
            .input
            .split("\n")
            .map(process_line)
            .sum::<i32>()
            .to_string();
    }

    fn part2(&self) -> String {
        return self
            .input
            .split("\n")
            .map(preprocess_line)
            .map(|line| process_line(&line))
            .sum::<i32>()
            .to_string();
    }
}

#[cfg(test)]
mod day1tests {
    use crate::{day1::Day1, Day};

    #[test]
    fn test_part1() {
        let day = Day1 {
            input: include_str!("../../data/1.in"),
        };
        let result = day.part1();
        assert_eq!(result, "53651");
    }

    #[test]
    fn test_part2() {
        let day = Day1 {
            input: include_str!("../../data/1.in"),
        };
        let result = day.part2();
        assert_eq!(result, "53894");
    }
}
