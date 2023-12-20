use std::collections::HashMap;

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = include_str!("example_input");
#[allow(dead_code)]
const INPUT: &str = include_str!("input");

fn main() {
    part1(INPUT);
}

fn part1(input: &str) {
    let (directions, network) = input.split_once("\n\n").unwrap();

    let directions: Vec<_> = parse_directions(directions).collect();
    let network = parse_network(network);

    let mut counter = 0;
    let mut directions_iter = directions.iter().copied().cycle();
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let direction = directions_iter.next().expect("directions_iter is infinite");
        current_node = network.next_node(current_node, direction);
        counter += 1;
    }

    println!("part 1: {}", counter);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_directions(directions: &str) -> impl Iterator<Item = Direction> + '_ {
    directions.bytes().map(|dir| match dir {
        b'L' => Direction::Left,
        b'R' => Direction::Right,
        _ => panic!("unknown direction"),
    })
}

#[derive(Debug)]
struct Network<'a> {
    network: HashMap<&'a str, NetworkNode<'a>>,
}

impl<'a> Network<'a> {
    fn next_node(&self, curr_node: &str, dir: Direction) -> &'a str {
        match dir {
            Direction::Left => self.network[curr_node].left,
            Direction::Right => self.network[curr_node].right,
        }
    }
}

#[derive(Debug)]
struct NetworkNode<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_network(network: &str) -> Network {
    Network {
        network: network
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(" = ").unwrap();
                let (left, right) = to[1..(to.len() - 1)].split_once(", ").unwrap();

                (from, NetworkNode { left, right })
            })
            .collect(),
    }
}
