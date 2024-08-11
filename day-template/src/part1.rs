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
