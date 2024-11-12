#[derive(Debug)]
struct Card {
    id: u64,
    winning_numbers: Vec<u64>,
    player_numbers: Vec<u64>,
}

impl<'a> Card {
    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (id, rest) = input.split_once(":").ok_or("No numbers")?;
        let id = id
            .split_whitespace()
            .nth(1)
            .ok_or("No card number")?
            .parse()?;

        let (winning_numbers, player_numbers) =
            rest.split_once("|").ok_or("No separator for numbers")?;
        let winning_numbers = Self::parse_numbers(winning_numbers, 5)?;
        let player_numbers = Self::parse_numbers(player_numbers, 8)?;
        Ok(Self {
            id,
            winning_numbers,
            player_numbers,
        })
    }

    fn parse_numbers(input: &str, length: usize) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        let output = input.trim().split_whitespace().fold(
            Ok(Vec::<u64>::with_capacity(length)),
            |acc: Result<Vec<u64>, Box<dyn std::error::Error>>, n| {
                let mut acc = acc?;
                acc.push(n.parse()?);
                Ok(acc)
            },
        )?;
        Ok(output)
    }

    fn get_matching_numbers(&'a self) -> impl Iterator<Item = &u64> + 'a {
        self.player_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashMap;

    pub fn solve(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let mut card_inventory = HashMap::<u64, u64>::new();
        for line in input.lines() {
            let card = Card::parse(line)?;
            let current_card_count = *card_inventory
                .entry(card.id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            let total_matches = card.get_matching_numbers().count();
            if total_matches > 0 {
                for winner in card.id + 1..=card.id + total_matches as u64 {
                    card_inventory
                        .entry(winner)
                        .and_modify(|count| *count += current_card_count)
                        .or_insert(current_card_count);
                }
            }
        }
        Ok(card_inventory.values().sum())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_1: &str = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        #[test]
        fn solve_example_1() {
            assert_eq!(solve(EXAMPLE_1).unwrap(), 30);
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(input
            .lines()
            .map(|card| {
                let card = Card::parse(card)?;
                Ok(card
                    .get_matching_numbers()
                    .fold(0, |acc, _| if acc == 0 { 1 } else { 2 * acc }))
            })
            .fold(
                Ok(0),
                |acc: Result<u64, Box<dyn std::error::Error>>,
                 curr: Result<u64, Box<dyn std::error::Error>>| {
                    Ok(acc? + curr?)
                },
            )?)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_1: &str = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        #[test]
        fn solve_example_1() {
            assert_eq!(part1::solve(EXAMPLE_1).unwrap(), 13);
        }
    }
}
