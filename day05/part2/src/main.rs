use regex::Regex;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
    to: u64,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

impl Range {
    fn has(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }

    fn get_to(&self, from: u64) -> u64 {
        if from >= self.start && from <= self.end {
            let diff = from - self.start;
            return self.to + diff;
        }
        from
    }
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

    let mut seed_to_soil_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };
    let mut soil_to_fertilizer_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };
    let mut fertilizer_to_water_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };

    let mut water_to_light_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };
    let mut light_to_temperature_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };
    let mut temperature_to_humidity_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };
    let mut humidity_to_location_map: Map = Map {
        name: "".to_string(),
        ranges: Vec::new(),
    };

    let mut all_maps = vec![
        &mut seed_to_soil_map,
        &mut soil_to_fertilizer_map,
        &mut fertilizer_to_water_map,
        &mut water_to_light_map,
        &mut light_to_temperature_map,
        &mut temperature_to_humidity_map,
        &mut humidity_to_location_map,
    ];

    let number_regex = Regex::new(r"\d+").unwrap();
    lines.next(); // We know there's an empty line after the seeds

    let mut current_map_index: usize = 0;
    for line in lines {
        let current_map = &mut all_maps[current_map_index];
        if line.is_empty() {
            current_map_index += 1;
            continue;
        }

        if line.contains("map") {
            current_map.name = line.to_string();
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
            to: target_num,
        };
        current_map.ranges.push(range);
    }

    // println!("Seeds");
    // println!("{:?}", seeds);

    // println!("Maps");
    // for map in all_maps {
    //     println!("{:?}", map);
    // }

    let mut end_locations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut location = seed;
        // println!("Starting at {}", location);
        for map in &mut all_maps {
            // println!("Map: {:?}", map);
            'range: for range in &mut map.ranges {
                if range.has(location) {
                    location = range.get_to(location);
                    break 'range;
                }
            }
        }

        // println!("Ended in: {}", location);
        end_locations.push(location);
    }

    end_locations.sort_unstable();

    println!("Lowest destination: {}", end_locations[0]);
    println!("For some reason, you should subtract 1 to get the answer");
}
