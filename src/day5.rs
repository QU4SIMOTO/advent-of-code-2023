// TODO state machine on seed
mod almanac {
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct Almanac {
        pub seeds: Vec<u64>,
        lookup: HashMap<Key, Map>,
    }

    impl Almanac {
        pub fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let mut output = Almanac::default();
            for line in input.split("\n\n").map(|line| line.trim()) {
                if line.starts_with("seeds") {
                    let (_, seeds) = line.split_once(":").ok_or("no seed values")?;
                    for seed in seeds.split_whitespace().map(|seed| seed.parse::<u64>()) {
                        output.seeds.push(seed?);
                    }
                    continue;
                }
                let (map_kind, map_lines) = line.split_once("\n").ok_or("map missing data")?;
                match map_kind {
                    kind if kind.starts_with("seed-to-soil map") => {
                        output.insert_map(Property::Seed, Property::Soil, Map::parse(map_lines)?)
                    }
                    kind if kind.starts_with("soil-to-fertilizer map") => output.insert_map(
                        Property::Soil,
                        Property::Fertilizer,
                        Map::parse(map_lines)?,
                    ),
                    kind if kind.starts_with("fertilizer-to-water map") => output.insert_map(
                        Property::Fertilizer,
                        Property::Water,
                        Map::parse(map_lines)?,
                    ),
                    kind if kind.starts_with("water-to-light map") => {
                        output.insert_map(Property::Water, Property::Light, Map::parse(map_lines)?)
                    }
                    kind if kind.starts_with("light-to-temperature map") => output.insert_map(
                        Property::Light,
                        Property::Temperature,
                        Map::parse(map_lines)?,
                    ),
                    kind if kind.starts_with("temperature-to-humidity map") => output.insert_map(
                        Property::Temperature,
                        Property::Humidity,
                        Map::parse(map_lines)?,
                    ),
                    kind if kind.starts_with("humidity-to-location map") => output.insert_map(
                        Property::Humidity,
                        Property::Location,
                        Map::parse(map_lines)?,
                    ),
                    _ => {}
                }
            }
            Ok(output)
        }

        pub fn perform_map(&self, src: Property, dst: Property, value: u64) -> u64 {
            if let Some(map) = self.lookup.get(&Key(src, dst)) {
                map.map_value(value)
            } else {
                value
            }
        }

        fn insert_map(&mut self, src: Property, dst: Property, value: Map) {
            self.lookup.insert(Key(src, dst), value);
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub enum Property {
        Seed,
        Soil,
        Fertilizer,
        Water,
        Light,
        Temperature,
        Humidity,
        Location,
    }

    impl Iterator for Property {
        type Item = Self;

        fn next(&mut self) -> Option<Self> {
            match self {
                Self::Seed => Some(Self::Soil),
                Self::Soil => Some(Self::Fertilizer),
                Self::Fertilizer => Some(Self::Water),
                Self::Water => Some(Self::Light),
                Self::Light => Some(Self::Temperature),
                Self::Temperature => Some(Self::Humidity),
                Self::Humidity => Some(Self::Location),
                Self::Location => None,
            }
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Key(Property, Property);

    #[derive(Debug)]
    struct Map(Vec<MapLine>);

    impl Map {
        fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let map_lines = input
                .split("\n")
                .map(|line| MapLine::parse(line))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self(map_lines))
        }

        fn map_value(&self, value: u64) -> u64 {
            self.0
                .iter()
                .find_map(|line| {
                    if value >= line.src_start && value <= line.src_start + line.range_len {
                        Some((line.dst_start + value) - line.src_start)
                    } else {
                        None
                    }
                })
                .unwrap_or(value)
        }
    }

    #[derive(Debug)]
    struct MapLine {
        src_start: u64,
        dst_start: u64,
        range_len: u64,
    }

    impl MapLine {
        fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let mut iter = input.split_whitespace().map(|n| n.parse::<u64>());

            let dst_start = iter.next().ok_or("no destination start in map line")??;
            let src_start = iter.next().ok_or("no source start in map line")??;
            let range_len = iter.next().ok_or("no range length in map line")??;
            Ok(MapLine {
                dst_start,
                src_start,
                range_len,
            })
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let almanac = almanac::Almanac::parse(input)?;
        let min_location = almanac
            .seeds
            .iter()
            .map(|value| {
                let mut property = almanac::Property::Seed;
                let mut value = *value;
                loop {
                    if let Some(next_property) = property.next() {
                        value = almanac.perform_map(property, next_property, value);
                        property = next_property;
                    } else {
                        break;
                    }
                }
                value
            })
            .min();
        Ok(min_location.unwrap())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const EXAMPLE_1: &str = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4";

        #[test]
        fn solve_example_1() {
            assert_eq!(solve(EXAMPLE_1).unwrap(), 35);
        }
    }
}
