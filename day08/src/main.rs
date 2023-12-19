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
        if new_node == "ZZZ" {
            break;
        }
        current = new_node;
    }
    steps
}

fn main() {
    let input = include_str!("../input.txt");
    let (directions, network_map) = parse_input(input);
    println!("Steps: {}", get_steps_to_zzz(directions, network_map));
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
}
