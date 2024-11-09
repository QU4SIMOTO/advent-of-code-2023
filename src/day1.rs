fn format_results(results: impl Iterator<Item = Result<u64, &'static str>>) -> Result<u64, String> {
    results
        .enumerate()
        .fold(Ok(0), |acc, (line_number, result)| match acc {
            Ok(total) => match result {
                Ok(n) => Ok(total + n),
                Err(e) => Err(format!("Error on line {line_number}: {e}")),
            },
            e => e,
        })
}

pub mod part2 {
    use super::*;

    pub fn solve(input: &str) -> Result<u64, String> {
        format_results(extract_data(input))
    }

    fn extract_data<'a>(input: &'a str) -> impl Iterator<Item = Result<u64, &'static str>> + 'a {
        input.lines().map(|line| {
            let mut digits = Parser(line);
            let first = digits.next().ok_or("no digits")?;
            let last = digits.last().unwrap_or(first);
            Ok(format!("{first}{last}")
                .parse()
                .expect("both are ascii digits"))
        })
    }

    struct Parser<'a>(&'a str);
    impl<'a> Iterator for Parser<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            fn try_parse(input: &mut Parser, item: &'static str) -> bool {
                if input.0.starts_with(item) {
                    input.0 = &input.0[item.len()..];
                    true
                } else {
                    false
                }
            }
            loop {
                if self.0.len() == 0 {
                    return None;
                } else {
                    match self {
                        _ if try_parse(self, "one") | try_parse(self, "1") => break Some("1"),
                        _ if try_parse(self, "two") | try_parse(self, "2") => break Some("2"),
                        _ if try_parse(self, "three") | try_parse(self, "3") => break Some("3"),
                        _ if try_parse(self, "four") | try_parse(self, "4") => break Some("4"),
                        _ if try_parse(self, "five") | try_parse(self, "5") => break Some("5"),
                        _ if try_parse(self, "six") | try_parse(self, "6") => break Some("6"),
                        _ if try_parse(self, "seven") | try_parse(self, "7") => break Some("7"),
                        _ if try_parse(self, "eight") | try_parse(self, "8") => break Some("8"),
                        _ if try_parse(self, "nine") | try_parse(self, "9") => break Some("9"),
                        _ => {
                            self.0 = &self.0[1..];
                            continue;
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const EXAMPLE_1: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        const EXAMPLE_2: &'static str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        #[test]
        fn extract_example_1() {
            assert_eq!(
                extract_data(EXAMPLE_1).collect::<Vec<_>>(),
                vec![Ok(12), Ok(38), Ok(15), Ok(77)]
            );
        }

        #[test]
        fn extract_example_2() {
            assert_eq!(
                extract_data(EXAMPLE_2).collect::<Vec<_>>(),
                vec![Ok(29), Ok(83), Ok(13), Ok(24), Ok(42), Ok(14), Ok(76)]
            );
        }

        #[test]
        fn solve_example_1() {
            assert_eq!(solve(EXAMPLE_1), Ok(142));
        }

        #[test]
        fn solve_example_2() {
            assert_eq!(solve(EXAMPLE_2), Ok(281));
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(input: &str) -> Result<u64, String> {
        format_results(extract_data(input))
    }

    fn extract_data<'a>(input: &'a str) -> impl Iterator<Item = Result<u64, &'static str>> + 'a {
        input.lines().map(|line| {
            let mut digits = line.chars().filter(char::is_ascii_digit);
            let first = digits.next().ok_or("no digits")?;
            let last = digits.last().unwrap_or(first);
            Ok(format!("{first}{last}")
                .parse()
                .expect("both are ascii digits"))
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const EXAMPLE_1: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        #[test]
        fn extract_example_1() {
            assert_eq!(
                extract_data(EXAMPLE_1).collect::<Vec<_>>(),
                vec![Ok(12), Ok(38), Ok(15), Ok(77)]
            );
        }

        #[test]
        fn solve_example_1() {
            assert_eq!(solve(EXAMPLE_1), Ok(142));
        }
    }
}
