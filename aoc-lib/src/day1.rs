pub fn part1(_data: &str) -> String {
    return "todo".to_string();
}

#[cfg(test)]
mod day1tests {
    use crate::day1;

    #[test]
    fn test_part1() {
        let data = include_str!("../../data/1.in");
        let result = day1::part1(&data);
        assert_eq!(result, "72478");
    }
}
