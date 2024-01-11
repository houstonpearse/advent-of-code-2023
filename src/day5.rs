use std::{fs::read_to_string, ops::Range};

pub fn solution1() -> i64 {
    let input = read_to_string("./inputs/input5.txt").unwrap();
    let split: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<i64> = split[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let seed_to_soil = split[1].strip_prefix("seed-to-soil map:\n").unwrap();
    let soil_to_fertilizer = split[2].strip_prefix("soil-to-fertilizer map:\n").unwrap();
    let fertilizer_to_water = split[3].strip_prefix("fertilizer-to-water map:\n").unwrap();
    let water_to_light = split[4].strip_prefix("water-to-light map:\n").unwrap();
    let light_to_temperature = split[5].strip_prefix("light-to-temperature map:\n").unwrap();
    let temperature_to_humidity = split[6].strip_prefix("temperature-to-humidity map:\n").unwrap();
    let humidity_to_location = split[7].strip_prefix("humidity-to-location map:\n").unwrap().strip_suffix("\n").unwrap();

    let chain: MapChain = MapChain::new()
        .add_chain(MyMap::new(seed_to_soil))
        .add_chain(MyMap::new(soil_to_fertilizer))
        .add_chain(MyMap::new(fertilizer_to_water))
        .add_chain(MyMap::new(water_to_light))
        .add_chain(MyMap::new(light_to_temperature))
        .add_chain(MyMap::new(temperature_to_humidity))
        .add_chain(MyMap::new(humidity_to_location));
        
    return seeds.iter().map(|seed| chain.chain_maps(seed)).min().unwrap();
}
#[allow(dead_code)]
pub fn solution2() -> i64 {
    let input = read_to_string("./inputs/input5.txt").unwrap();
    let split: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<i64> = split[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let seed_to_soil = split[1].strip_prefix("seed-to-soil map:\n").unwrap();
    let soil_to_fertilizer = split[2].strip_prefix("soil-to-fertilizer map:\n").unwrap();
    let fertilizer_to_water = split[3].strip_prefix("fertilizer-to-water map:\n").unwrap();
    let water_to_light = split[4].strip_prefix("water-to-light map:\n").unwrap();
    let light_to_temperature = split[5].strip_prefix("light-to-temperature map:\n").unwrap();
    let temperature_to_humidity = split[6].strip_prefix("temperature-to-humidity map:\n").unwrap();
    let humidity_to_location = split[7].strip_prefix("humidity-to-location map:\n").unwrap().strip_suffix("\n").unwrap();

    let chain: MapChain = MapChain::new()
        .add_chain(MyMap::new(seed_to_soil))
        .add_chain(MyMap::new(soil_to_fertilizer))
        .add_chain(MyMap::new(fertilizer_to_water))
        .add_chain(MyMap::new(water_to_light))
        .add_chain(MyMap::new(light_to_temperature))
        .add_chain(MyMap::new(temperature_to_humidity))
        .add_chain(MyMap::new(humidity_to_location));
    let location_map = chain.chain.get(chain.chain.len()-1).unwrap();
    
    for i in location_map.map_range() {
        let seed_val = chain.chain_maps_rev(&i);
        let mut j = 0;
        while j < seeds.len() {
            if seed_val >= seeds[j] && seed_val < (seeds[j] + seeds[j+1]) {
                return i;
            }
            j+=2;
        }
    }
    return 0;
}
struct MapChain {
    chain: Vec<MyMap>,
}

impl MapChain {
    fn new() -> MapChain {
        return MapChain {
            chain: Vec::new()
        }
    }
    fn add_chain(mut self, map: MyMap) -> MapChain {
        self.chain.push(map);
        return self
    }

    fn chain_maps(&self, value: &i64) -> i64 {
        let mut val: i64 = *value;
        for map in self.chain.iter() {
            val = map.map(val);
        }
        return val
    }
    fn chain_maps_rev(&self, value: &i64) -> i64 {
        let mut val: i64 = *value;
        for map in self.chain.iter().rev() {
            val = map.map_rev(val);
        }
        return val
    }
}
struct MyMap {
    items: Vec<Item>,
}

impl MyMap {
    fn new(map_str_list: &str) -> MyMap {
        MyMap {
            items: map_str_list.split("\n").map(|x| Item::new(x)).collect(),
        }
    }
    fn map(&self, value: i64) -> i64{
        for item in self.items.iter() {
            if item.in_range(value) {
                return item.forward(value);
            }
        }
        return value
    }
    fn map_rev(&self,value: i64) ->i64 {
        for item in self.items.iter() {
            if item.rev_in_range(value) {
                return item.backward(value);
            }
        }
        return value
    }
    fn map_range(&self) -> Range<i64> {
        let mut min_start_to=self.items.get(0).unwrap().start_to;
        let mut max_end_to = self.items.get(0).unwrap().end_to;
        for item in self.items.iter() {
            if item.start_to < min_start_to {
                min_start_to = item.start_to
            } 
            if item.end_to > max_end_to {
                max_end_to = item.end_to
            }
        }
        return min_start_to..(max_end_to+1);
    }
}

#[derive(Debug)]
struct Item {
    start_to: i64,
    start_from: i64,
    end_to: i64,
    end_from: i64,
}

impl Item {
    fn new(map_str: &str) -> Item {
        let nums: Vec<i64> = map_str
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        Item {
            start_to: nums[0],
            start_from: nums[1],
            end_to: nums[0] + nums[2] - 1,
            end_from: nums[1] + nums[2] - 1,
        }
    }

    fn forward(&self, value: i64) -> i64 {
        value - self.start_from + self.start_to
    }
    fn backward(&self,value: i64) -> i64 {
        value - self.start_to + self.start_from
    }
    fn in_range(&self, value: i64) -> bool {
        return (value >= self.start_from) && (value <= self.end_from);
    }
    fn rev_in_range(&self, value:i64) -> bool {
        return (value >= self.start_to) && (value <= self.end_to);
    }
}

