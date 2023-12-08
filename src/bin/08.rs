use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct NodeIdentifier(char, char, char);

impl NodeIdentifier {
    pub fn start() -> Self {
        Self('A', 'A', 'A')
    }

    pub fn is_end(&self) -> bool {
        self.0 == 'Z' && self.1 == 'Z' && self.2 == 'Z'
    }

    pub fn is_ghost_start(&self) -> bool {
        self.2 == 'A'
    }

    pub fn is_ghost_end(&self) -> bool {
        self.2 == 'Z'
    }
}

impl TryFrom<&str> for NodeIdentifier {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();

        Ok(Self(
            chars.next().ok_or(())?,
            chars.next().ok_or(())?,
            chars.next().ok_or(())?,
        ))
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

fn parse(
    input: &str,
) -> (
    Vec<Direction>,
    HashMap<NodeIdentifier, (NodeIdentifier, NodeIdentifier)>,
) {
    let (directions, nodes) = input.split_once("\n\n").expect("break in input");

    (
        directions
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("valid direction"),
        nodes
            .lines()
            .map(|line| line.split_once(" = ").expect("valid formed node"))
            .map(|(tag, connections)| {
                (
                    NodeIdentifier::try_from(tag).expect("valid node identifier"),
                    {
                        let (left, right) = connections[1..connections.len() - 1]
                            .split_once(", ")
                            .expect("node pairs seperated by comma");

                        (
                            NodeIdentifier::try_from(left).expect("left identifier valid"),
                            NodeIdentifier::try_from(right).expect("right identifier valid"),
                        )
                    },
                )
            })
            .collect::<HashMap<NodeIdentifier, (NodeIdentifier, NodeIdentifier)>>(),
    )
}

fn run(
    start: NodeIdentifier,
    end_test: fn(&NodeIdentifier) -> bool,
    directions: &[Direction],
    nodes: &HashMap<NodeIdentifier, (NodeIdentifier, NodeIdentifier)>,
) -> u32 {
    let mut position = start;
    let mut step_count = 0;
    let mut directions = directions.into_iter().cycle();

    while !end_test(&position) {
        let node = nodes.get(&position).expect("node present in nodes list");

        position = match directions.next().expect("next direction") {
            Direction::Left => node.0,
            Direction::Right => node.1,
        };

        step_count += 1;
    }

    step_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);

    Some(run(
        NodeIdentifier::start(),
        NodeIdentifier::is_end,
        &directions,
        &nodes,
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse(input);

    Some(
        nodes
            .keys()
            .cloned()
            .filter(NodeIdentifier::is_ghost_start)
            .map(|position| run(position, NodeIdentifier::is_ghost_end, &directions, &nodes) as u64)
            .fold(1u64, num::integer::lcm),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
