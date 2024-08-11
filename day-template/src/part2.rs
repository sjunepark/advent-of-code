pub fn parse(input: &str) -> usize {
    unimplemented!()
}

fn parse_line(line: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_parser() {
        let inputs = vec![
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        inputs.into_iter().for_each(|input| {
            let (line, expected) = input;
            assert_eq!(expected, digit_parser(line).unwrap().1);
        });
    }

    #[test]
    fn test_parse_line() {
        let inputs = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        inputs.into_iter().for_each(|input| {
            let (line, expected) = input;
            assert_eq!(expected, parse_line(line));
        });
    }

    #[test]
    fn test_parse() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(281, parse(input));
    }
}
