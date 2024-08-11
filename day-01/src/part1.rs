use nom::InputIter;

pub fn parse(input: &str) -> usize {
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> usize {
    let mut digits = line.iter_elements().flat_map(|c| c.to_digit(10));

    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);

    let result = format!("{}{}", first, last).parse::<usize>().unwrap();
    // Check that the result is a two-digit number
    assert!(result >= 10);
    assert!(result <= 99);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let inputs = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        inputs.into_iter().for_each(|input| {
            let (line, expected) = input;
            assert_eq!(expected, parse_line(line));
        });
    }

    #[test]
    fn test_parse() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(parse(input), 142);
    }
}
