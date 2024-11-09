pub fn solve(input: &str) -> Result<u64, String> {
    extract_data(input)
        .enumerate()
        .fold(Ok(0), |acc, (line_number, result)| match acc {
            Ok(total) => match result {
                Ok(n) => Ok(total + n),
                Err(e) => Err(format!("Error on line {line_number}: {e}")),
            },
            e => e,
        })
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
    const EXAMPLE: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    #[test]
    fn extract_example() {
        assert_eq!(
            extract_data(EXAMPLE).collect::<Vec<_>>(),
            vec![Ok(12), Ok(38), Ok(15), Ok(77)]
        );
    }

    #[test]
    fn solve_example() {
        assert_eq!(solve(EXAMPLE), Ok(142));
    }
}
