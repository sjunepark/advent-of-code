use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::value;
use nom::IResult;

pub fn parse(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> usize {
    let mut index = 0;
    let input = std::iter::from_fn(|| {
        let result = if index < line.len() {
            Some(&line[index..])
        } else {
            None
        };
        index += 1;
        result
    });

    let mut digits = input
        .into_iter()
        .flat_map(|input| digit_parser(input))
        .map(|(_remainder, digit)| digit);

    let fist = digits.next().unwrap();
    let last = digits.last().unwrap_or(fist);

    let result = format!("{}{}", fist, last).parse::<usize>().unwrap();
    // Check that the result is a two-digit number
    assert!(result >= 10);
    assert!(result <= 99);
    result
}

/// Parses the input digit(whether numeric or spelled out) and returns the corresponding number
/// Assumes that zero is not a valid digit
fn digit_parser(input: &str) -> IResult<&str, u32> {
    alt((
        numeric_digit_parser,
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

/// Parse a single numeric digit from the input
/// Assumes that zero is not a valid digit
fn numeric_digit_parser(input: &str) -> IResult<&str, u32> {
    one_of("123456789")(input).map(|(input, c)| (input, c.to_digit(10).expect("should be a digit")))
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
