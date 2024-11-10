#[derive(Default, Debug, PartialEq, Eq)]
struct Round {
    r: u64,
    g: u64,
    b: u64,
}

impl Round {
    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        input.split(",").fold(Ok(Round::default()), |acc, curr| {
            let Ok(mut acc) = acc else {
                return acc;
            };
            let (count, colour) = curr
                .trim()
                .split_once(" ")
                .ok_or("count must contain colour and number")?;
            let count = count.trim().parse::<u64>()?;
            match colour.trim() {
                "red" => acc.r += count,
                "green" => acc.g += count,
                "blue" => acc.b += count,
                other => return Err(format!("invalid colour {other}").into()),
            };
            Ok(acc)
        })
    }
}

#[derive(Debug)]
struct Rounds<'a> {
    rest: &'a str,
}

impl<'a> Iterator for Rounds<'a> {
    type Item = Result<Round, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.len() == 0 {
            return None;
        }
        let round = if let Some((curr, rest)) = self.rest.split_once(";") {
            self.rest = rest;
            curr
        } else {
            std::mem::replace(&mut self.rest, "")
        };
        Some(Round::parse(round))
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: u64,
    rounds_data: &'a str,
}

impl<'a> Game<'a> {
    fn parse(input: &'a str) -> Self {
        let (id_data, rounds_data) = input.split_once(":").unwrap();
        let (_, id_data) = id_data.split_once("Game").unwrap();
        Self {
            id: id_data.trim().parse().unwrap(),
            rounds_data,
        }
    }

    fn rounds(&self) -> Rounds {
        Rounds {
            rest: self.rounds_data,
        }
    }
}

fn are_rounds_valid<T>(rule: T, rounds: Rounds) -> Result<bool, Box<dyn std::error::Error>>
where
    T: Fn(Round) -> bool,
{
    rounds.fold(Ok(true), |acc, round| {
        let round = round?;
        if !rule(round) {
            Ok(false)
        } else {
            acc
        }
    })
}

pub mod part1 {
    use super::*;

    pub fn solve(
        red: u64,
        green: u64,
        blue: u64,
        input: &str,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let counts = Round {
            r: red,
            g: green,
            b: blue,
        };
        let rule = move |round: Round| match round {
            _ if counts.r < round.r => false,
            _ if counts.g < round.g => false,
            _ if counts.b < round.b => false,
            _ => true,
        };
        input.lines().fold(Ok(0), |id_sum, game| {
            let id_sum = id_sum?;
            let game = Game::parse(game);
            if are_rounds_valid(rule, game.rounds())? {
                Ok(id_sum + game.id)
            } else {
                Ok(id_sum)
            }
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_1: &str = "\
                                 Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                                 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                                 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                                 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                                 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
                                 ";

        #[test]
        fn parse() {
            let mut games = EXAMPLE_1.lines();
            let game = Game::parse(games.next().unwrap());
            assert_eq!(game.id, 1);
            assert_eq!(
                game.rounds()
                    .map(|round| round.unwrap())
                    .collect::<Vec<_>>(),
                vec![
                    Round { r: 4, g: 0, b: 3 },
                    Round { r: 1, g: 2, b: 6 },
                    Round { r: 0, g: 2, b: 0 },
                ]
            );
            let game = Game::parse(games.next().unwrap());
            assert_eq!(game.id, 2);
            assert_eq!(
                game.rounds()
                    .map(|round| round.unwrap())
                    .collect::<Vec<_>>(),
                vec![
                    Round { r: 0, g: 2, b: 1 },
                    Round { r: 1, g: 3, b: 4 },
                    Round { r: 0, g: 1, b: 1 },
                ]
            );
        }

        #[test]
        fn solve_example_1() {
            assert_eq!(part1::solve(12, 13, 14, EXAMPLE_1).unwrap(), 8);
        }
    }
}
