use crate::Day;

pub struct Day1 {
    pub input: &'static str,
}

impl Day for Day1 {
    fn part1(&self) -> String {
        todo!()
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
        assert_eq!(result, "72478");
    }
}
