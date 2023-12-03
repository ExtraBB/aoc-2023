use crate::Day;

pub struct Day1 {
    pub input: &'static str,
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let mut sum: i32 = 0;

        for line in self.input.split("\n") {
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

            sum += first * 10 + last;
        }

        return sum.to_string();
    }

    fn part2(&self) -> String {
        todo!()
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
}
