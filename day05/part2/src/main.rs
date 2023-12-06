use regex::Regex;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
    shift_by: i64,
}

// #[derive(Debug)]
// struct Map {
//     ranges: Vec<Range>,
// }

impl Range {
    fn has(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }

    // fn get_to(&self, from: u64) -> u64 {
    //     if from >= self.start && from <= self.end {
    //         let diff = from - self.start;
    //         return self.shift_by + diff;
    //     }
    //     from
    // }
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = "seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    //
    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4
    //
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    //
    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13
    //
    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69
    //
    // humidity-to-location map:
    // 60 56 37
    // 56 93 4";

    let mut lines = input.lines();
    let seeds_regex = Regex::new(r"\d+\s\d+").unwrap();
    let seeds_and_ranges = seeds_regex.captures_iter(lines.next().unwrap_or(""));
    let seeds = seeds_and_ranges.fold(Vec::new(), |mut acc, c| {
        let parts: Vec<&str> = c[0].split(' ').collect();
        let start = parts[0].parse::<u64>().unwrap_or(0);
        let increment = parts[1].parse::<u64>().unwrap_or(0);
        for i in start..start + increment {
            acc.push(i);
        }
        acc
    });

    println!("{}", seeds.len());
    // println!("{:?}", seeds);

    // let mut seed_to_soil_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    // let mut soil_to_fertilizer_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    // let mut fertilizer_to_water_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    //
    // let mut water_to_light_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    // let mut light_to_temperature_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    // let mut temperature_to_humidity_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    // let mut humidity_to_location_map: Map = Map {
    //     name: "".to_string(),
    //     ranges: Vec::new(),
    // };
    //
    // let mut all_maps = vec![
    //     &mut seed_to_soil_map,
    //     &mut soil_to_fertilizer_map,
    //     &mut fertilizer_to_water_map,
    //     &mut water_to_light_map,
    //     &mut light_to_temperature_map,
    //     &mut temperature_to_humidity_map,
    //     &mut humidity_to_location_map,
    // ];
    //
    let mut all_maps: Vec<Vec<Range>> = vec![Vec::new(); 7];

    let number_regex = Regex::new(r"\d+").unwrap();
    lines.next(); // We know there's an empty line after the seeds

    let mut current_map: Vec<Range> = Vec::new();
    for line in lines {
        if line.is_empty() {
            all_maps.push(current_map);
            current_map = Vec::new();
            continue;
        }

        if line.contains("map") {
            // current_map.name = line.to_string();
            continue;
        }

        let numbers = number_regex
            .captures_iter(line)
            .map(|c| c[0].parse::<u64>().unwrap_or(0))
            .collect::<Vec<u64>>();

        let target_num = numbers[0];
        let source_num = numbers[1];
        let til = numbers[2];

        let range = Range {
            start: source_num,
            end: source_num + til,
            shift_by: (source_num as i64) - (target_num as i64),
        };
        current_map.push(range);
    }

    // println!("Seeds");
    // println!("{:?}", seeds);

    // println!("Maps");
    // for map in all_maps {
    //     println!("{:?}", map);
    // }

    let mut lowest_destination: Option<u64> = None;
    for seed in seeds {
        let mut location = seed;
        // println!("Starting at {}", location);
        for map in &mut all_maps {
            // println!("Map: {:?}", map);
            'range: for range in map {
                if range.has(location) {
                    // location = range.get_to(location);
                    location = ((location as i64) - range.shift_by) as u64;

                    break 'range;
                }
            }
        }

        if lowest_destination.is_none() || location < lowest_destination.unwrap() {
            lowest_destination = Some(location);
        }

        // println!("Ended in: {}", location);
    }

    println!("Lowest destination: {}", lowest_destination.unwrap_or(666));
}
