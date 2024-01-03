use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct NetworkNode {
    left: String,
    right: String,
}

impl NetworkNode {
    fn left(&self) -> &str {
        &self.left
    }

    fn right(&self) -> &str {
        &self.right
    }
}

fn parse_input(input: &str) -> (String, HashMap<String, NetworkNode>) {
    let mut network_map: HashMap<String, NetworkNode> = HashMap::new();
    let mut lines = input.lines();
    let directions = lines.next().unwrap();
    lines.next();
    for line in lines {
        let parts = line.split('=').collect::<Vec<&str>>();
        let node_name = parts[0].trim();
        let left_and_right = parts[1].trim().split(',').collect::<Vec<&str>>();
        let left = left_and_right[0].trim().replace('(', "");
        let right = left_and_right[1].trim().replace(')', "");
        network_map
            .entry(node_name.to_string())
            .or_insert(NetworkNode { left, right });
    }
    (directions.to_string(), network_map)
}

fn get_steps_to_zzz(direcitions: String, network_map: HashMap<String, NetworkNode>) -> u32 {
    let mut steps = 0;
    let mut current: &str = "AAA";
    for c in direcitions.chars().cycle() {
        steps += 1;
        let new_node = match c {
            'L' => network_map.get(current).unwrap().left(),
            'R' => network_map.get(current).unwrap().right(),
            _ => {
                panic!("Unknown direction: {}", c);
            }
        };
        if new_node.ends_with('Z') {
            break;
        }
        current = new_node;
    }
    steps
}

fn get_steps_to_z(
    directions: &str,
    network_map: &HashMap<String, NetworkNode>,
    starting_point: String,
) -> u64 {
    let mut steps = 0;
    let mut current: &str = &starting_point;
    for c in directions.chars().cycle() {
        steps += 1;
        let new_node = match c {
            'L' => network_map.get(current).unwrap().left(),
            'R' => network_map.get(current).unwrap().right(),
            _ => {
                panic!("Unknown direction: {}", c);
            }
        };
        if new_node.ends_with('Z') {
            break;
        }
        current = new_node;
    }
    steps
}

fn get_starting_spots(network_map: &HashMap<String, NetworkNode>) -> Vec<String> {
    network_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect()
}

fn get_steps_to_zzz_for_all_starting_spots(
    directions: String,
    network_map: HashMap<String, NetworkNode>,
) -> u64 {
    let spots = get_starting_spots(&network_map);
    let mut steps: Vec<u64> = Vec::new();

    for spot in spots {
        steps.push(get_steps_to_z(&directions, &network_map, spot))
    }

    steps.iter().fold(1, |acc, &b| lcm(acc, b))
}

fn main() {
    let input = include_str!("../input.txt");
    let (directions, network_map) = parse_input(input);
    println!(
        "Steps: {}",
        get_steps_to_zzz_for_all_starting_spots(directions, network_map)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2steps() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (directions, network_map) = parse_input(input);

        assert_eq!(get_steps_to_zzz(directions, network_map), 2);
    }

    #[test]
    fn test_6steps() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (directions, network_map) = parse_input(input);

        assert_eq!(get_steps_to_zzz(directions, network_map), 6);
    }

    #[test]
    fn test_parallel_in_6_steps() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (directions, network_map) = parse_input(input);

        assert_eq!(
            get_steps_to_zzz_for_all_starting_spots(directions, network_map),
            6
        );
    }
}
