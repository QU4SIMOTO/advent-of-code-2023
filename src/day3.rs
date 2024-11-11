#[derive(Debug)]
struct Node(i64, i64);

impl Node {
    fn is_neighbour(&self, other: &Self) -> bool {
        (self.0 == other.0 || other.0 == self.0 + 1 || other.0 == self.0 - 1)
            && (self.1 == other.1 || other.1 == self.1 + 1 || other.1 == (self.1 - 1))
    }
}

#[derive(Debug)]
struct Symbol {
    node: Node,
}

#[derive(Debug)]
struct Part {
    nodes: Vec<Node>,
    value: u64,
}

#[derive(Debug, Default)]
struct SchematicGraph {
    symbols: Vec<Symbol>,
    parts: Vec<Part>,
}

impl SchematicGraph {
    fn parse(input: &str) -> Self {
        let mut graph = SchematicGraph::default();
        for (i, line) in input.lines().enumerate() {
            let mut iter = line.chars().enumerate().peekable();
            while let Some((j, c)) = iter.next() {
                match c {
                    '.' => continue,
                    c if c.is_ascii_digit() => {
                        let mut end_index = j;
                        loop {
                            if let Some(_) = iter.next_if(|(_, c_next)| c_next.is_ascii_digit()) {
                                end_index += 1;
                            } else {
                                break;
                            }
                        }
                        let value = *&line[j..=end_index].parse::<u64>().expect("is ascii digit");
                        let nodes = (j..=end_index).map(|j| Node(i as i64, j as i64)).collect();
                        graph.parts.push(Part { nodes, value });
                    }
                    _ => graph.symbols.push(Symbol {
                        node: Node(i as i64, j as i64),
                    }),
                }
            }
        }
        graph
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(input: &str) -> u64 {
        let schematic = SchematicGraph::parse(input);
        schematic
            .parts
            .iter()
            .filter_map(|part| {
                part.nodes.iter().find(|part_node| {
                    schematic
                        .symbols
                        .iter()
                        .find(|symbol| part_node.is_neighbour(&symbol.node))
                        .is_some()
                })?;
                Some(part.value)
            })
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_1: &str = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";

        #[test]
        fn solve_example_1() {
            assert_eq!(solve(EXAMPLE_1), 4361);
        }
    }
}
